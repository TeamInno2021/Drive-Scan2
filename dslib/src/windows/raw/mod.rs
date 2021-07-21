//! raw c-like structures for directly interfacing with memory blocks

use super::{winapi, OsError};

pub mod boot;
pub mod mft;

pub use boot::BootSector;
