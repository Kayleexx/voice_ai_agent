use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use crate::schema::logs;

#[derive(Queryable, Debug, Serialize)]
pub struct Log {
    pub id: i32,
    pub user_input: String,
    pub bot_response: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = logs)]
pub struct NewLog<'a> {
    pub user_input: &'a str,
    pub bot_response: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct WebhookEvent {
    pub user_input: String,
}

#[derive(Debug, Serialize)]
pub struct BotResponse {
    pub response: String,
}
