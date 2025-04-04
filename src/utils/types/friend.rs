use serde::Deserialize;

pub struct Friend {
    pub id: i32,
    pub username: String,
    pub profile_picture: Option<String>,
}

#[derive(serde::Serialize)]
pub struct FriendRequest {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub username: String,
}

#[derive(Deserialize)]
pub struct FriendRequestForm {
    #[serde(rename = "receiverUsername")]
    pub receiver_username: String,
    pub token: String,
}

#[derive(Deserialize)]
pub struct FriendForm {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub token: String,
}
