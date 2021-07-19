use std::path::Path;
use std::ptr;

use super::to_wstring;
use winapi::shared::minwindef::MAX_PATH;
use winapi::um::fileapi::GetVolumeInformationW;

pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    let root = dir.ancestors().last().unwrap(); // this unwrap is safe as .ancestors() always returns at least one value

    // Get the name of the partition system of the target device
    let mut system: Vec<u16> = Vec::with_capacity(MAX_PATH + 1);
    unsafe {
        GetVolumeInformationW(
            to_wstring(&root.to_str().unwrap()).as_ptr(),
            ptr::null_mut(),
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null_mut(),
            system.as_mut_ptr(),
            (MAX_PATH + 1) as u32,
        );

        system.set_len(MAX_PATH + 1);
        // remove any characters after the first null byte
        system.truncate(system.iter().position(|c| *c == 0).unwrap_or(system.len()));
    }

    let system = String::from_utf16_lossy(&system);
    info!("Detected partition type '{}' for device {:?}", system, root);

    // note that the current system only supports NTFS drives
    Ok(system == "NTFS")
}
