use crate::utils::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Topic {
    topic_id: Id,
    #[serde(rename = "type")]
    topic_type: TopicType,
    title: String,
    subject: String,
    #[serde(rename = "imageList")]
    image_list: Vec<String>,
    #[serde(rename = "sourceWebUrl")]
    source_web_url: String,
    user_id: Id,
    user_nick_name: String,
    #[serde(rename = "userAvatar")]
    user_avatar: String,
    gender: Gender,
    last_reply_date: String, //时间戳
    vote: Boolean,
    hot: Boolean,
    hits: Number,
    replies: Number,
    essence: Boolean,
    top: Boolean,
    status: Number,
    pic_path: String,
    ratio: Number,
    #[serde(rename = "recommendAdd")]
    recommend_add: Number,
    #[serde(rename = "isHasRecommendAdd")]
    is_has_recommend_add: Boolean,
    board_id: Id,
    board_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum TopicType {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "vote")]
    Vote,
}
