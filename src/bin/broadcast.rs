use std::collections::HashMap;

use rustengan::{Node, Payload, Service};
use serde::{Deserialize, Serialize};

#[derive(Default)]
struct Broadcast {
    messages: Vec<usize>,
}

#[derive(Deserialize)]
struct BroadcastRequest {
    message: usize,
}

#[derive(Serialize)]
struct BroadcastResponse {}

impl Service<Broadcast, BroadcastRequest, BroadcastResponse> for Node<Broadcast> {
    fn handle(
        &mut self,
        request: Payload<BroadcastRequest>,
    ) -> anyhow::Result<rustengan::Payload<BroadcastResponse>> {
        self.state.messages.push(request.info.message);
        Ok(Payload::new(
            "broadcast_ok".to_string(),
            BroadcastResponse {},
        ))
    }
}

#[derive(Deserialize)]
struct ReadRequest {}

#[derive(Serialize)]
struct ReadResponse {
    messages: Vec<usize>,
}

impl Service<Broadcast, ReadRequest, ReadResponse> for Node<Broadcast> {
    fn handle(
        &mut self,
        request: Payload<ReadRequest>,
    ) -> anyhow::Result<rustengan::Payload<ReadResponse>> {
        let response = ReadResponse { messages: self.state.messages.clone() };
        Ok(Payload::new("topology_ok".to_string(), response))
    }
}

#[derive(Deserialize)]
struct TopologyRequest {
    #[serde(flatten)]
    map: HashMap<String, Vec<String>>,
}

#[derive(Serialize)]
struct TopologyResponse {}

impl Service<Broadcast, TopologyRequest, TopologyResponse> for Node<Broadcast> {
    fn handle(
        &mut self,
        _request: Payload<TopologyRequest>,
    ) -> anyhow::Result<rustengan::Payload<TopologyResponse>> {
        Ok(Payload::new("topology_ok".to_string(), TopologyResponse {}))
    }
}
fn main() -> anyhow::Result<()> {
    let mut node = Node::<Broadcast>::new()?;
    node.run()?;
    Ok(())
}
