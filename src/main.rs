use std::io::{stdin, stdout, Write};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use serde_json::{de, ser};

#[derive(Serialize, Deserialize, Debug)]
struct Payload<ExtraInfo> {
    #[serde(rename = "type")]
    type_payload: String,
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    extra_info: ExtraInfo,
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
struct State<ExtraState: Default> {
    id: String,
    node_ids: Vec<String>,
    extra_state: ExtraState,
}

impl<a> State<a>
where
    a: Default,
{
    fn new() -> anyhow::Result<State<a>> {
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
                Ok(State {
                    id: init_mesage.body.extra_info.node_id,
                    node_ids: init_mesage.body.extra_info.node_ids,
                    extra_state: a::default(),
                })
            }
            Err(e) => {
                let error_response =
                    PayloadError::error(0, "error parsing init message".to_string());
                println!("{}", ser::to_string(&error_response)?);
                Err(anyhow!("error parsing input message"))
            }
        }
    }
}

trait Service<ExtraState, Request, Response>
where
    ExtraState: Default,
    Request: for<'a> Deserialize<'a>,
    Response: Serialize,
{
    fn step(request: Payload<Request>) -> Payload<Response>;
    fn start(node: State<ExtraState>) -> anyhow::Result<()>
    where
        Self: Sized,
    {
        loop {
            let mut input = String::new();
            stdin().read_line(&mut input);
            let request = de::from_str::<Message<Payload<Request>>>(&input)?;
            let mut response = Self::step(request.body);
            let message = Message {
                src: node.id.to_string(),
                dest: request.src,
                body: response,
            };
            println!("{}", ser::to_string(&message)?);
            // eprintln!("Error Reading from stdin");
        }
    }
}

#[derive(Default)]
struct Echo;
#[derive(Deserialize, Serialize)]
struct EchoPayload {
    echo: String,
}

impl Service<Echo, EchoPayload, EchoPayload> for State<Echo> {
    fn step(request: Payload<EchoPayload>) -> Payload<EchoPayload> {
        Payload {
            type_payload: "echo_ok".to_string(),
            msg_id: request.msg_id,
            in_reply_to: request.msg_id,
            extra_info: EchoPayload {
                echo: request.extra_info.echo,
            },
        }
    }
}

fn main() -> anyhow::Result<()> {
    let node = State::<Echo>::new()?;
    <State<Echo> as Service<Echo, EchoPayload, EchoPayload>>::start(node);
    Ok(())
}
