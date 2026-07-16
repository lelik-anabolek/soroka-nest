pub type Instant = u32;
pub type Duration = u32;
pub trait Clock {
    /// Monotonic ticks since boot.
    fn now(&self) -> Instant;
    fn reset(&mut self);
}

/// Logical timers of Node
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TimerId {
    Beacon,
    ParentWatch,
    JoinRetry,
    Housekeeping,
    SyncWithRoot,
}
