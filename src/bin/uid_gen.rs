use std::time::SystemTime;

use rand::random;
use rustengan::{Node, Payload, Service};
use serde::{Deserialize, Serialize};

struct State {
    message_count: u128,
}

#[derive(Deserialize)]
struct Request {}

#[derive(Serialize)]
struct Response {
    id: u128,
}

impl Default for State {
    fn default() -> Self {
        State {
            message_count: 0u128,
        }
    }
}

impl Service<State, Request, Response> for Node<State> {
    fn handle(
        &mut self,
        _request: Payload<Request>,
    ) -> anyhow::Result<rustengan::Payload<Response>> {
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let result = 
            31_u128.wrapping_pow(2).wrapping_mul(self.state.message_count)
            .wrapping_add(31_u128.wrapping_pow(1).wrapping_mul(time))
            .wrapping_add(random());
        let mut response = Payload::new("generate_ok".to_string(), Response { id: result });
        response.msg_id = Some(usize::try_from(self.state.message_count)?);
        self.state.message_count = self.state.message_count.wrapping_add(1);
        Ok(response)
    }
}

fn main() -> anyhow::Result<()> {
    let mut node = Node::<State>::new()?;
    node.run()?;
    Ok(())
}
