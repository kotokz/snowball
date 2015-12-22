extern crate hyper;
extern crate cookie;
extern crate url;

mod super_agent;

use super_agent::SuperAgent;

fn main() {

    let mut agent = SuperAgent::new("http://xueqiu.com").unwrap();
    let r = agent.get_with_params("http://xueqiu.com/statuses/topic.json",
                                  &[("simple_user", "1"), ("topicType", "5"), ("page", "1")])
                 .unwrap();

    println!("{}", r.body);

    let r = agent.add_param("simple_user", "1")
                 .add_param("topicType", "5")
                 .add_param("page", "1")
                 .get("http://xueqiu.com/statuses/topic.json")
                 .unwrap();
    println!("{}", r.code);
}
