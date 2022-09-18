pub use reqwest::{Client, Method, Url};
pub use serde::{Deserialize, Serialize};
pub use std::marker::PhantomData;
pub use std::str::FromStr;

use super::NORMAL_CONTENT_TYPE;

pub type Flag = u8;
pub type Id = u16;
pub type Number = u32;

pub type Error = Box<dyn std::error::Error>;

//模拟1代表true，0代表false
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[repr(u8)]
pub enum Boolean {
    #[serde(rename = "1")]
    True = 1,
    #[default]
    #[serde(rename = "0")]
    False = 0,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Body {
    #[serde(rename = "externInfo")]
    pub extern_info: ExternInfo,
}

//为了简化结构体
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CommonHeader {
    pub rs: Boolean,
    #[serde(rename = "errcode")]
    pub err_code: String,
    pub head: Head,
    pub body: Body,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternInfo {
    pub padding: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Never;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Gender {
    #[serde(rename = "0")]
    Reserved,
    #[serde(rename = "1")]
    Male,
    #[serde(rename = "2")]
    Female,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Head {
    #[serde(rename = "errCode")]
    pub err_code: String,
    #[serde(rename = "errInfo")]
    pub err_info: String,
    pub version: String,
    pub alert: Flag,
}
