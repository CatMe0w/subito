use rusqlite::{Connection, Result};
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
async fn main() -> Result<()> {
    let conn = Connection::open("proma.db")?;
    match setup_tables(&conn) {
        Ok(_) => println!("Tables created"),
        Err(e) => match e.to_string().as_str() {
            "table user already exists" => println!("Tables already exist, continuing"),
            _ => panic!("Error: {}", e),
        },
    }

    let client = reqwest::Client::new();
    fetch_thread(7831278321, client).await?;
    
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
