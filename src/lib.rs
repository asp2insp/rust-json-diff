extern crate rustc_serialize;
use std::fmt;
use rustc_serialize::json;

enum Change {
    Remove(Vec<String>, json::Json),
    Add(Vec<String>, json::Json),
    Insert(Vec<String>, json::Json),
    Rename(Vec<String>, String),
    Replace(Vec<String>, json::Json),
}

impl fmt::Debug for Change {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Change::Remove(ref kp, ref j) => write!(f, "Remove()"),
            &Change::Add(ref kp, ref j) => write!(f, "Add()"),
            &Change::Insert(ref kp, ref j) => write!(f, "Insert()"),
            &Change::Rename(ref kp, ref j) => write!(f, "Rename()"),
            &Change::Replace(ref kp, ref j) => write!(f, "Replace()"),
        }
    }
}

fn json_diff(left: &json::Json, right: &json::Json) -> Vec<Change> {
    Vec::new()
}


#[test]
fn it_works() {
    let left = json::Json::from_str("{\"a\": 100, \"b\": [1, 2, 3]}").unwrap();
    let right = json::Json::from_str("{\"a\": 100, \"b\": [1, 2, 3, 4]}").unwrap();
    let changes = json_diff(&left, &right);
    assert_eq!(0, changes.len());
}
