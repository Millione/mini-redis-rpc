use std::{
    collections::{BTreeSet, HashMap},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use pilota::FastStr;
use tokio::{sync::Notify, time::Instant};

#[derive(Clone, Debug)]
pub struct Db {
    shared: Arc<Shared>,
}

#[derive(Debug)]
struct Shared {
    state: Mutex<State>,
    background_task: Notify,
    shutdown: AtomicBool,
}

#[derive(Debug)]
struct State {
    entries: HashMap<FastStr, Entry>,
    expirations: BTreeSet<(Instant, FastStr)>,
}

#[derive(Debug)]
struct Entry {
    value: FastStr,
    expires_at: Option<Instant>,
}

impl Db {
    pub fn new() -> Self {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                entries: HashMap::new(),
                expirations: BTreeSet::new(),
            }),
            background_task: Notify::new(),
            shutdown: AtomicBool::new(false),
        });

        tokio::spawn(purge_expired_tasks(shared.clone()));

        Self { shared }
    }

    pub fn get(&self, key: &FastStr) -> Option<FastStr> {
        let state = self.shared.state.lock().unwrap();

        state.entries.get(key).map(|entry| entry.value.clone())
    }

    pub fn set(&self, key: FastStr, value: FastStr, expire: Option<Duration>) {
        let mut state = self.shared.state.lock().unwrap();

        let mut notify = false;

        let expires_at = expire.map(|duration| {
            let when = Instant::now() + duration;

            notify = state
                .next_expiration()
                .map(|expiration| expiration > when)
                .unwrap_or(true);

            state.expirations.insert((when, key.clone()));
            when
        });

        let prev = state
            .entries
            .insert(key.clone(), Entry { value, expires_at });

        if let Some(prev) = prev {
            if let Some(when) = prev.expires_at {
                state.expirations.remove(&(when, key));
            }
        }

        drop(state);

        if notify {
            self.shared.background_task.notify_one();
        }
    }

    pub fn del(&self, key: &FastStr) -> Option<FastStr> {
        let mut state = self.shared.state.lock().unwrap();

        let mut notify = false;

        let prev = state.entries.remove(key);

        if let Some(prev) = &prev {
            if let Some(when) = prev.expires_at {
                notify = state
                    .next_expiration()
                    .map(|expiration| expiration > when)
                    .unwrap_or(true);
                state.expirations.remove(&(when, key.clone()));
            }
        }

        drop(state);

        if notify {
            self.shared.background_task.notify_one();
        }

        prev.map(|entry| entry.value)
    }
}

impl Drop for Db {
    fn drop(&mut self) {
        self.shared.shutdown.store(true, Ordering::Relaxed);
        self.shared.background_task.notify_one();
    }
}

impl Shared {
    fn purge_expired_keys(&self) -> Option<Instant> {
        let mut state = self.state.lock().unwrap();

        if self.is_shutdown() {
            return None;
        }

        let state = &mut *state;

        let now = Instant::now();

        while let Some(&(when, ref key)) = state.expirations.iter().next() {
            if when > now {
                return Some(when);
            }
            state.entries.remove(key);
            state.expirations.remove(&(when, key.clone()));
        }

        None
    }

    fn is_shutdown(&self) -> bool {
        self.shutdown.load(Ordering::Relaxed)
    }
}

impl State {
    fn next_expiration(&self) -> Option<Instant> {
        self.expirations
            .iter()
            .next()
            .map(|expiration| expiration.0)
    }
}

async fn purge_expired_tasks(shared: Arc<Shared>) {
    while !shared.is_shutdown() {
        if let Some(when) = shared.purge_expired_keys() {
            tokio::select! {
                _ = tokio::time::sleep_until(when) => {}
                _ = shared.background_task.notified() => {}
            }
        } else {
            shared.background_task.notified().await;
        }
    }

    tracing::debug!("Purge background task shut down")
}
