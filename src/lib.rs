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

impl Change {
    fn write(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Change::Remove(ref kp, ref j) => write!(f, "Remove({})", j),
            &Change::Add(ref kp, ref j) => write!(f, "Add({})", j),
            &Change::Insert(ref kp, ref j) => write!(f, "Insert({})", j),
            &Change::Rename(ref kp, ref j) => write!(f, "Rename({})", j),
            &Change::Replace(ref kp, ref j) => write!(f, "Replace({})", j),
        }
    }
}

impl fmt::Debug for Change {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write(f)
    }
}

fn json_diff(left: &json::Json, right: &json::Json) -> Vec<Change> {
    vec!(Change::Remove(Vec::new(), json::Json::Null))
}


#[test]
fn it_works() {
    let left = json::Json::from_str("{\"a\": 100, \"b\": [1, 2, 3]}").unwrap();
    let right = json::Json::from_str("{\"a\": 100, \"b\": [1, 2, 3, 4]}").unwrap();
    let changes = json_diff(&left, &right);
    assert_eq!("[Remove(null)]", format!("{:?}", changes));
}
