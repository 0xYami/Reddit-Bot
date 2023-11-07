use serde_json;
use serde_json::json;

pub fn format(post: &serde_json::Value) -> serde_json::Value {
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
        "Title": title,
        "Downvotes": downvotes,
        "Upvotes": upvotes,
        "Upvote Ratio": upvote_ratio,
        "Total Awards Received": total_awards_received,
        "Number of Comments": num_comments,
        "Post Score": score,
        "Permanent Link": "https://reddit.com {}".replace("{}", permalink.as_str().unwrap()),
        "Post Author": author,
        "Subreddit Name Prefixed": subreddit_name_prefixed,
        "Subreddit Subscriber Count": subscriber_count,
        "Post Created (UTC)": created_utc
    });
    
    return post_data;
}
