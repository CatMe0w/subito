use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
struct User {
    id: i64,
    username: String,
    nickname: String,
    avatar: String,
}

#[derive(Serialize, Deserialize)]
struct Thread {
    id: i64,
    title: String,
    user_id: i64,
    reply_num: i32,
    is_good: bool,
}

#[derive(Serialize, Deserialize)]
struct Post {
    id: i64,
    floor: i32,
    user_id: i64,
    content: String,
    time: String,
    comment_num: i32,
    signature: String,
    tail: String,
    thread_id: i64,
}

#[derive(Serialize, Deserialize)]
struct Comment {
    id: i64,
    user_id: i64,
    content: String,
    time: String,
    post_id: i64,
}
#[tokio::main]
async fn main() {
    fetch_thread(7831278321).await;
}

fn make_sign(post_body: &BTreeMap<&str, &str>) -> String {
    let mut sign = String::new();
    for (k, v) in post_body {
        sign.push_str(k);
        sign.push_str("=");
        sign.push_str(v);
    }
    sign.push_str("tiebaclient!!!");
    format!("{:X}", md5::compute(sign))
}
