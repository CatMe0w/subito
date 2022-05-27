use serde::Serialize;

pub mod scraper;
pub mod content_parser;

pub struct User {
    pub user_id: i64,
    pub username: Option<String>,
    pub nickname: String,
    pub avatar: String,
}

pub struct Thread {
    pub thread_id: i64,
    pub op_user_id: i64,
    pub title: String,
    pub user_id: i64,
    pub time: String,
    pub reply_num: i32,
    pub is_good: bool,
    pub op_post_content: Vec<Content>,
}

pub struct Post {
    pub post_id: i64,
    pub floor: i32,
    pub user_id: i64,
    pub content: Vec<Content>,
    pub time: String,
    pub comment_num: i32,
    pub signature: Option<String>,
    pub tail: Option<String>,
}

pub struct Comment {
    pub comment_id: i64,
    pub user_id: i64,
    pub content: Vec<Content>,
    pub time: String,
}

#[derive(Serialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub _type: String,
    pub content: String,
}