use anyhow::Result;
use image::flat::NormalForm::ColumnMajorPacked;
use redis::{Commands, Connection, ToRedisArgs};

use crate::REDIS_CLI;

/// set string's value
pub fn set<T>(key: &str, value: T, expire_seconds: Option<usize>) -> Result<()>
    where
        T: ToRedisArgs,
{
    let mut conn = REDIS_CLI.get_connection()?;
    // set string
    redis::cmd("set").arg(key).arg(value).query(&mut conn)?;

    // set expire time
    if expire_seconds.is_some() {
        let _: usize = conn.expire(key, expire_seconds.unwrap_or_default())?;
    }

    Ok(())
}

/// get string's value
pub fn get(key: &str) -> Result<String> {
    let mut conn = REDIS_CLI.get_connection()?;
    let value: String = redis::cmd("get").arg(key).query(&mut conn)?;
    Ok(value)
}

/// del key
pub fn del(keys: &[&str]) -> Result<()> {
    let mut conn = REDIS_CLI.get_connection()?;
    let effected: i64 = redis::cmd("del").arg(keys).query(&mut conn)?;
    Ok(())
}
