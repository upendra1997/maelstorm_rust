use rustengan::{Node, Payload, Service};
use serde::{Deserialize, Serialize};

#[derive(Default)]
struct Echo;
#[derive(Deserialize, Serialize)]
struct EchoPayload {
    echo: String,
}

impl Service<Echo, EchoPayload, EchoPayload> for Node<Echo> {
    fn handle(&mut self, request: Payload<EchoPayload>) -> anyhow::Result<Payload<EchoPayload>> {
        let mut response = Payload::new("echo_ok".to_string(), request.info);
        response.msg_id = request.msg_id;
        Ok(response)
    }
}

fn main() -> anyhow::Result<()> {
    let mut node = Node::<Echo>::new()?;
    node.run()?;
    Ok(())
}
