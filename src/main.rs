extern crate clap;
extern crate config;
extern crate oauth_client as oauth;
extern crate serde_json;

use clap::{App, Arg, ArgMatches, SubCommand};
use oauth::Token;
use serde_json::Value;

use std::collections::HashMap;
use std::env;
use std::process;

mod key {
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

mod subcommand {
    pub const TIMELINE: &'static str = "timeline";
    pub const TIMELINE_SHORT: &'static str = "tl";
    pub const TWEET: &'static str = "tweet";
    pub const TWEET_SHORT: &'static str = "tw";
}

#[derive(Debug)]
struct Twicli {
    consumer: oauth::Token<'static>,
    access: oauth::Token<'static>,
}

impl Twicli {
    fn new() -> Self {
        let mut cfg_path = env::home_dir().unwrap();
        cfg_path.push(".twclirc.yaml");

        let mut settings = config::Config::default();
        settings
            .merge(config::File::from(cfg_path.as_path()))
            .unwrap();
        let mut cfg = settings.try_into::<HashMap<String, String>>().unwrap();

        if !cfg.contains_key(key::CONSUMER_KEY)
            || !cfg.contains_key(key::CONSUMER_SECRET)
            || !cfg.contains_key(key::ACCESS_TOKEN)
            || !cfg.contains_key(key::ACCESS_TOKEN_SECRET)
        {
            println!(".twclirc.yml is invalid!");
            process::exit(1);
        }

        Self {
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

    fn timeline(&self, count: &str) {
        let mut params = HashMap::new();
        params.insert("count".into(), count.into());

        let bytes = oauth::get(
            api::HOME_TIMELINE,
            &self.consumer,
            Some(&self.access),
            Some(&params),
        )
        .unwrap();

        let last_tweets_json = String::from_utf8(bytes).unwrap();
        let tweets: Vec<Value> = serde_json::from_str(&last_tweets_json).unwrap();

        if tweets.is_empty() {
            println!("No tweet(´・ω・｀)");
            return;
        }

        for t in tweets {
            println!("{}", t["user"]["name"].to_string().trim_matches('"'));
            println!();
            println!(
                "{}",
                t["text"]
                    .to_string()
                    .replace("\\n", "\n")
                    .replace("\\t", "\t")
                    .trim_matches('"')
            );
            println!();
            println!("{}", t["created_at"].to_string().trim_matches('"'));
            println!(
                "---------------------------------------------------\
                 ---------------------------------------------------"
            );
        }
    }

    fn tweet(&self, status: &str) {
        let mut params = HashMap::new();
        params.insert("status".into(), status.into());

        match oauth::post(
            api::STATUSES_UPDATE,
            &self.consumer,
            Some(&self.access),
            Some(&params),
        ) {
            Ok(_) => println!("Success."),
            Err(err) => println!("{}", err.to_string()),
        }
    }
}

fn main() {
    let app_m = App::new("twcli-rust")
        .version("0.2")
        .author("Kenichi Sakiyama.")
        .about("This is very simple Twitter cli client with Rust.")
        .subcommand(
            SubCommand::with_name(subcommand::TIMELINE)
                .about("Show home timeline.")
                .arg(Arg::with_name("count").index(1)),
        )
        .subcommand(
            SubCommand::with_name(subcommand::TWEET)
                .about("Tweet your status.")
                .arg(Arg::with_name("status").index(1).required(true)),
        )
        .subcommand(
            SubCommand::with_name(subcommand::TIMELINE_SHORT)
                .about("Show home timeline.")
                .arg(Arg::with_name("count").index(1)),
        )
        .subcommand(
            SubCommand::with_name(subcommand::TWEET_SHORT)
                .about("Tweet your status.")
                .arg(Arg::with_name("status").index(1).required(true)),
        )
        .usage("twcli timeline(tl) &{count} / twcli tweet(tw) \"${status}\"")
        .get_matches();

    app_m.subcommand_name().map(|subcmd| match subcmd.as_ref() {
        subcommand::TIMELINE => {
            exec_timeline(&app_m, subcommand::TIMELINE);
        }
        subcommand::TIMELINE_SHORT => {
            exec_timeline(&app_m, subcommand::TIMELINE_SHORT);
        }
        subcommand::TWEET => {
            exec_tweet(&app_m, subcommand::TWEET);
        }
        subcommand::TWEET_SHORT => {
            exec_tweet(&app_m, subcommand::TWEET_SHORT);
        }
        _ => {}
    });
}

fn exec_timeline(app_m: &ArgMatches, sub_command: &str) {
    let twitter = Twicli::new();

    if let Some(sub_m) = app_m.subcommand_matches(sub_command) {
        match sub_m.value_of("count") {
            Some(c) => twitter.timeline(&c),
            None => twitter.timeline("20"),
        }
    }
}

fn exec_tweet(app_m: &ArgMatches, sub_command: &str) {
    let twitter = Twicli::new();

    if let Some(sub_m) = app_m.subcommand_matches(sub_command) {
        match sub_m.value_of("status") {
            Some(s) => twitter.tweet(&s),
            None => println!("Need status."),
        }
    }
}
