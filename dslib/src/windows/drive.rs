use super::boot::BootSector;
use super::OsError;
use std::path::PathBuf;
use std::ptr;

use super::winapi::{get_last_error, Handle, WinapiExt};
use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING};
use winapi::um::winbase::GetVolumeNameForVolumeMountPointA;
use winapi::um::winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ};

#[derive(Debug)]
pub struct DriveInfo {
    /// The directory to scan
    pub path: PathBuf,
    /// A handle to the target volume
    pub handle: Handle,
    /// Low level bootsector information
    pub boot: BootSector,
    // Extra cluster information for parsing mft records
    pub bytes_per_mft_record: u64,
}

impl DriveInfo {
    pub fn parse(path: PathBuf) -> Result<Self, OsError> {
        // Get drive root
        let root = path.ancestors().last().unwrap().to_string_lossy(); // this unwrap is safe as .ancestors() always returns at least one value

        // Get drive volume
        let volume: String = WinapiExt::call(
            50,
            |size, volume| unsafe {
                GetVolumeNameForVolumeMountPointA(
                    root.to_string().as_ptr() as *const i8,
                    volume as *mut i8,
                    size as u32,
                )
            },
            |status| status != 0,
            true,
        )?;

        debug!("Targeting volume: {:?}", volume);

        // Remove the trailing backslashes from the volume name
        // Because for some reason CreateFile does not accept them
        let mut volume = volume.trim_end_matches('\\').to_string();

        // Append a null byte to terminate the string
        volume.push('\0');

        // Get drive handle
        let handle: Handle = unsafe {
            // Note that we aren't actually creating a file,
            // just opening a handle to the device
            CreateFileA(
                volume.as_ptr() as *const i8,
                GENERIC_READ,
                FILE_SHARE_READ | FILE_SHARE_WRITE,
                ptr::null_mut(),
                OPEN_EXISTING,
                0,
                ptr::null_mut(),
            )
            .into()
        };

        if !handle.is_valid() {
            return Err(get_last_error().into());
        }

        // Get drive bootsector
        let boot = unsafe { BootSector::read_from(&handle)? };

        // Calculate the number of bytes per mft record
        let bytes_per_mft_record = if boot.clusters_per_mft_record >= 128 {
            1 << (256 - boot.clusters_per_mft_record as u8 as u16) as u8
        } else {
            (boot.clusters_per_mft_record
                * boot.bytes_per_sector as u32
                * boot.sectors_per_cluster as u32) as u64
        };

        Ok(DriveInfo {
            path,
            handle,
            boot,
            bytes_per_mft_record,
        })
    }
}
