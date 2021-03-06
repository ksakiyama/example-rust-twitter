# Twitter CLI Client Example in Rust
This is a example of Twitter CLI client written in Rust.

This is the first project I wrote in Rust.
I found it difficult to learn Rust and it took me a while to make this program. However, Rust is a very nice and beautiful programming language.

If you are a Rust beginner and refer to my code, I'll be very happy.

## Usage
This small client has only two functions. Show timeline and tweet comment. You can use following commands.

### 0. Prepare
```bash
# resolve dependencies and build
cargo build
```

Save a configuration file on your home directory. 

~/.twclirc.yaml
```
CONSUMER_KEY: aaa
CONSUMER_SECRET: bbb
ACCESS_TOKEN: ccc
ACCESS_TOKEN_SECRET: ddd
```

### 1. Show home timeline
```bash
# show 10 tweets of your timeline in your commandline.
cargo run timeline 10
```

### 2. Tweet your text
```bash
# tweet your text on Twitter.
cargo run tweet "test tweet. remove this later."
```

## Dependencies
I used four libralies. They helped me a lot to get my program finished faster. And I realized that Rust's standard library has a few network library?

```toml
oauth-client = "0.3"
serde_json = "1.0.16"
config = "0.8"
clap = "~2.31"
```

* [gifnksm/oauth-client-rs](https://github.com/gifnksm/oauth-client-rs)
* [serde-rs/json](https://github.com/serde-rs/json)
* [mehcode/config-rs](https://github.com/mehcode/config-rs)
* [kbknapp/clap-rs](https://github.com/kbknapp/clap-rs)

## Reference
Thank you for your source code. I frequently refered to this reposiory.<br>
[gifnksm/twitter-api-rs](https://github.com/gifnksm/twitter-api-rs)