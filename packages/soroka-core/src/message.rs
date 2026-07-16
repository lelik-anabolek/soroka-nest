use crate::node::{NodeId, Rank};
use heapless::Vec;

pub const MAX_PAYLOAD: usize = 64; // depends on radio, sophisticated value

/// LenT = u8: length keeps in one byte (MAX_PAYLOAD <= 255),
/// что убирает usize-поле длины и его выравнивание
pub type Payload = Vec<u8, MAX_PAYLOAD, u8>;

/// IMPORTANT (wire): postcard ser/deser depends on order —
/// new options must be added at the end
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wire", derive(serde::Serialize, serde::Deserialize))]
pub enum Message {
    Beacon(Beacon),
    JoinRequest(Rank),
    JoinAccept,
    JoinReject,
    Detach,
    Data(Data),
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "wire", derive(serde::Serialize, serde::Deserialize))]
pub struct Beacon {
    pub rank: Rank,
    pub free_slots: u8,
    /// counter of freshness
    pub root_epoch: u16,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "wire", derive(serde::Serialize, serde::Deserialize))]
pub struct Data {
    pub src: NodeId, // origin sender (многохоповый)
    pub ttl: u8,     // time to live
    pub payload: Payload,
}

const _: () = assert!(core::mem::size_of::<Message>() <= 70);

pub enum Addr {
    Unicast(NodeId),
    Broadcast,
}

#[cfg(all(test, feature = "wire"))]
mod tests {
    use super::*;

    #[test]
    fn message_wire_roundtrip() {
        let mut payload = Payload::new();
        payload.extend_from_slice(&[0xDE, 0xAD, 0xBE]).unwrap();
        let msg = Message::Data(Data {
            src: 7,
            ttl: 3,
            payload,
        });

        let mut buf = [0u8; MAX_PAYLOAD + 8];
        let used = postcard::to_slice(&msg, &mut buf).unwrap();
        let back: Message = postcard::from_bytes(used).unwrap();

        assert_eq!(msg, back);
        // варинт с payload из 3 байт не должен занимать все 64+ байта на проводе
        assert!(used.len() < 10, "wire size: {}", used.len());
    }

    #[test]
    fn beacon_wire_roundtrip() {
        let msg = Message::Beacon(Beacon {
            rank: 2,
            free_slots: 5,
            root_epoch: 42,
        });

        let mut buf = [0u8; 16];
        let used = postcard::to_slice(&msg, &mut buf).unwrap();
        let back: Message = postcard::from_bytes(used).unwrap();

        assert_eq!(msg, back);
    }
}
