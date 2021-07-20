//! raw c-like structures for directly interfacing with memory blocks

use super::{winapi, OsError};

mod boot;

pub use boot::BootSector;
