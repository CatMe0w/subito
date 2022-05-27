use rusqlite::{params, Connection, Result};
use std::error::Error;
use subito::scraper::fetch_thread;

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
    let thread_id = 7831278321;
    let (users, posts) = fetch_thread(thread_id, 1, None, &client, &conn).await?;
    
    for user in users {
        conn.execute(
            "insert or ignore into user values (?1,?2,?3,?4)",
            params![user.user_id, user.username, user.nickname, user.avatar],
        )?;
    }

    for post in posts {
        conn.execute(
            "insert or ignore into post values (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            params![
                post.post_id,
                post.floor,
                post.user_id,
                serde_json::to_string(&post.content)?,
                post.time,
                post.comment_num,
                post.signature,
                post.tail,
                thread_id,
            ],
        )?;
    }

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
