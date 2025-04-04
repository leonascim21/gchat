use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,
    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}
