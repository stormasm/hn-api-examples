/// Checks for null in the hackernews feed
/// If a null is found, do not parse and keep going...
use hn_api::types::Story;
use hn_api::HnClient;

fn print(api: &HnClient, items: &[u32]) {
    for id in items {
        println!("\nid = {}\n", id);

        let item = api.get_json(*id).unwrap();

        match item.as_ref() {
            "null" => println!("null for id = {}", id),
            _ => {
                println!("{:?}", item);
                let story: Story = serde_json::from_str(&item).unwrap();
                println!("\n{:?} {:?}", story.by.unwrap(), story.title.unwrap());
            }
        }
    }
}

fn main() {
    let api = HnClient::init().unwrap();
    let top = [
        21669726, 21664126, 21672696, 21655779, 21672563, 21655225, 21669234,
    ];
    let count = 7;
    print(&api, &top[..count]);
}
