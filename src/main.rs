#[macro_use]
extern crate serde_derive;
extern crate oauth_client as oauth;
extern crate serde_json;

use oauth::Token;
use std::collections::HashMap;
use serde_json::{Value, Error};

fn run() {
    let home_timeline: &'static str = "https://api.twitter.com/1.1/statuses/home_timeline.json";
    let consumer_key = "abc";
    let consumer_secret = "abc";
    let access_token = "abv";
    let access_token_secret = "abv";

    let consumer = Token::new(consumer_key, consumer_secret);
    let access = Token::new(access_token, access_token_secret);

    let bytes = oauth::get(home_timeline, &consumer, Some(&access), None).unwrap();
    let last_tweets_json = String::from_utf8(bytes).unwrap();
    let ts : Vec<Value> = serde_json::from_str(&last_tweets_json).unwrap();

    if ts.is_empty() {
        println!("No tweet in your timeline...");
    } else {
        for t in ts {
            println!("{}::{}", t["user"]["name"], t["text"]);
            // println!("{}", t["text"]);
            println!("{}", t["created_at"]);
            println!("---------------------------------------------------");
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tweet {
    pub created_at: String,
    pub text: String,
    // pub user: String,
    // pub name: String
    pub user: HashMap<String, String>
}

fn main() {
    run();
    println!("Hello, world!");
}
