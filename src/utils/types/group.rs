use serde::Deserialize;
#[derive(Deserialize)]
pub struct CreateGroupForm {
    pub token: String,
    #[serde(rename = "groupName")]
    pub group_name: String,
    #[serde(rename = "memberIds")]
    pub member_ids: Vec<i32>,
}

#[derive(Deserialize)]
pub struct AddUsersForm {
    pub token: String,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "newMemberIds")]
    pub new_member_ids: Vec<i32>,
}

#[derive(Deserialize)]
pub struct RemoveUserForm {
    pub token: String,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "removeId")]
    pub remove_id: i32,
}

#[derive(Deserialize)]
pub struct EditPictureForm {
    pub token: String,
    #[serde(rename = "groupId")]
    pub group_id: i32,
    #[serde(rename = "pictureUrl")]
    pub picture_url: String,
}