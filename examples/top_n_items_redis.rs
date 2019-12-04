use hn_api::types::Item;
use hn_api::HnClient;
use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::Commands;

fn write_json_to_redis(key: String, value: String) -> redis::RedisResult<()> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();

    let _: () = con.set("hn-story-start", &key)?;

    let _x0 = redis::cmd("HSET")
        .arg("hn-story-19")
        .arg(key)
        .arg(value)
        .query::<u64>(&mut *con)
        .unwrap();

    Ok(())
}

fn process_items(api: &HnClient, item_ids: Vec<u32>) {
    for item_id in item_ids {
        let item_json = api.get_json(item_id).unwrap();
        match item_json.as_ref() {
            "null" => println!("{} null", item_id),
            _ => {
                let item: Item = serde_json::from_str(&item_json).unwrap();
                let item_type = item.item_type();
                match item_type.as_ref() {
                    "story" => {
                        println!("{} story", item_id);
                        let _ = write_json_to_redis(item_id.to_string(), item_json);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn top_n_items(numofitems: u32, max_number: u32) -> Vec<u32> {
    let mut items = Vec::new();
    items.push(max_number);
    let mut value = max_number;

    for _ in 1..numofitems {
        value = value - 1;
        items.push(value);
    }
    items
}

fn main() {
    let api = HnClient::init().unwrap();

    let max_item_id = api.get_max_item_id().unwrap();
    // Instead of the max_item_id start with this item_id
    // let max_item_id = 21625360;
    println!("max item id = {}", max_item_id);

    let item_ids = top_n_items(10000, max_item_id);
    process_items(&api, item_ids);
}
