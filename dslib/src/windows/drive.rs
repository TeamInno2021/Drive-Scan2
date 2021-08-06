use super::OsError;
use std::path::PathBuf;
use std::ptr;

use super::winapi::{get_last_error, read_file, Handle, PtrCast, WinapiExt};
use winapi::um::fileapi::{CreateFileA, OPEN_EXISTING};
use winapi::um::winbase::GetVolumeNameForVolumeMountPointA;
use winapi::um::winnt::{FILE_SHARE_READ, FILE_SHARE_WRITE, GENERIC_READ};

/// NTFS OEM ID
const NTFS: u64 = 0x202020205346544E;

/// NTFS boot sector information
#[repr(packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct BootSector {
    _alignment: [u8; 3],
    signature: u64,
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    _reserved: [u8; 26],
    pub total_sectors: u64,
    pub mft_cluster: u64,
    pub mftmirr_cluster: u64,
    pub clusters_per_mft_record: u32,
    pub clusters_per_index_record: u32,
}

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

        // Read drive bootsector
        let boot = unsafe {
            let data = read_file(&handle, 512, None)?;
            let boot: BootSector = *PtrCast::cast(data.as_ptr());

            // Ensure this is actually an NTFS drive by comparing the metadata to the expected signature
            if boot.signature != NTFS {
                Err(OsError::from(
                    "attemped to scan non-ntfs drive as if it were ntfs, this is likely a bug",
                ))
            } else {
                Ok(boot)
            }
        }?;

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
