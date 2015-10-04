use std::cmp::Ordering;
use uuid::Uuid;

pub struct JsType {
    uid: u64,
    thing: Box<JsT>,
}

impl JsType {
    fn new(thing: Box<JsT>) -> JsType {
    }
}

impl PartialEq for JsType {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Eq for JsType{}

impl PartialOrd for JsType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.uid.cmp(&other.uid))
    }
}

impl Ord for JsType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.uid.cmp(&other.uid)
    }
}

pub trait JsT {}
