pub use reqwest::{Client, Method, Url};
pub use serde::{Deserialize, Serialize};
pub use std::marker::PhantomData;
pub use std::str::FromStr;

use super::{Error, Flag, NORMAL_CONTENT_TYPE};

//模拟1代表true，0代表false
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(into = "u8")]
#[serde(from = "u8")]
pub enum Boolean {
    True,
    #[default]
    False,
}

//FIXME:为了保证对应关系，这里0/1之外的数值会panic
impl From<u8> for Boolean {
    fn from(n: u8) -> Self {
        match n {
            1 => Boolean::True,
            0 => Boolean::False,
            _ => panic!("Not a valid number"),
        }
    }
}

impl Into<u8> for Boolean {
    fn into(self) -> u8 {
        match self {
            Boolean::True => 1,
            Boolean::False => 0,
        }
    }
}

impl Into<bool> for Boolean {
    fn into(self) -> bool {
        match self {
            Boolean::True => true,
            Boolean::False => false,
        }
    }
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
pub struct Head {
    #[serde(rename = "errCode")]
    pub err_code: String,
    #[serde(rename = "errInfo")]
    pub err_info: String,
    pub version: String,
    pub alert: Flag,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Body {
    #[serde(rename = "externInfo")]
    pub extern_info: ExternInfo,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternInfo {
    pub padding: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Never;

//这里的Command沿用了原API文档中的“命令"， 配合UrlWithQuery
pub trait Command {
    const METHOD: Method;
    const CONTENT_TYPE: &'static str;
}

#[derive(Debug, Clone)]
pub struct UrlWithQuery<Command>(pub Url, pub PhantomData<Command>);

impl<T> Command for UrlWithQuery<T> {
    const METHOD: Method = Method::POST;
    const CONTENT_TYPE: &'static str = NORMAL_CONTENT_TYPE;
}

impl<T: Command> UrlWithQuery<T> {
    pub async fn fetch<Response: for<'de> Deserialize<'de>>(self) -> Result<Response, Error> {
        let res = http_request(
            Client::new(),
            UrlWithQuery::<T>::METHOD,
            self.0,
            UrlWithQuery::<T>::CONTENT_TYPE,
        )
        .await?;

        Ok(serde_json::from_str(res.as_str())?)
    }
}

pub async fn http_request(
    client: Client,
    method: Method,
    url: Url,
    content_type: &str,
) -> Result<String, Error> {
    Ok(client
        .request(method, url)
        .header("content-type", content_type)
        .send()
        .await?
        .text()
        .await?)
}
