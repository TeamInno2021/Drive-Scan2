use std::path::Path;

use winapi::um::fileapi::GetVolumeInformationA;

pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    todo!();
}
