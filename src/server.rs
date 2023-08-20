use std::time::Duration;

use pilota::FastStr;
use volo::async_trait;
use volo_thrift::AnyhowError;

use crate::{
    db::Db,
    gen::volo_gen::redis::{GetResp, RedisService, SetReq},
};

pub struct Server {
    db: Db,
}

impl Server {
    pub fn new() -> Self {
        Self { db: Db::new() }
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RedisService for Server {
    async fn get(&self, key: FastStr) -> Result<GetResp, AnyhowError> {
        Ok(GetResp {
            value: self.db.get(&key).clone(),
        })
    }

    async fn set(&self, req: SetReq) -> Result<(), AnyhowError> {
        self.db.set(
            req.key,
            req.value,
            req.expires.map(|ms| Duration::from_millis(ms as u64)),
        );
        Ok(())
    }

    async fn del(&self, keys: Vec<FastStr>) -> Result<i64, AnyhowError> {
        Ok(self.db.del(&keys))
    }

    async fn ping(&self) -> Result<(), AnyhowError> {
        Ok(())
    }

    async fn publish(&self, channel: FastStr, message: FastStr) -> Result<i64, AnyhowError> {
        Ok(self.db.publish(channel, message))
    }

    async fn subscribe(&self, channels: Vec<FastStr>) -> Result<Vec<FastStr>, AnyhowError> {
        let mut messages = Vec::with_capacity(channels.len());
        let rxs = self.db.subscribe(channels);
        for mut rx in rxs {
            messages.push(rx.recv().await.unwrap_or_default())
        }
        Ok(messages)
    }
}
