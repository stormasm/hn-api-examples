use hn_api::types::Item;
use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::{Commands, RedisResult};
use std::io::Error;

fn del_hashmap_key(key: String, field: String) -> RedisResult<()> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();
    con.hdel(key, field)
}

fn get_hashmap_keys(key: String) -> RedisResult<Vec<u32>> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();
    con.hkeys(key)
}

fn main() -> Result<(), Error> {
    let no_title = "story-title-is-none";

    let mut keys = get_hashmap_keys("hn-story-20".to_string()).unwrap();
    keys.sort();

    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();

    for key in &keys {
        let value: RedisResult<String> = con.hget("hn-story-20".to_string(), key.to_string());
        let item_json = value.unwrap();
        let item: Item = serde_json::from_str(&item_json).unwrap();
        let story_title = &item.title().unwrap();

        if story_title == &no_title {
            let _x = del_hashmap_key("hn-story-20".to_string(), key.to_string());
        }
    }
    println!("Number of keys = {}", keys.len());
    Ok(())
}
