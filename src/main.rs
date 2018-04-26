extern crate clap;
extern crate config;
extern crate oauth_client as oauth;
extern crate serde_json;

use oauth::Token;
use serde_json::Value;
use clap::{App, Arg, SubCommand};

use std::collections::HashMap;
use std::env;
use std::process;

#[derive(Debug)]
struct Twicli {
    consumer: oauth::Token<'static>,
    access: oauth::Token<'static>,
}

mod keys {
    pub const CONSUMER_KEY: &'static str = "consumer_key";
    pub const CONSUMER_SECRET: &'static str = "consumer_secret";
    pub const ACCESS_TOKEN: &'static str = "access_token";
    pub const ACCESS_TOKEN_SECRET: &'static str = "access_token_secret";
}

mod api {
    pub const STATUSES_UPDATE: &'static str = "https://api.twitter.com/1.1/statuses/update.json";
    pub const HOME_TIMELINE: &'static str =
        "https://api.twitter.com/1.1/statuses/home_timeline.json";
}

impl Twicli {
    fn new() -> Twicli {
        let mut cfg_path = env::home_dir().unwrap();
        cfg_path.push(".twclirc.yml");

        let mut settings = config::Config::default();
        settings
            .merge(config::File::from(cfg_path.as_path()))
            .unwrap();
        let mut cfg = settings.try_into::<HashMap<String, String>>().unwrap();

        if !cfg.contains_key(keys::CONSUMER_KEY) || !cfg.contains_key(keys::CONSUMER_SECRET)
            || !cfg.contains_key(keys::ACCESS_TOKEN)
            || !cfg.contains_key(keys::ACCESS_TOKEN_SECRET)
        {
            println!(".twclirc.yml is invalid!");
            process::exit(1);
        }

        Twicli {
            consumer: Token::new(
                cfg.remove("consumer_key").unwrap(),
                cfg.remove("consumer_secret").unwrap(),
            ),
            access: Token::new(
                cfg.remove("access_token").unwrap(),
                cfg.remove("access_token_secret").unwrap(),
            ),
        }
    }

    fn timeline(self, count: &str) {
        let mut params = HashMap::new();
        params.insert("count".into(), count.into());

        let bytes = oauth::get(
            api::HOME_TIMELINE,
            &self.consumer,
            Some(&self.access),
            Some(&params),
        ).unwrap();
        let last_tweets_json = String::from_utf8(bytes).unwrap();
        let tweets: Vec<Value> = serde_json::from_str(&last_tweets_json).unwrap();

        if tweets.is_empty() {
            println!("No tweet(´・ω・｀)");
        } else {
            for t in tweets {
                println!("{}", t["user"]["name"].to_string().trim_matches('"'));
                println!("");
                println!(
                    "{}",
                    t["text"]
                        .to_string()
                        .replace("\\n", "\n")
                        .replace("\\t", "\t")
                        .trim_matches('"')
                );
                println!("");
                println!("{}", t["created_at"].to_string().trim_matches('"'));
                println!(
                    "---------------------------------------------------\
                     ---------------------------------------------------"
                );
            }
        }
    }

    fn tweet(self, status: &str) {
        let mut params = HashMap::new();
        params.insert("status".into(), status.into());

        match oauth::post(
            api::STATUSES_UPDATE,
            &self.consumer,
            Some(&self.access),
            Some(&params),
        ) {
            Ok(_) => {
                // let resp = String::from_utf8(bytes).unwrap();
                // println!("{:?}", resp);
                println!("Success.");
            }
            Err(err) => {
                println!("{}", err.to_string());
            }
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
        .subcommand(
            SubCommand::with_name("tl")
                .about("Show home timeline.")
                .arg(Arg::with_name("count").index(1)),
        )
        .subcommand(
            SubCommand::with_name("tw")
                .about("Tweet your status.")
                .arg(Arg::with_name("status").index(1).required(true)),
        )
        .usage("twcli timeline(tl) &{count} / twcli tweet(tw) \"${status}\"")
        .get_matches();

    let twitter = Twicli::new();

    match app_m.subcommand_name() {
        Some("timeline") => {
            if let Some(sub_m) = app_m.subcommand_matches("timeline") {
                match sub_m.value_of("count") {
                    Some(c) => twitter.timeline(&c),
                    None => twitter.timeline("20"),
                }
            }
        }
        Some("tl") => {
            if let Some(sub_m) = app_m.subcommand_matches("tl") {
                match sub_m.value_of("count") {
                    Some(c) => twitter.timeline(&c),
                    None => twitter.timeline("20"),
                }
            }
        }
        Some("tweet") => {
            if let Some(sub_m) = app_m.subcommand_matches("tweet") {
                match sub_m.value_of("status") {
                    Some(s) => {
                        twitter.tweet(&s);
                    }
                    None => {
                        println!("Need status.");
                    }
                }
            }
        }
        Some("tw") => {
            if let Some(sub_m) = app_m.subcommand_matches("tw") {
                match sub_m.value_of("status") {
                    Some(s) => {
                        twitter.tweet(&s);
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
