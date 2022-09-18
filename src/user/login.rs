use std::os::raw;

use crate::{structs::UrlWithQuery, utils::*};

#[derive(Debug, Clone, Copy)]
pub enum LogType {
    Login,
    Logout,
}

impl LogType {
    fn as_str(&self) -> &str {
        match self {
            LogType::Login => "login",
            LogType::Logout => "logout",
        }
    }
}

impl UrlWithQuery<Login> {
    pub fn new() -> Self {
        Self(
            Url::from_str("https://bbs.uestc.edu.cn/mobcent/app/web/index.php?r=user/login")
                .unwrap(),
            PhantomData,
        )
    }

    pub fn log_type(mut self, log_type: LogType) -> Self {
        self.0
            .query_pairs_mut()
            .append_pair("type", log_type.as_str());
        self
    }

    pub fn username(mut self, username: &str) -> Self {
        self.0.query_pairs_mut().append_pair("username", username);
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.0.query_pairs_mut().append_pair("password", password);
        self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Login {
    #[serde(flatten)]
    common_header: CommonHeader,
    #[serde(rename = "isValidation")]
    is_validation: Boolean,
    token: String,
    secret: String,
    score: Number,
    uid: Id,
    #[serde(rename = "userName")]
    username: String,
    avatar: String,
    gender: Flag,
    #[serde(rename = "userTitle")]
    user_title: String,
    #[serde(rename = "creditShowList")]
    credit_show_list: Vec<Credit>,

    //这些意义不明，保留为了方便
    #[serde(skip)]
    #[serde(rename = "repeatList")]
    repeat_list: Never,
    #[serde(skip)]
    verify: Never,
    #[serde(skip)]
    mobile: Never,
    #[serde(skip)]
    #[serde(rename = "group_id")]
    group_id: Never,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Credit {
    credit_type: String,
    title: String,
    data: Number,
}

impl Login {
    pub fn build() -> UrlWithQuery<Login> {
        UrlWithQuery::<Login>::new()
    }
}
