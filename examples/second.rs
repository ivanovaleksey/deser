extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action")]
enum Message {
    CreateAgent(CreateAgentData),
    // CreateTrack(CreateTrackData),
    // ListAgent(ListAgentData),
    // ListTrack(ListTrackData),
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateAgentData {
    label: Option<String>,
    cid: String,
}

fn main() {
    let none_data = r#"{
        "action": "create_agent",
        "cid": "qwerty"
    }"#;

    let msg: Message = serde_json::from_str(none_data).unwrap();
    println!("{:?}", msg);

    let some_data = r#"{
        "action": "create_agent",
        "label": "Foo bar",
        "cid": "qwerty"
    }"#;

    let msg: Message = serde_json::from_str(some_data).unwrap();
    println!("{:?}", msg);
}
