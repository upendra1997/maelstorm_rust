use std::io::stdin;

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::{de, ser};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload<ExtraInfo> {
    #[serde(rename = "type")]
    pub type_payload: String,
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub extra_info: ExtraInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message<Payload> {
    src: String,
    dest: String,
    body: Payload,
}

trait CreatePayload<ExtraInfo> {
    fn new() -> Payload<ExtraInfo>;
}

#[derive(Serialize, Deserialize, Debug)]
struct InitRequest {
    node_id: String,
    node_ids: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct PayloadError {
    code: usize,
    text: String,
}

impl PayloadError {
    fn error(code: usize, text: String) -> Payload<PayloadError> {
        Payload {
            type_payload: "error".to_string(),
            msg_id: None,
            in_reply_to: None,
            extra_info: PayloadError { code, text },
        }
    }
}

#[derive(Debug)]
pub struct Node<State: Default> {
    id: String,
    node_ids: Vec<String>,
    extra_state: State,
}

impl<State> Node<State>
where
    State: Default,
{
    pub fn new() -> anyhow::Result<Node<State>> {
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        match de::from_str::<Message<Payload<InitRequest>>>(&input) {
            Ok(init_mesage) => {
                let ack_response = Message {
                    src: init_mesage.dest,
                    dest: init_mesage.src,
                    body: Payload {
                        type_payload: "init_ok".to_string(),
                        msg_id: None,
                        in_reply_to: init_mesage.body.msg_id,
                        extra_info: (),
                    },
                };
                println!("{}", ser::to_string(&ack_response)?);
                Ok(Node {
                    id: init_mesage.body.extra_info.node_id,
                    node_ids: init_mesage.body.extra_info.node_ids,
                    extra_state: State::default(),
                })
            }
            Err(e) => {
                let error_response =
                    PayloadError::error(0, format!("error parsing init message: {}", e));
                println!("{}", ser::to_string(&error_response)?);
                Err(anyhow!("error parsing input message"))
            }
        }
    }

    pub fn run<Request, Response>(self) -> anyhow::Result<()>
    where
        Request: for<'a> Deserialize<'a>,
        Response: Serialize,
        Self: Service<State, Request, Response>,
    {
        <Node<State> as Service<State, Request, Response>>::start(self)
    }
}

pub trait Service<ExtraState, Request, Response>
where
    ExtraState: Default,
    Request: for<'a> Deserialize<'a>,
    Response: Serialize,
{
    fn handle(request: Payload<Request>) -> Payload<Response>;
    fn start(node: Node<ExtraState>) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            let request = de::from_str::<Message<Payload<Request>>>(&input)?;
            let response = Self::handle(request.body);
            let message = Message {
                src: node.id.to_string(),
                dest: request.src,
                body: response,
            };
            println!("{}", ser::to_string(&message)?);
        }
    }
}
