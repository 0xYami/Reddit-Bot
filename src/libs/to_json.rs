use serde_json;

/** https://github.com/withervt/Reddit-Bot/wiki/Parsing-Data */
pub fn parse(content: String) -> serde_json::Value {
    let json: serde_json::Value = serde_json::from_str(&content).unwrap();
    json
}
