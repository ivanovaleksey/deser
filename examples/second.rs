extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action")]
enum Message {
    CreateAgent(CreateAgentReq),
    CreateTrack(CreateTrackReq),
    // ListAgent(ListAgentData),
    // ListTrack(ListTrackData),
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateAgentReq {
    data: Option<CreateAgentData>,
    cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateAgentData {
    label: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateTrackReq {
    data: CreateTrackData,
    cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateTrackData {
    id: String
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
        "data": {
            "label": "Foo bar"
        },
        "cid": "qwerty"
    }"#;

    let msg: Message = serde_json::from_str(some_data).unwrap();
    println!("{:?}", msg);

    match msg {
        Message::CreateAgent(req) => {
            println!("{:?}", req.data);
        },
        _ => unreachable!(),
    }
}
