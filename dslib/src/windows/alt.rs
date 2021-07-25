use super::OsError;
use crate::File;

use std::path::PathBuf;

/// An alternate scanner which functions on any drive and does not need elevated permissions.
///
/// Note that this is several times slower than the primary scanner
pub fn scan(dir: PathBuf) -> Result<File, OsError> {
    todo!();
}
