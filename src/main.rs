use reqwest;
use serde_json;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://reddit.com/.api")
        .header("User-Agent", "HackedRedditClient/0.1")
        .send()
        .await?;

    let content = response.text().await?;

    let json = to_json(content);

    println!("{:#?}", json);
    
    Ok(())
}

fn to_json(string: String) -> serde_json::Value {
    let v: serde_json::Value = serde_json::from_str(&string).unwrap();

    return v;
}

