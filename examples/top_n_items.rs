use hn_api::types::Item;
use hn_api::HnClient;

fn process_items(api: &HnClient, item_ids: Vec<u32>) {
    for item_id in item_ids {
        // println!("\nid = {}\n", item_id);

        let item_json = api.get_json(item_id).unwrap();

        match item_json.as_ref() {
            "null" => println!("{} null", item_id),
            _ => {
                // println!("{:?}", item_id);
                let item: Item = serde_json::from_str(&item_json).unwrap();
                println!("{} {}", item_id, item.item_type());
                // println!("\n{:?} {:?}", story.by.unwrap(), story.title.unwrap());
            }
        }
    }
}

#[allow(dead_code)]
fn process_items_old(api: &HnClient, item_ids: Vec<u32>) {
    for item_id in &item_ids {
        let item = api.get_item(*item_id).unwrap().unwrap();
        println!("{} {}", item_id, item.item_type());
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
