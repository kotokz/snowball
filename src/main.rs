#![feature(slice_patterns, test, plugin, custom_derive, custom_attribute)]
#![plugin(clippy,serde_macros)]
extern crate hyper;
extern crate cookie;
extern crate url;
extern crate serde;
extern crate serde_json;

mod types;
mod super_agent;

use super_agent::SuperAgent;
use types::Topic;

fn main() {

    let agent = SuperAgent::new("http://xueqiu.com");
    let r = agent.get_with_params("http://xueqiu.com/statuses/topic.json",
                                  &[("simple_user", "1"), ("topicType", "5"), ("page", "1")])
                 .send()
                 .unwrap();

    println!("{}", r.code);

    let r = agent.get("http://xueqiu.com/statuses/topic.json")
                 .add_param("simple_user", "1")
                 .add_param("topicType", "5")
                 .add_param("page", "1")
                 .send()
                 .unwrap();
    println!("{}", r.code);

    // let map: Vec<Value> = serde_json::from_str(&r.body).unwrap();
    // for (key, value) in map[1].as_object().unwrap().iter() {
    //     println!("{}: {}", key, match *value {
    //         Value::U64(_) => format!("u64,"),
    //         Value::String(_) => format!("String,"),
    //         Value::Bool(_)  => format!("bool,"),
    //         _ => format!("{:?} other", value)
    //     });
    // }
    let map = serde_json::from_str::<Vec<Topic>>(&r.body).unwrap();
    for v in map {
        println!("{}: {}", v.user.screen_name.unwrap(), v.description);
    }
}
