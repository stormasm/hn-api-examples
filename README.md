
This is a fork of
[hn_api](https://github.com/dbrgn/hn_api)

with the following added capabilities:

* add in an item_type

### Start Switch

So the idea behind the start switch is to figure
out where in the Hacker News feed you want to
start grabbing ids.  For now the switch will only
define the upper bound and will assume that the
lower bound will be calculated based on the number
of ids you define you want to pull.

The starting id can come from one of three places

1) A number you pass into the code from which to start
2) A number you read from a Redis key
3) The max_item_id based on this code
```
    let max_item_id = api.get_max_item_id().unwrap();
```

[Max Item Id](https://github.com/HackerNews/API#max-item-id)
which is [located here](https://hacker-news.firebaseio.com/v0/maxitem.json?print=pretty)
