use crate::{clock::Instant, command::Command, message::Beacon};
use heapless::{Vec, index_map::FnvIndexMap};

use crate::event::Event;

/// Unique identificator in the system
pub type NodeId = u16;

/// Distance to Root: 0 at the Root, the lower the closer
pub type Rank = u16;

#[derive(Debug, Default)]
pub struct NodeConfig {
    pub preferred_connections: usize,
    pub maximum_connections: usize,
    /// the higher value the more preferable for connection
    pub attach_priority: u16,
    pub seed: u32,
}

#[derive(Debug, PartialEq)]
pub enum NodeState {
    Detached,
    Joining {
        candidate: NodeId,
        attempts: u8,
    },
    Attached {
        parent: NodeId,
        rank: Rank,
        parent_last_seen: Instant,
    },
}

const _: () = assert!(core::mem::size_of::<NodeState>() <= 12);

pub struct NeighborInfo {
    beacon: Beacon,
    rssi: i8,
    last_seen: Instant,
}
pub struct Node<const MAX_CHILDREN: usize> {
    id: NodeId,
    config: NodeConfig,
    children: Vec<NodeId, MAX_CHILDREN>,
    neighbors: FnvIndexMap<NodeId, NeighborInfo, 16>,
    state: NodeState,
    is_root: bool,
}

impl<const MAX_CHILDREN: usize> Node<MAX_CHILDREN> {
    pub fn new(id: NodeId, config: NodeConfig, is_root: bool) -> Self {
        debug_assert!(config.maximum_connections <= MAX_CHILDREN);

        Self {
            id,
            config,
            children: heapless::Vec::<NodeId, MAX_CHILDREN>::new(),
            state: NodeState::Detached,
            neighbors: FnvIndexMap::<NodeId, NeighborInfo, 16>::new(),
            is_root: is_root,
        }
    }

    /// Current distance to Root: 0 at the Root itself, None while detached
    pub fn rank(&self) -> Option<Rank> {
        if self.is_root {
            return Some(0);
        }
        match self.state {
            NodeState::Attached { rank, .. } => Some(rank),
            _ => None,
        }
    }

    pub fn parent(&self) -> Option<NodeId> {
        match self.state {
            NodeState::Attached { parent, .. } => Some(parent),
            _ => None,
        }
    }

    pub fn handle(&mut self, event: Event, now: Instant, emit: &mut impl FnMut(Command)) {
        match event {
            _ => todo!(),
        }
    }
}
