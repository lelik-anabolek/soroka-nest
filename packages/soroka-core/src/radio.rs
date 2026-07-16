use crate::message::Addr;

pub trait Radio {
    type Error;

    fn send(&mut self, to: Addr, frame: &[u8]) -> Result<(), Self::Error>;
}
