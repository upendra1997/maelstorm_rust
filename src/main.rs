use rustengan::{Node, Payload, Service};
use serde::{Deserialize, Serialize};

#[derive(Default)]
struct Echo;
#[derive(Deserialize, Serialize)]
struct EchoPayload {
    echo: String,
}

impl Service<Echo, EchoPayload, EchoPayload> for Node<Echo> {
    fn handle(request: Payload<EchoPayload>) -> Payload<EchoPayload> {
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
    let node = Node::<Echo>::new()?;
    node.run()?;
    Ok(())
}
