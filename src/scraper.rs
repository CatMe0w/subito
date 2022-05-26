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
    client: &reqwest::Client,
    conn: &Connection,
) -> Result<(), Box<dyn Error>> {
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

    // save to db
    // todo: remove db executions from scraper module?
    for user in user_list.as_array().unwrap() {
        conn.execute(
            "insert or ignore into user values (?1,?2,?3,?4)",
            params![
                user["id"].as_str(),
                user["name"].as_str().unwrap_or(""),
                user["name_show"].as_str(),
                user["portrait"].as_str(),
            ],
        )?;
    }
    for post in post_list.as_array().unwrap() {
        conn.execute(
            "insert or ignore into post values (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            params![
                post["id"].as_str(),
                post["floor"].as_str(),
                post["author_id"].as_str(),
                post["content"].as_str(),
                get_cst_datetime(post["time"].as_str().unwrap().to_string()),
                post["sub_post_number"].as_str(),
                post["signature"].as_str(),
                post["tail"].as_str(),
                thread_id,
            ],
        )?;
    }
    Ok(())
}
