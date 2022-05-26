use rusqlite::{params, Connection, Result};
use serde_json::Value;
use std::{collections::BTreeMap, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("proma.db")?;
    if let Err(e) = setup_tables(&conn) {
        match e.to_string().as_str() {
            "table user already exists" => println!("Tables already exist, continuing"),
            _ => panic!("Error: {}", e),
        }
    }

    let client = reqwest::Client::new();
    fetch_thread(7831278321, &client, &conn).await?;

    Ok(())
}

fn setup_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "create table user(
        id numeric primary key not null,
        username text,
        nickname text,
        avatar text not null)",
        [],
    )?;
    conn.execute(
        "create table thread(
        id numeric primary key not null,
        title text not null,
        user_id numeric not null,
        reply_num numeric not null,
        is_good numeric not null,
        foreign key(user_id) references user(id))",
        [],
    )?;
    conn.execute(
        "create table post(
        id numeric primary key not null,
        floor numeric not null,
        user_id numeric not null,
        content text,
        time text not null,
        comment_num numeric not null,
        signature text,
        tail text,
        thread_id numeric not null,
        foreign key(user_id) references user(id),
        foreign key(thread_id) references thread(id))",
        [],
    )?;
    conn.execute(
        "create table comment(
        id numeric primary key not null,
        user_id numeric not null,
        content text,
        time text not null,
        post_id numeric not null,
        foreign key(user_id) references user(id),
        foreign key(post_id) references post(id))",
        [],
    )?;
    Ok(())
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

async fn fetch_thread(
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
                post["time"].as_str(),
                post["sub_post_number"].as_str(),
                post["signature"].as_str(),
                post["tail"].as_str(),
                thread_id,
            ],
        )?;
    }
    Ok(())
}
