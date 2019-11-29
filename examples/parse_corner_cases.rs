use hn_api::HnClient;

fn print(api: &HnClient, items: &[u32]) {
    for id in items {
        println!("id = {}", id);
        let _item = api.get_item(*id).unwrap().unwrap();

        // let item = api.get_item(*id).to_string();

        // let item = api.get_item(*id).unwrap().to_string();
        // println!("{}",item);

        /*
                let _err = match item {
                        Some(item) => {
                            println!("Good to go")
                        },
                        None => {
                            println!("Got error")
                        },
                    };
        */

        // println!("{} {}", id, item.id());

        // println!("- {}: {}", item.id(), item.title().unwrap_or("?"),)
    }
}

fn main() {
    let api = HnClient::init().unwrap();

    // This is the call that returns a
    // working array
    //let top = api.get_top_stories().unwrap();

    // This works
    // let top = [21655958, 21656551, 21656190];
    // This breaks
    // let top = [21654193, 21655958, 21656551];

    // working comment
    // let top = [21663922];

    // working job
    // let top = [21655200];

    // breaking deleted
    let top = [21665489];
    let count = 1;

    print(&api, &top[..count]);
}
