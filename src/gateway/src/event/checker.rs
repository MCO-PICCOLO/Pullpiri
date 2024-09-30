use crate::event;

pub fn check(name: &str, value: &str) -> bool {
    let e = event::get(name).unwrap();

    e.target_value == value
}
