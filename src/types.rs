use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    remark: Option<String>,
    #[serde(rename="type")]
    user_type: Option<String>,
    pub id: Option<usize>,
    pub screen_name: Option<String>,
    following: bool,
    follow_me: bool,
    gender: Option<String>,
    photo_domain: Option<String>,
    profile_image_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    id: Option<usize>,
    user_id: Option<usize>,
    title: String,
    created_at: usize,
    retweet_count: usize,
    reply_count: usize,
    fav_count: usize,
    truncated: bool,
    #[serde(rename="commentId")]
    comment_id: usize,
    retweet_status_id: usize,
    symbol_id: Option<String>,
    pub description: String,
    #[serde(rename="type")]
    topic_type: Option<String>,
    source_link: Option<String>,
    edited_at: Option<usize>,
    pic: String,
    pub user: User,
    retweeted_status: Option<String>,
    target: String,
    fragment: Option<String>,
    blocked: bool,
    blocking: bool,
    topic_pic: String,
    topic_symbol: String,
    topic_title: String,
    topic_desc: String,
    donate_count: usize,
    donate_snowcoin: usize,
    view_count: usize,
    mark: usize,
    card: Option<String>,
    favorited: bool,
    favorited_created_at: Option<String>,
    #[serde(rename="timeBefore")]
    time_before: Option<String>,
    expend: bool,
    #[serde(rename="canEdit")]
    can_edit: bool,
    #[serde(rename="firstImg")]
    first_img: Option<String>,
    topic_pic_thumbnail_small: String,
    topic_pic_thumbnail: String,
    #[serde(rename="topic_pic_headOrPad")]
    topic_pic_head_or_pad: String,
    promotion_pic: Option<String>,
    promotion_url: Option<String>,
    text: String,
    source: String,
}
