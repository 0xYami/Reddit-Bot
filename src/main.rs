use reqwest;
use serde_json;
use serde_json::json;
use std::env;
use serde_yaml;

const ILLEGAL_CHARS: [&str; 8] = [
    "!",
    "#",
    "$",
    "^",
    "&",
    "*",
    "(",
    ")"
];

const HELP_MESSAGE: &str = "Usage: %path %prefix [pretty|raw|help]";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<_> = env::args().collect();
    let client = reqwest::Client::new();

    let config = std::fs::File::open("config.yml").expect("Error opening config.yml");

    let config: serde_yaml::Value = serde_yaml::from_reader(config).expect("Error parsing config.yml");

    let url = config["url"].as_str().unwrap();
    let cmd_prefix = config["cmd_prefix"].as_str().unwrap();
    
    let response = client
        .get(url)
        .header("User-Agent", "HackedRedditClient/0.1")
        .send()
        .await?;

    let content = response.text().await?;

    let json = to_json(content);

    let zeroarg = args.get(0).map(|s| s.as_str());

    for arg in args.iter() {
        for illegal_char in ILLEGAL_CHARS.iter() {
            if arg.contains(illegal_char) {
                println!("Error: Illegal character in argument: {}", illegal_char);
                std::process::exit(1);
            }
        }
    }



    match args.get(1).as_deref() {
        Some(cmd) if cmd == &format!("{}pretty", cmd_prefix) => {
            let mut i = 1;
            for post in json["data"]["children"].as_array().unwrap() {
                println!("Post #{}", i);
                println!();
                println!("{:#?}", get_post_data(&post["data"]));
                println!();
                i += 1;
            }
        }
        Some(cmd) if cmd == &format!("{}raw", cmd_prefix) => {
            println!("{:#?}", json);
        }
        Some(cmd) if cmd == &format!("{}help", cmd_prefix) => {
            println!("{}", HELP_MESSAGE.replace("%path", zeroarg.unwrap()).replace("%prefix", cmd_prefix));
        }
        _ => {
            println!("{}", HELP_MESSAGE.replace("%path", zeroarg.unwrap()).replace("%prefix", cmd_prefix));
        }
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
