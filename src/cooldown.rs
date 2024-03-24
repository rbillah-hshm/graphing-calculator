use macroquad::time::get_time;
use std::collections::HashMap;

trait ObjDefault<T> {
    fn default() -> T;
}
pub struct Object {
    duration: isize,
    old_time: isize,
    pub on: bool,
}
impl ObjDefault<Object> for Object {
    fn default() -> Object {
        Object {
            duration: 5,
            old_time: get_time().floor() as isize,
            on: false,
        }
    }
}
pub mod job {
    use super::*;
    pub fn add<'a>(storage: &mut HashMap<&'a str, Object>, tag: &'a str, duration: isize) {
        let object = Object {
            duration,
            ..Object::default()
        };
        storage.insert(tag, object);
    }
    pub fn is_on(storage: &HashMap<&str, Object>, tag: &str) -> bool {
        storage.get(tag).unwrap().on
    }
    pub fn remove(storage: &mut HashMap<&str, Object>, tag: &str) {
        storage.remove(tag);
    }
    pub fn update(storage: &mut HashMap<&str, Object>) {
        for (tag, object) in storage.iter_mut() {
            let record_time = get_time().floor() as isize;
            if ((record_time - object.duration) > object.old_time) {
                object.old_time = record_time;
                object.on = true;
            }
        }
    }
    pub fn update_next(storage: &mut HashMap<&str, Object>) {
        for (tag, object) in storage.iter_mut() {
            object.on = false;
        }
    }
}
