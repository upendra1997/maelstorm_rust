use std::time::SystemTime;

use rand::random;
use rustengan::{Node, Payload, Request};
use serde::{Deserialize, Serialize};

struct State {
    message_count: u128,
}

#[derive(Deserialize)]
struct Req {}

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

impl Request for Req {
    type Response = Response;
}

fn main() -> anyhow::Result<()> {
    let mut node = Node::<State, Req>::new()?;
    node.add_handler(&|node, _req| {
        let mut state = node.state.borrow_mut();
        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let result = 31_u128
            .wrapping_pow(2)
            .wrapping_mul(state.message_count)
            .wrapping_add(31_u128.wrapping_pow(1).wrapping_mul(time))
            .wrapping_add(random());
        let mut response = Payload::new("generate_ok".to_string(), Response { id: result });
        response.msg_id = Some(usize::try_from(state.message_count)?);
        state.message_count = state.message_count.wrapping_add(1);
        Ok(response)
    });
    node.run()?;
    Ok(())
}
