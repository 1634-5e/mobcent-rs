use std::marker::PhantomData;

use reqwest::{Client, Method, Url};
use serde::Deserialize;

use crate::utils::{Error, NORMAL_CONTENT_TYPE};

// FIXME: 一个Command在文档中指url请求，有一定的格式要求，请求成功会返回一个类型的数据，目前稍微有点对不上，主要是Common Post Request和Upload Post ~

//这里的Command沿用了原API文档中的“命令"
#[derive(Debug, Clone)]
pub struct UrlWithQuery<Command>(pub Url, pub PhantomData<Command>);

impl<Response: for<'de> Deserialize<'de>> UrlWithQuery<Response> {
    const METHOD: Method = Method::POST;
    const CONTENT_TYPE: &'static str = NORMAL_CONTENT_TYPE;

    pub async fn fetch(self) -> Result<Response, Error> {
        let res = http_request(
            Client::new(),
            UrlWithQuery::<Response>::METHOD,
            self.0,
            UrlWithQuery::<Response>::CONTENT_TYPE,
        )
        .await?;

        println!("{:#}", res);

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
