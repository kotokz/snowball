use hyper::client::IntoUrl;
use url::Url;
use url::ParseError as UrlError;


pub enum URLMapper {
    SnowballPage,
    NewsTopicJson,
    StockShareholdersJson,
    StockSelectorJson,
}

impl URLMapper {
    pub fn to_str(&self) -> &'static str {
        match *self {
            URLMapper::SnowballPage => "http://xueqiu.com",
            URLMapper::NewsTopicJson => "http://xueqiu.com/statuses/topic.json",
            URLMapper::StockShareholdersJson => "http://xueqiu.com/stock/f10/shareholdernum.json",
            URLMapper::StockSelectorJson => "http://xueqiu.com/stock/screener/screen.json",
        }
    }
}

impl IntoUrl for URLMapper {
    fn into_url(self) -> Result<Url, UrlError> {
        self.to_str().into_url()
    }
}
