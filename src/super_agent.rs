use hyper::Client;
use hyper::client::IntoUrl;
use hyper::header::{Cookie, SetCookie, Connection};
use cookie::CookieJar;
use std::io::Error as IoError;
use url::{Url, ParseError};
use hyper::Error as HttpError;
use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;
use std::io::Read;

pub struct SuperAgent<'a> {
    login_url: &'a str,
    client: Client,
    cookiejar: CookieJar<'a>, /* target: Option<Url>,
                               * params: Option<Vec<(&'a str, &'a str)>>, */
}

impl<'a> SuperAgent<'a> {
    pub fn new(url: &'a str) -> SuperAgent<'a> {

        let client = Client::new();
        let c = CookieJar::new(b"Super8SecretAgent_");
        let mut agent = SuperAgent {
            login_url: url,
            client: client,
            cookiejar: c,
        };
        agent.initial_cookie();
        agent
    }

    pub fn get<U: IntoUrl>(&self, url: U) -> UrlBuilder {
        // self.target = Some(try!(url.into_url().map_err(AgentError::UrlParseError)));
        UrlBuilder {
            agent: self,
            url: url.into_url().ok(),
            params: None,
        }
    }

    pub fn get_with_params(&self, url: &'a str, url_params: &[(&'a str, &'a str)]) -> UrlBuilder {
        UrlBuilder {
            agent: self,
            url: url.into_url().ok(),
            params: Some(url_params.iter().cloned().collect()),
        }
    }
    pub fn initial_cookie(&mut self) {
        match self.client
                  .get(self.login_url)
                  .header(Connection::close())
                  .send() {
            Ok(res) => {
                res.headers.get::<SetCookie>().unwrap().apply_to_cookie_jar(&mut self.cookiejar);
            }
            Err(e) => println!("Unable get cookie, {:?}", e),
        }
    }
}

pub struct UrlBuilder<'a> {
    agent: &'a SuperAgent<'a>,
    url: Option<Url>,
    params: Option<Vec<(&'a str, &'a str)>>,
}
impl<'a> UrlBuilder<'a> {
    pub fn add_param(mut self, key: &'a str, val: &'a str) -> UrlBuilder<'a> {
        {
            let mut params = match self.params {
                Some(ref mut p) => p,
                None => {
                    self.params = Some(Vec::new());
                    self.params.as_mut().unwrap()
                }
            };
            params.push((key, val));
        }
        self
    }

    pub fn send(mut self) -> Result<Response, AgentError> {
        let mut link = match self.url {
            Some(ref mut r) => r,
            None => return Err(AgentError::MissingUrl),
        };

        if let Some(ref params) = self.params {
            link.set_query_from_pairs(params.to_vec().into_iter());
        }

        println!("{:?}", link);
        let mut resp = try!(self.agent
                                .client
                                .get(link.clone())
                                .header(Connection::close())
                                .header(Cookie::from_cookie_jar(&self.agent.cookiejar))
                                .send()
                                .map_err(AgentError::HttpRequestError));
        let mut resp_body = String::new();

        try!(resp.read_to_string(&mut resp_body).map_err(AgentError::HttpIoError));

        Ok(Response {
            code: resp.status.to_u16(),
            status: resp.status,
            headers: resp.headers.clone(),
            body: resp_body,
        })
    }
}


pub struct Response {
    pub code: u16,
    pub status: ::hyper::status::StatusCode,
    pub headers: ::hyper::header::Headers,
    pub body: String,
}

impl Display for Response {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), FmtError> {
        self.body.fmt(fmt)
    }
}

#[derive(Debug)]
pub enum AgentError {
    UrlParseError(ParseError),
    HttpRequestError(HttpError),
    HttpIoError(IoError),
    MissingUrl,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_get_with_params() {

        let mut agent = SuperAgent::new("http://xueqiu.com");

        let r = agent.get("http://xueqiu.com/statuses/topic.json?simple_user=1&topicType=5&page=1")
                     .send();
        let r2 = agent.get_with_params("http://xueqiu.com/statuses/topic.json",
                                       &[("simple_user", "1"), ("topicType", "5"), ("page", "1")])
                      .send();
        assert_eq!(r.unwrap().code, r2.unwrap().code);
    }
}
