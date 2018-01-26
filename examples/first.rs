extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "type")]
enum Message {
    Request {
        action: String,
        data: Option<serde_json::Value>,
        cid: String,
    },
    // Response {  },
    // Event {  },
}

fn main() {
    let none_data = r#"{
        "type": "request",
        "action": "create",
        "cid": "qwerty"
    }"#;

    let msg: Message = serde_json::from_str(none_data).unwrap();
    println!("{:?}", msg);

    let some_data = r#"{
        "type": "request",
        "action": "create",
        "data": {
            "label": "Foo bar"
        },
        "cid": "qwerty"
    }"#;

    let msg: Message = serde_json::from_str(some_data).unwrap();
    println!("{:?}", msg);
}
