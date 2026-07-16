//! command it is what Node trigger
use crate::{
    clock::{Duration, TimerId},
    message::{Addr, Message, Payload},
    node::NodeId,
};

pub enum Command {
    Transmit {
        to: Addr,
        msg: Message,
    },
    StartTimer {
        id: TimerId,
        after: Duration,
    },
    CancelTimer(TimerId),
    Deliver {
        src: NodeId,
        payload: Payload,
    },
}
