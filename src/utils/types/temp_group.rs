use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct CreateTempGroupForm {
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub password: Option<String>,
    pub token: String,
}

pub struct TempGroupsInfo {
    pub temp_chat_key: String,
    pub group_id: i32,
    pub end_date: DateTime<Utc>,
    pub password: Option<String>,
    pub name: String,
}