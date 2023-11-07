use reqwest;
use serde_json;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<_> = env::args().collect();
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://www.reddit.com/.api")
        .header("User-Agent", "HackedRedditClient/0.1")
        .send()
        .await?;

    let content = response.text().await?;

    let json = to_json(content);
    
     let pretty_print_data = args.get(1).map(|arg| arg == "pretty").unwrap_or(false);

    if pretty_print_data {
        let mut i = 0;

        for post in json["data"]["children"].as_array().unwrap() {
            println!("Post #{}", i);
            println!("{:#?}", get_post_data(&post["data"]));
            i += 1;
        }
    } else {
        println!("{}", serde_json::to_string(&json).unwrap());
    }
    Ok(())
}

fn to_json(string: String) -> serde_json::Value {
    let v: serde_json::Value = serde_json::from_str(&string).unwrap();

    return v;
}

fn get_post_data(post: &serde_json::Value) -> serde_json::Value {
    let title = &post["title"];
    let downvotes = &post["downs"];
    let upvotes = &post["ups"];
    let upvote_ratio = &post["upvote_ratio"];
    let total_awards_received = &post["total_awards_received"];
    let num_comments = &post["num_comments"];
    let score = &post["score"];
    let permalink = &post["permalink"];
    let author = &post["author"];
    let subreddit_name_prefixed = &post["subreddit_name_prefixed"];
    let subscriber_count = &post["subreddit_subscribers"];
    let created_utc = &post["created_utc"];

    let post_data = json!({
        "title": title,
        "downvotes": downvotes,
        "upvotes": upvotes,
        "upvote_ratio": upvote_ratio,
        "total_awards_received": total_awards_received,
        "num_comments": num_comments,
        "score": score,
        "permalink": permalink,
        "author": author,
        "subreddit_name_prefixed": subreddit_name_prefixed,
        "subscriber_count": subscriber_count,
        "created_utc": created_utc
    });
    
    return post_data;
}