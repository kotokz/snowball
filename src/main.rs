#![feature(slice_patterns, plugin, custom_derive, custom_attribute)]
#![plugin(clippy,serde_macros)]
extern crate hyper;
extern crate cookie;
extern crate url;
extern crate serde;
extern crate serde_json;

mod types;
mod super_agent;
mod urlmapper;

use super_agent::SuperAgent;
// use serde_json::Value;
use types::{Topic, ShareHolderList};
use urlmapper::URLMapper::{SnowballPage, NewsTopicJson, StockShareholdersJson};

fn main() {

    let agent = SuperAgent::new(SnowballPage.to_str());
    let r = agent.get_with_params(NewsTopicJson,
                                  &[("simple_user", "1"), ("topicType", "5"), ("page", "1")])
                 .send()
                 .unwrap();

    println!("{}", r.code);

    let r = agent.get(NewsTopicJson)
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

    let sh = agent.get(StockShareholdersJson)
                  .add_param("symbol", "SZ300178")
                  .add_param("page", "1")
                  .add_param("size", "500")
                  .send()
                  .unwrap();

    let sh_map: ShareHolderList = serde_json::from_str(&sh.body).unwrap();
    println!("{}, {}",
             sh_map.list[1].totalshamt.unwrap(),
             sh_map.list[1].holdproportionpacc.unwrap());
}
