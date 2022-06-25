use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    age: i32,
}

fn main() {
    println!("Hello JSON");

    let s = r#"
        {"name": "李文举", "age": 22}
    "#.to_string();

    let u = User {
        name: "ha".to_owned(),
        age: 19,
    };

    let v: User = serde_json::from_str(&s).unwrap();
    let s = serde_json::to_string(&u).unwrap();

    println!("{:?}", v);
    println!("{}", s);
}