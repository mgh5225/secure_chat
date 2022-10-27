use std::collections::HashMap;

use libs::{
    packet::{DataPacket, Packet, PacketType},
    packet_manager, BaseModels, PacketModels,
};
use redis::Commands;
pub struct Database {
    db: redis::Client,
}

impl Database {
    pub fn new() -> Result<Self, redis::RedisError> {
        let db = redis::Client::open("redis://127.0.0.1/")?;

        Ok(Self { db })
    }

    pub fn create_user(&self, user: BaseModels::User) -> Result<(), redis::RedisError> {
        let mut conn = self.db.get_connection()?;

        let user = user.get_hash();

        conn.hset_multiple(user.0, &user.1)?;

        Ok(())
    }

    pub fn get_user(&self, key: String) -> Result<BaseModels::User, redis::RedisError> {
        let mut conn = self.db.get_connection()?;
        let hash: HashMap<u8, String> = conn.hgetall(key)?;

        if hash.is_empty() {
            return Err(redis::RedisError::from((
                redis::ErrorKind::IoError,
                "User Not Found",
            )));
        }

        let user = BaseModels::User::from_hash(hash);

        Ok(user)
    }
}
