use serde::*;
use serde_json::Value;

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
struct Card {
    param: Option<String>,
    data: Option<String>,
    #[serde(rename="type")]
    card_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topic {
    id: Option<usize>,
    user_id: Option<i64>,
    title: String,
    created_at: usize,
    retweet_count: Option<usize>,
    reply_count: usize,
    fav_count: usize,
    truncated: bool,
    #[serde(rename="commentId")]
    comment_id: Option<usize>,
    retweet_status_id: Option<usize>,
    symbol_id: Option<String>,
    pub description: String,
    #[serde(rename="type")]
    topic_type: Option<String>,
    source_link: Option<String>,
    edited_at: Option<usize>,
    pic: Option<String>,
    pub user: User,
    retweeted_status: Option<Value>,
    target: Option<String>,
    fragment: Option<String>,
    blocked: bool,
    blocking: bool,
    topic_pic: Option<String>,
    topic_symbol: Option<String>,
    topic_title: Option<String>,
    topic_desc: Option<String>,
    donate_count: usize,
    donate_snowcoin: usize,
    view_count: usize,
    mark: usize,
    card: Option<Card>,
    favorited: bool,
    favorited_created_at: Option<String>,
    #[serde(rename="timeBefore")]
    time_before: Option<String>,
    expend: bool,
    #[serde(rename="canEdit")]
    can_edit: bool,
    #[serde(rename="firstImg")]
    first_img: Option<String>,
    topic_pic_thumbnail_small: Option<String>,
    topic_pic_thumbnail: Option<String>,
    #[serde(rename="topic_pic_headOrPad")]
    topic_pic_head_or_pad: Option<String>,
    promotion_pic: Option<String>,
    promotion_url: Option<String>,
    text: String,
    source: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareHolderList {
    pub list: Vec<ShareHolder>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShareHolder {
    aavgholdsumgrhalfyear: Option<f64>,
    aavgholdsumgrq: Option<f64>,
    afavgholdsum: Option<f64>,
    afavgholdsumgrhalfyear: Option<f64>,
    afavgholdsumgrq: Option<f64>,
    afholdproportionpacc: Option<f64>,
    afproportionchg: Option<f64>,
    afproportiongrhalfyear: Option<f64>,
    afproportiongrq: Option<f64>,
    aholdproportionpacc: Option<f64>,
    aproportionchg: Option<f64>,
    aproportiongrhalfyear: Option<f64>,
    aproportiongrq: Option<f64>,
    askavgsh: Option<f64>,
    askshamt: Option<f64>,
    askshrto: Option<f64>,
    /// A股户均持股数半年增长率
    pub avgholdsumgrhalfyear: Option<f64>,
    /// A股户均持股数季度增长率
    pub avgholdsumgrq: Option<f64>,
    bavgholdsumgrhalfyear: Option<f64>,
    bavgholdsumgrq: Option<f64>,
    bholdproportionpacc: Option<f64>,
    bproportionchg: Option<f64>,
    bproportiongrhalfyear: Option<f64>,
    bproportiongrq: Option<f64>,
    bskavgsh: Option<f64>,
    bskshamt: Option<f64>,
    bskshrto: Option<f64>,
    convshamt: Option<f64>,
    corpshamt: Option<f64>,
    /// 统计日期
    pub enddate: Option<String>,
    forecorpamt: Option<f64>,
    havgholdsumgrhalfyear: Option<f64>,
    havgholdsumgrq: Option<f64>,
    hholdproportionpacc: Option<f64>,
    /// 户均持股比例
    pub holdproportionpacc: Option<f64>,
    hproportionchg: Option<f64>,
    hproportiongrhalfyear: Option<f64>,
    hproportiongrq: Option<f64>,
    hskavgsh: Option<f64>,
    hskshamt: Option<f64>,
    hskshrto: Option<f64>,
    indishamt: Option<f64>,
    othershamt: Option<f64>,
    pdomorpshamt: Option<f64>,
    pforecorpshamt: Option<f64>,
    /// 户均持股比例环比变化
    pub proportionchg: Option<f64>,
    /// 户均持股比例半年增长率
    pub proportiongrhalfyear: Option<f64>,
    /// 户均持股比例季度增长率
    pub proportiongrq: Option<f64>,
    pstatecorpshamt: Option<f64>,
    pstateshamt: Option<f64>,
    socicorpshamt: Option<f64>,
    staffshamt: Option<f64>,
    symbol: Option<f64>,
    /// 股东总户数
    pub totalshamt: Option<f64>,
    /// 股东总户数较上期增减
    pub totalshrto: Option<f64>,
}


#[cfg(test)]
mod tests {
    use super::*;
    use super_agent::SuperAgent;
    use urlmapper::URLMapper::*;

    #[test]
    fn verify_shareholderlist_type() {
        let agent = SuperAgent::new("http://xueqiu.com");

        let sh = agent.get(StockShareholdersJson)
                      .add_param("symbol", "SZ300178")
                      .add_param("page", "1")
                      .add_param("size", "500")
                      .send()
                      .unwrap();

        let sh_map: ShareHolderList = ::serde_json::from_str(&sh.body).unwrap();
        assert!(sh_map.list[1].totalshamt.is_some());
        assert!(sh_map.list[1].holdproportionpacc.is_some());
    }
}
