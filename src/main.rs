use reqwest;
use std::env;
use serde_yaml;

mod libs;

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
    let user_agent = config["user_agent"].as_str().unwrap();
    
    let response = client
        .get(url)
        .header("User-Agent", user_agent)
        .send()
        .await?;

    let content = response.text().await?;

    let json = libs::to_json::parse(content);

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
                println!("{:#?}", libs::get_post_data::format(&post["data"]));
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

