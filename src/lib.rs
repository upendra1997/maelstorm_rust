use std::{cell::RefCell, io::stdin};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::{de, ser};

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload<Info> {
    #[serde(rename = "type")]
    pub type_payload: String,
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub info: Info,
}

impl<Info> Payload<Info> {
    pub fn new(type_payload: String, info: Info) -> Self {
        Payload {
            type_payload,
            msg_id: None,
            in_reply_to: None,
            info,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Message<Payload> {
    src: String,
    dest: String,
    body: Payload,
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
        let error = PayloadError { code, text };
        Payload::new("error".to_string(), error)
    }
}

pub trait Request {
    type Response;
}

pub struct Node<'a, State: Default, Req: Request> {
    pub id: String,
    pub node_ids: Vec<String>,
    pub state: RefCell<State>,
    handlers:
        Vec<&'a (dyn for<'b> Fn(&'b Self, Payload<Req>) -> anyhow::Result<Payload<Req::Response>>)>,
}

impl<State, Req> Node<'_, State, Req>
where
    State: Default,
    Req: Request,
{
    pub fn new() -> anyhow::Result<Node<'static, State, Req>> {
        let mut input = String::new();
        stdin().read_line(&mut input)?;
        match de::from_str::<Message<Payload<InitRequest>>>(&input) {
            Ok(init_mesage) => {
                let ack_response = Message {
                    src: init_mesage.dest,
                    dest: init_mesage.src,
                    body: Payload {
                        type_payload: "init_ok".to_string(),
                        msg_id: init_mesage.body.msg_id,
                        in_reply_to: init_mesage.body.msg_id,
                        info: (),
                    },
                };
                println!("{}", ser::to_string(&ack_response)?);
                Ok(Node {
                    id: init_mesage.body.info.node_id,
                    node_ids: init_mesage.body.info.node_ids,
                    state: RefCell::new(State::default()),
                    handlers: Vec::new(),
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

    pub fn run(&mut self) -> anyhow::Result<()>
    where
        Req: for<'a> Deserialize<'a>,
        Req::Response: Serialize,
        // Self: Service<State, Req, Req::Response>,
    {
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            for handler in &self.handlers {
                let request = de::from_str::<Message<Payload<Req>>>(&input);
                if let Err(e) = request {
                    eprintln!("unable to process request {} with error {}", input, e);
                    continue;
                }
                let request = request.unwrap();
                let msg_id = request.body.msg_id;
                match handler(self, request.body) {
                    Ok(mut response) => {
                        response.in_reply_to = msg_id;
                        let message = Message {
                            src: self.id.to_string(),
                            dest: request.src,
                            body: response,
                        };
                        println!("{}", ser::to_string(&message)?);
                    }
                    Err(e) => {
                        let error_response =
                            PayloadError::error(1, format!("error processing message: {}", e));
                        println!("{}", ser::to_string(&error_response)?);
                    }
                }
            }
        }
    }

    pub fn add_handler(
        &mut self,
        handler: &'static dyn for<'b> Fn(
            &'b Node<'_, State, Req>,
            Payload<Req>,
        ) -> anyhow::Result<Payload<Req::Response>>,
    ) {
        self.handlers.push(handler);
    }
}

pub trait Service<ExtraState, Request, Response>
where
    ExtraState: Default,
    Request: for<'a> Deserialize<'a>,
    Response: Serialize,
{
    fn handle(&mut self, request: Payload<Request>) -> anyhow::Result<Payload<Response>>;
}
