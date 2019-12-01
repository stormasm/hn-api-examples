use hn_api::HnClient;

fn print(api: &HnClient, items: &[u32]) {
    for id in items {
        println!("id = {}", id);

        let item = api.get_json(*id).unwrap();
        println!("{:?}",item);
    }
}

fn main() {
    let api = HnClient::init().unwrap();
    let top = [21669726];
    let count = 1;
    print(&api, &top[..count]);
}
