//! event is what happened in the world, Node may dont know about it, or recieve event with the latency, never get it
use crate::{clock::TimerId, message::Message, node::NodeId};

pub enum Event {
    MessageReceived {
        from: NodeId,
        msg: Message,
        rssi: i8,
    },
    TimerFired(TimerId),
}
