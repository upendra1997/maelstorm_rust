use rustengan::{Node, Payload, Request};
use serde::{Deserialize, Serialize};

#[derive(Default)]
struct Echo;
#[derive(Deserialize, Serialize)]
struct EchoPayload {
    echo: String,
}

impl Request for EchoPayload {
    type Response = EchoPayload;
}

fn main() -> anyhow::Result<()> {
    let mut node = Node::<Echo, EchoPayload>::new()?;
    node.add_handler(&|_node, request| {
        let mut response = Payload::new("echo_ok".to_string(), request.info);
        response.msg_id = request.msg_id;
        Ok(response)
    });
    node.run()?;
    Ok(())
}
