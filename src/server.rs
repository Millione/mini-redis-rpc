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

    async fn del(&self, key: FastStr) -> Result<bool, AnyhowError> {
        Ok(self.db.del(&key).is_some())
    }

    async fn ping(&self) -> Result<(), AnyhowError> {
        Ok(())
    }
}
