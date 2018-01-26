extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Message {
    Request(RequestKind),
    Response(ResponseKind),
    Event(EventKind),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action")]
enum RequestKind {
    AgentsCreate(AgentsCreateReq),
    AgentsList(AgentsListReq),
    // TracksCreate(CreateTrackReq),
    // TracksList(ListTrackData),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action")]
enum ResponseKind {
    AgentsCreate(AgentsCreateResp),
    AgentsList(AgentsListResp),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "name")]
enum EventKind {
    AgentCreated,
    TrackCreated,
}



#[derive(Debug, Serialize, Deserialize)]
struct AgentsCreateReq {
    params: AgentsCreateReqData,
    cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentsCreateReqData {
    label: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentsListReq {
    cid: String,
}



#[derive(Debug, Serialize, Deserialize)]
struct AgentsCreateResp {
    result: AgentsCreateRespData,
    cid: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct AgentsCreateRespData {
    label: String
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentsListResp {
    result: Vec<AgentsListRespItem>,
    cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentsListRespItem {
    id: String,
    data: AgentsListRespData,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentsListRespData {
    label: String,
}




fn main() {
    agents_create_req();
    println!("");
    agents_list_req();
    println!("");

    agents_create_resp();
    println!("");
    agents_list_resp();
    println!("");

    agents_created_event();
    println!("");
}

fn agents_create_req() {
    let data = r#"{
        "type": "request",
        "action": "agents_create",
        "params": {
            "label": "Foo bar"
        },
        "cid": "qwerty"
    }"#;

    let msg: Message = serde_json::from_str(data).unwrap();
    println!("{:?}", msg);

    match msg {
        Message::Request(req) => {
            match req {
                RequestKind::AgentsCreate(req) => {
                    println!("{:?}", req.params);
                }
                _ => unimplemented!(),
            }
        },
        _ => unimplemented!(),
    }
}

fn agents_list_req() {
    let data = r#"{
        "type": "request",
        "action": "agents_list",
        "cid": "qwerty"
    }"#;

    let msg: Message = serde_json::from_str(data).unwrap();
    println!("{:?}", msg);

    match msg {
        Message::Request(req) => {
            match req {
                RequestKind::AgentsList(req) => {
                    println!("{:?}", req);
                }
                _ => unimplemented!(),
            }
        },
        _ => unimplemented!(),
    }
}

fn agents_create_resp() {
    let data = r#"{
        "type": "response",
        "action": "agents_create",
        "result": {
            "label": "Some"
        },
        "cid": "qwerty"
    }"#;

    let msg: Message = serde_json::from_str(data).unwrap();
    println!("{:?}", msg);

}

fn agents_list_resp() {
    let data = r#"{
        "type": "response",
        "action": "agents_list",
        "result": [
            {
                "id": "1",
                "data": {
                    "label": "Label 1"
                }
            },
            {
                "id": "2",
                "data": {
                    "label": "Label 2"
                }
            }
        ],
        "cid": "qwerty"
    }"#;

    let msg: Message = serde_json::from_str(data).unwrap();
    println!("{:?}", msg);
}

fn agents_created_event() {
    let data = r#"{
        "type": "event",
        "name": "agent_created"
    }"#;

    let msg: Message = serde_json::from_str(data).unwrap();
    println!("{:?}", msg);
}
