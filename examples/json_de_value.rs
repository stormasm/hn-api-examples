use hn_api::types::Story;
use hn_api::HnClient;

fn print(api: &HnClient, items: &[u32]) {
    for id in items {
        println!("\nid = {}\n", id);

        let item = api.get_json(*id).unwrap();
        println!("{:?}", item);

        let story: Story = serde_json::from_str(&item).unwrap();
        println!("\n{:?} {:?}", story.by.unwrap(), story.title.unwrap());
    }
}

fn main() {
    let api = HnClient::init().unwrap();
    let top = [21669726, 21672696];
    let count = 2;
    print(&api, &top[..count]);
}
