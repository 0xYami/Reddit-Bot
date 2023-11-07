use serde_json;


pub fn parse(content: String) -> serde_json::Value {
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    json
}
