use hyper::Client;
use hyper::client::IntoUrl;
use hyper::header::{Cookie, SetCookie, Connection};
use cookie::CookieJar;
use std::io::Error as IoError;
use url::ParseError;
use hyper::Error as HttpError;
use std::fmt::{Display, Formatter};
use std::fmt::Error as FmtError;
use std::io::Read;

pub struct SuperAgent<'a> {
    login_url: &'a str,
    client: Client,
    cookiejar: CookieJar<'a>,
    params: Option<Vec<(&'a str, &'a str)>>,
}

impl<'a> SuperAgent<'a> {
    pub fn new(url: &'a str) -> Result<SuperAgent<'a>, AgentError> {

        let client = Client::new();
        let c = CookieJar::new(b"Super8SecretAgent_");
        let mut agent = SuperAgent {
            login_url: url,
            client: client,
            cookiejar: c,
            params: None,
        };
        agent.initial_cookie();
        Ok(agent)
    }

    pub fn get<U: IntoUrl>(&mut self, url: U) -> Result<Response, AgentError> {
        let mut link = try!(url.into_url().map_err(AgentError::UrlParseError));
        if let Some(ref params) = self.params {
            link.set_query_from_pairs(params.to_vec().into_iter());
        }

        println!("{:?}", link);
        let mut resp = try!(self.client
                                .get(link)
                                .header(Connection::close())
                                .header(Cookie::from_cookie_jar(&self.cookiejar))
                                .send()
                                .map_err(AgentError::HttpRequestError));
        let mut resp_body = String::new();

        try!(resp.read_to_string(&mut resp_body).map_err(AgentError::HttpIoError));

        let response = Response {
            code: resp.status.to_u16(),
            status: resp.status,
            headers: resp.headers.clone(),
            body: resp_body,
        };

        self.params = None;

        return Ok(response);
    }

    pub fn get_with_params(&mut self,
                           url: &'a str,
                           url_params: &[(&'a str, &'a str)])
                           -> Result<Response, AgentError> {
        self.params = Some(url_params.iter().cloned().collect());

        return self.get(url);
    }

    pub fn add_param(mut self, key: &'a str, val: &'a str) -> SuperAgent<'a> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verify_get_with_params() {

        let mut agent = SuperAgent::new("http://xueqiu.com").unwrap();

        let r = agent.get("http://xueqiu.com/statuses/topic.json?simple_user=1&topicType=5&page=1")
                     .unwrap();
        let r2 = agent.get_with_params("http://xueqiu.com/statuses/topic.json",
                                       &[("simple_user", "1"), ("topicType", "5"), ("page", "1")])
                      .unwrap();
        assert_eq!(r.body, r2.body);
    }
}
