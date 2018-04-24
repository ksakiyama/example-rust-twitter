extern crate clap;
extern crate config;
extern crate oauth_client as oauth;
extern crate serde_json;

use oauth::Token;
use serde_json::Value;
use clap::{App, Arg, SubCommand};

use std::collections::HashMap;
use std::env;

fn run() {
    let mut cfg_path = env::home_dir().unwrap();
    cfg_path.push(".twclirc.yml");
    cfg_path.as_path();

    let mut settings = config::Config::default();
    settings
        .merge(config::File::from(cfg_path.as_path()))
        .unwrap();
    let cfg = settings.try_into::<HashMap<String, String>>().unwrap();

    let home_timeline: &'static str = "https://api.twitter.com/1.1/statuses/home_timeline.json";

    let consumer_key = &cfg["consumer_key"];
    let consumer_secret = &cfg["consumer_secret"];
    let access_token = &cfg["access_token"];
    let access_token_secret = &cfg["access_token_secret"];

    let consumer = Token::new(consumer_key.as_str(), consumer_secret.as_str());
    let access = Token::new(access_token.as_str(), access_token_secret.as_str());

    let mut params = HashMap::new();
    params.insert("count".into(), "5".into());

    let bytes = oauth::get(home_timeline, &consumer, Some(&access), Some(&params)).unwrap();
    let last_tweets_json = String::from_utf8(bytes).unwrap();
    let tweets: Vec<Value> = serde_json::from_str(&last_tweets_json).unwrap();

    if tweets.is_empty() {
        println!("No tweet in your timeline...");
    } else {
        for t in tweets {
            println!("{}::{}", t["user"]["name"], t["text"]);
            println!("{}", t["created_at"]);
            println!("---------------------------------------------------");
        }
    }
}

fn main() {
    let app_m = App::new("twcli-rust")
        .version("1.0")
        .author("Kenichi Sakiyama.")
        .about("This is very simple Twitter cli client with Rust.")
        .subcommand(
            SubCommand::with_name("timeline")
                .about("Show home timeline.")
                .arg(Arg::with_name("count").index(1)),
        )
        .subcommand(
            SubCommand::with_name("tweet")
                .about("Tweet your status.")
                .arg(Arg::with_name("status").index(1).required(true)),
        )
        .usage("twcli timeline(tl) &{count} / twcli tweet(tw) ${status}")
        .get_matches();

    match app_m.subcommand_name() {
        Some("timeline") => {
            if let Some(sub_m) = app_m.subcommand_matches("timeline") {
                match sub_m.value_of("count") {
                    // set count
                    Some(c) => println!("{}", c),
                    None => run(),
                }
            }
        }
        Some("tweet") => {
            println!("tweet");
            if let Some(sub_m) = app_m.subcommand_matches("tweet") {
                match sub_m.value_of("status") {
                    Some(s) => {
                        println!("{}", s);
                    }
                    None => {
                        println!("Need status.");
                    }
                }
            }
        }
        _ => {
            println!("{}", app_m.usage());
        }
    }
}
