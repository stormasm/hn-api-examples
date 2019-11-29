use hn_api::HnClient;

fn process_items(api: &HnClient, item_ids: Vec<u32>) {
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
    println!("max item id = {}", max_item_id);

    let item_ids = top_n_items(500, max_item_id);
    process_items(&api, item_ids);
}
