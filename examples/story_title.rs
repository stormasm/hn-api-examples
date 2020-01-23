use hn_api::types::Item;

use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::{Commands, RedisResult};

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
    let mut keys = get_hashmap_keys("hn-story-20".to_string()).unwrap();
    keys.sort();

    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();

    let path = "/tmp13/hackernews-story-archive/data/story-title.json";
    let mut output = File::create(path)?;

    for key in &keys {
        let value: RedisResult<String> = con.hget("hn-story-20".to_string(), key.to_string());
        let item_json = value.unwrap();

        let item: Item = serde_json::from_str(&item_json).unwrap();
        println!("{}, {:?}", key, item.title().unwrap());
    }
    output.sync_all()?;
    println!("Number of keys = {}", keys.len());
    Ok(())
}

/*
        match item_json.as_ref() {
            "null" => println!("\n{} null\n", key),
            _ => {
                let item: Item = serde_json::from_str(&item_json).unwrap();
                let item_type = item.item_type();
                match item_type.as_ref() {
                    "story" => {
                        // println!("{} story", key);
                        println!("{}, {:?}", key, item.title().unwrap());
                        // let _ = write_json_to_redis(item_id.to_string(), item_json);
                    }
                    _ => {}
                }
            }
        }
*/
//        println!("{:?}", item.title().unwrap());

/*
        println!("{} story", item_id);
        println!("{:?}", item.title().unwrap());

        let titlejson = json!({
            "id": key,
            "title": title,
        });

        // Convert to a string of JSON and print it out
        println!("{}", titlejson.to_string());
*/
//write!(output, "{}", item_json.to_string())?;
//write!(output, "{}", "\n")?;
