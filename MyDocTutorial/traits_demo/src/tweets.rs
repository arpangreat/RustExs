use super::*;

pub fn run() {
    let tweet = Tweet {
        username: String::from("swastik"),
        content: String::from("YAYAYAYAYYAYYYAYAYAYAYYAYAYA!!!!!!!"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
