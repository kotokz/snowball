extern crate hyper;
extern crate cookie;
extern crate url;

mod super_agent;

use super_agent::SuperAgent;

fn main() {

    let agent = SuperAgent::new("http://xueqiu.com").unwrap();

    // let r = agent.get("http://xueqiu.com/statuses/topic.json?simple_user=1&topicType=5&page=1")
    //             .unwrap();

    // println!("{}", r.body);

    let r = agent.get_with_params("http://xueqiu.com/statuses/topic.json",
                                  Some(&[("simple_user", "1"), ("topicType", "5"), ("page", "1")]))
                 .unwrap();

    println!("{}", r.body);
}
