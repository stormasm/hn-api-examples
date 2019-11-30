
Understand this code:

```
as_ref().unwrap()
```

from here:

```
/// Return the author of this item, if available.
pub fn author(&self) -> Option<&str> {
    match self {
        Item::Story(story) => Some(&story.by.as_ref().unwrap()),
        Item::Comment(comment) => Some(&comment.by.as_ref().unwrap()),
        Item::Poll(poll) => Some(&poll.by.as_ref().unwrap()),
        Item::Pollopt(pollopt) => Some(&pollopt.by.as_ref().unwrap()),
        _ => None,
    }
}
```

This null case is breaking the code...

Eventually I need to deal with this case,
but for now lets keep going with other
issues.

https://hacker-news.firebaseio.com/v0/item/21664126.json?print=pretty

[21664126,21655779,21655225,21654032,21649766,21659764,21647538,21647535,21647533,21638992,21638989,21636046,21632923,21625361]

These issues have been addressed...

https://hacker-news.firebaseio.com/v0/item/21654193.json?print=pretty

https://hacker-news.firebaseio.com/v0/item/21654248.json?print=pretty

https://hacker-news.firebaseio.com/v0/item/21661326.json?print=pretty

https://hacker-news.firebaseio.com/v0/item/21661425.json?print=pretty

https://hacker-news.firebaseio.com/v0/item/21661449.json?print=pretty

https://hacker-news.firebaseio.com/v0/item/21664796.json?print=pretty

https://hacker-news.firebaseio.com/v0/item/21664828.json?print=pretty
