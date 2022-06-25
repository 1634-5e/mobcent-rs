use crate::utils::*;
use crate::board::Board;

#[derive(Debug, Clone, Copy)]
pub enum SortBy {
    // #[serde(rename = "publish == new")]
    NewlyPublished,
    // #[serde(rename = "essence == marrow")]
    MarrowEssence,
    // #[serde(rename = "top")]
    Top,
    // #[serde(rename = "photo")]
    Photo,
    // #[serde(rename = "all")]
    All,
}

impl SortBy {
    fn as_str(&self) -> &str {
        match self {
            SortBy::NewlyPublished => "publish == new",
            SortBy::MarrowEssence => "essence == marrow",
            SortBy::Top => "top",
            SortBy::Photo => "photo",
            SortBy::All => "all",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FilterType {
    #[serde(rename = "0")]
    NoTopPosts,
    #[serde(rename = "1")]
    TopPosts,
    #[serde(rename = "2")]
    FilteredTopPosts,
    #[serde(rename = "3")]
    GlobalTopPosts,
}

impl FilterType {
    fn as_str(&self) -> &str {
        match self {
            FilterType::NoTopPosts => "0",
            FilterType::TopPosts => "1",
            FilterType::FilteredTopPosts => "2",
            FilterType::GlobalTopPosts => "3",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TopOrder {
    // #[serde(rename = "sortid")]
    SortId,
    // #[serde(rename = "typeid")]
    TypeId,
}

impl TopOrder {
    fn as_str(&self) -> &str {
        match self {
            TopOrder::SortId => "sortid",
            TopOrder::TypeId => "typeid",
        }
    }
}

impl UrlWithQuery<TopicList> {
    pub fn new() -> Result<Self, Error> {
        Ok(Self(
            Url::from_str("https://bbs.uestc.edu.cn/mobcent/app/web/index.php?r=forum/topiclist")?,
            PhantomData,
        ))
    }

    pub fn board_id(mut self, board_id: Id) -> Self {
        self.0
            .query_pairs_mut()
            .append_pair("boardId", board_id.to_string().as_str());
        self
    }

    pub fn page(mut self, page: Number) -> Self {
        self.0
            .query_pairs_mut()
            .append_pair("page", page.to_string().as_str());
        self
    }

    pub fn page_size(mut self, page_size: Number) -> Self {
        self.0
            .query_pairs_mut()
            .append_pair("pageSize", page_size.to_string().as_str());
        self
    }

    pub fn sort_by(mut self, sort_by: SortBy) -> Self {
        self.0
            .query_pairs_mut()
            .append_pair("sortby", sort_by.as_str());
        self
    }

    pub fn filter_type(mut self, filter_type: FilterType) -> Self {
        self.0
            .query_pairs_mut()
            .append_pair("filterType", filter_type.as_str());
        println!("{}", self.0);
        self
    }

    pub fn filter_id(mut self, filter_id: Id) -> Self {
        self.0
            .query_pairs_mut()
            .append_pair("filterId", filter_id.to_string().as_str());
        self
    }

    pub fn is_image_list(mut self, is_image_list: bool) -> Self {
        self.0
            .query_pairs_mut()
            .append_pair("isImageList", is_image_list.to_string().as_str());
        self
    }

    pub fn top_order(mut self, top_order: TopOrder) -> Self {
        self.0
            .query_pairs_mut()
            .append_pair("topOrder", top_order.as_str());
        self
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopicList {
    #[serde(flatten)]
    common_header: CommonHeader,
    #[serde(rename = "newTopicPanel")]
    new_topic_panel: Vec<NewTopicPanel>,
    list: Vec<Board>,
    online_user_num: Number,
    td_visitors: Number,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct NewTopicPanel {
    #[serde(rename = "type")]
    topic_type: FilterType,
    action: String,
    title: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct BoardCategory {
    board_category_id: Number,
    board_category_name: String,
    board_category_type: Id,
    board_list: Vec<Board>,
}

impl TopicList {
    pub fn new() -> Result<UrlWithQuery<TopicList>, Error> {
        Ok(UrlWithQuery::<TopicList>::new()?)
    }
}
