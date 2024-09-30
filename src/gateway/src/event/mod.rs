pub mod checker;
pub mod parser;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub express: String,
    pub target_value: String,
    pub topic: String,
    pub action_key: String,
    pub target_dest: i32,
    pub life_cycle: i32, // '1' means one time
}

lazy_static! {
    static ref EVENT_MAP: Mutex<HashMap<String, Event>> = Mutex::new(HashMap::new());
}

pub fn get(key: &str) -> Option<Event> {
    let event_map = EVENT_MAP.lock().unwrap();
    event_map.get(key).map(|result| Event {
        name: result.name.clone(),
        express: result.express.clone(),
        target_value: result.target_value.clone(),
        topic: result.topic.clone(),
        action_key: result.action_key.clone(),
        target_dest: result.target_dest,
        life_cycle: result.life_cycle,
    })
}

pub fn insert(key: &str, e: Event) -> Option<Event> {
    EVENT_MAP.lock().unwrap().insert(key.to_string(), e)
}

pub fn remove(key: &str) -> Option<Event> {
    EVENT_MAP.lock().unwrap().remove(key)
}
