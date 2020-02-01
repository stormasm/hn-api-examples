use hn_api::types::Item;
use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::{Commands, RedisResult};
use serde_json::json;
use std::fs::File;
use std::io::{Error, Write};

fn get_hashmap_keys(key: String) -> RedisResult<Vec<u32>> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();
    con.hkeys(key)
}

fn main() -> Result<(), Error> {
    let no_title = "story-title-is-none";
    let no_url = "story-url-is-none";

    let mut keys = get_hashmap_keys("hn-story-20".to_string()).unwrap();
    keys.sort();

    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();

    let path = "/tmp13/hackernews-story-archive/data/story-time.json";
    let mut output = File::create(path)?;

    for key in &keys {
        let value: RedisResult<String> = con.hget("hn-story-20".to_string(), key.to_string());
        let item_json = value.unwrap();
        let item: Item = serde_json::from_str(&item_json).unwrap();
        let story_title = &item.title().unwrap();
        let story_time = &item.time();
        let story_url = &item.url().unwrap();

        if (story_title != &no_title) && (story_url != &no_url) {
            let title_json = json!({
                "id": key,
                "title": story_title,
                "time": story_time,
                "url": story_url,
            });

            // Convert to a string of JSON and print it out
            // println!("{}", title_json.to_string());
            write!(output, "{}\n", title_json.to_string())?;
        }
    }
    output.sync_all()?;
    println!("Number of keys = {}", keys.len());
    Ok(())
}
