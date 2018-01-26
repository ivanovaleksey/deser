extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Message {
    AgentsCreateRequest(AgentsCreateRequest),
    AgentsCreateResponse(AgentsCreateResponse),
    AgentsCreatedEvent(AgentsCreatedEvent),

    AgentsListRequest(AgentsListRequest),
    AgentsListResponse(AgentsListResponse),
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentsCreateRequest {
    params: AgentsCreateRequestParams,
    cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentsCreateRequestParams {
    label: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentsListRequest {
    cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentsCreateResponse {
    result: AgentsCreateResponseResult,
    cid: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentsCreateResponseResult {
    label: String
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentsListResponse {
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

#[derive(Debug, Serialize, Deserialize)]
struct AgentsCreatedEvent {
    result: AgentsCreatedEventResult
}

#[derive(Debug, Serialize, Deserialize)]
struct AgentsCreatedEventResult {
    agent_id: String
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
        "type": "AgentsCreateRequest",
        "params": {
            "label": "Foo bar"
        },
        "cid": "qwerty"
    }"#;

    let msg: Message = serde_json::from_str(data).unwrap();
    match msg {
        Message::AgentsCreateRequest(req) => {
            println!("{:?}", req);
        },
        _ => unimplemented!(),
    }
}

fn agents_list_req() {
    let data = r#"{
        "type": "AgentsListRequest",
        "cid": "qwerty"
    }"#;

    let msg: Message = serde_json::from_str(data).unwrap();
    match msg {
        Message::AgentsListRequest(req) => {
            println!("{:?}", req);
        },
        _ => unimplemented!(),
    }
}

fn agents_create_resp() {
    let data = r#"{
        "type": "AgentsCreateResponse",
        "result": {
            "label": "Some"
        },
        "cid": "qwerty"
    }"#;

    let msg: Message = serde_json::from_str(data).unwrap();
    match msg {
        Message::AgentsCreateResponse(resp) => {
            println!("{:?}", resp);
        },
        _ => unimplemented!(),
    }

}

fn agents_list_resp() {
    let data = r#"{
        "type": "AgentsListResponse",
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
    match msg {
        Message::AgentsListResponse(resp) => {
            println!("{:?}", resp);
        },
        _ => unimplemented!(),
    }
}

fn agents_created_event() {
    let data = r#"{
        "type": "AgentsCreatedEvent",
        "result": {
            "agent_id": "123"
        }
    }"#;

    let msg: Message = serde_json::from_str(data).unwrap();
    match msg {
        Message::AgentsCreatedEvent(event) => {
            println!("{:?}", event.result);
        },
        _ => unimplemented!(),
    }
}
