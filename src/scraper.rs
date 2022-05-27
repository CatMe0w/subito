use crate::{content_parser::parse, Post, User};
use chrono::{TimeZone, Utc};
use chrono_tz::Asia::Shanghai;
use rusqlite::{params, Connection, Result};
use serde_json::Value;
use std::{collections::BTreeMap, error::Error};

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

fn get_cst_datetime(timestamp: String) -> String {
    // cst: china standard time, utc+8
    let timestamp = timestamp.parse::<i64>().unwrap();
    let datetime = Utc.timestamp(timestamp, 0).with_timezone(&Shanghai);
    let datetime = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    datetime
}

pub async fn fetch_thread(
    thread_id: i64,
    pseudo_page: i32,
    post_id: Option<i64>,
    client: &reqwest::Client,
) -> Result<(Vec<User>, Vec<Post>), Box<dyn Error>> {
    // fetch json
    let mut post_body = BTreeMap::new();
    let kz = thread_id.to_string(); // to make borrow checker happy
    post_body.insert("kz", kz.as_str());
    post_body.insert("_client_version", "9.9.8.32");
    let sign = make_sign(&post_body);
    post_body.insert("sign", sign.as_str());

    let res = client
        .post("https://tieba.baidu.com/c/f/pb/page")
        .form(&post_body)
        .send()
        .await?
        .text()
        .await?;

    // parse json
    let res: Value = serde_json::from_str(&res)?;
    // ok i decided to keep it untyped since constructing from baidu's shit is totally a pain
    let user_list: Value = serde_json::from_value(res["user_list"].clone())?;
    let post_list: Value = serde_json::from_value(res["post_list"].clone())?;

    let mut users: Vec<User> = Vec::new();
    for user in user_list.as_array().unwrap() {
        users.push(User {
            user_id: user["id"].as_str().unwrap().parse()?,
            username: user["name"].as_str().map(str::to_string), // keep the same behavior as og proma
            nickname: user["name_show"].as_str().unwrap().to_string(),
            avatar: user["portrait"].as_str().unwrap().to_string(),
        });
    }

    let mut posts: Vec<Post> = Vec::new();
    for post in post_list.as_array().unwrap() {
        posts.push(Post {
            post_id: post["id"].as_str().unwrap().parse()?,
            floor: post["floor"].as_str().unwrap().parse()?,
            user_id: post["author_id"].as_str().unwrap().parse()?,
            content: parse(&post["content"])?,
            time: get_cst_datetime(post["time"].as_str().unwrap().to_string()),
            comment_num: post["sub_post_number"].as_str().unwrap().parse()?,
            signature: post["signature"].as_str().map(str::to_string),
            tail: post["tail"].as_str().map(str::to_string),
        });
    }

    Ok((users, posts))
}
