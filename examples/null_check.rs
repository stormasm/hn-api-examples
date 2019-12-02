/// Checks for null in the hackernews feed
/// If a null is found, do not parse and keep going...
use hn_api::types::Item;
use hn_api::HnClient;

fn nullcheck(api: &HnClient, item_ids: Vec<u32>) {
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

fn main() {
    let api = HnClient::init().unwrap();
    let top = vec![
        21669726, 21664126, 21672696, 21655779, 21672563, 21655225, 21669234,
    ];
    nullcheck(&api, top);
}
