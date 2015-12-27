use hyper::client::IntoUrl;
use url::Url;
use url::ParseError as UrlError;


pub enum URLMapper {
    SnowballPage,
    NewsTopicJson,
    StockShareholdersJson,
    StockSelectorJson,
}

impl ToString for URLMapper {
    fn to_string(&self) -> String {
        match *self {
            URLMapper::SnowballPage => "http://xueqiu.com".to_owned(),
            URLMapper::NewsTopicJson => "http://xueqiu.com/statuses/topic.json".to_owned(),
            URLMapper::StockShareholdersJson => {
                "http://xueqiu.com/stock/f10/shareholdernum.json".to_owned()
            }
            URLMapper::StockSelectorJson => {
                "http://xueqiu.com/stock/screener/screen.json".to_owned()
            }
        }
    }
}

impl IntoUrl for URLMapper {
    fn into_url(self) -> Result<Url, UrlError> {
        self.to_string().into_url()
    }
}
