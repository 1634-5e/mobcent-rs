use crate::utils::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Board {
    board_id: Number,
    board_name: String,
    description: String,
    board_child: Number,
    board_img: String,
    last_posts_date: String,
    board_content: Number,
    #[serde(rename = "forumRedirect")]
    forum_redirect: String,
    #[serde(rename = "favNum")]
    fav_num: Number,
    td_posts_num: Number,
    topic_total_num: Number,
    posts_total_num: Number,
    is_focus: Boolean,
}