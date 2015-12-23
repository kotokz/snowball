#![feature(slice_patterns, test, plugin)]
#![plugin(clippy)]
extern crate hyper;
extern crate cookie;
extern crate url;

mod super_agent;

use super_agent::SuperAgent;

fn main() {

    let agent = SuperAgent::new("http://xueqiu.com");
    let r = agent.get_with_params("http://xueqiu.com/statuses/topic.json",
                                  &[("simple_user", "1"), ("topicType", "5"), ("page", "1")])
                 .send()
                 .unwrap();

    println!("{}", r.body);

    let r = agent.get("http://xueqiu.com/statuses/topic.json")
                 .add_param("simple_user", "1")
                 .add_param("topicType", "5")
                 .add_param("page", "1")
                 .send()
                 .unwrap();
    println!("{}", r.code);
}
