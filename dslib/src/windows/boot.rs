use super::winapi::{read_file, PtrCast};
use super::OsError;
use std::ffi::c_void;

/// NTFS OEM ID
const NTFS: u64 = 0x202020205346544E;

/// NTFS boot sector information
/// See <http://ntfs.com/ntfs-partition-boot-sector.htm> for extra information
#[derive(Debug, Clone, Copy)]
#[repr(packed(1))]
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

impl BootSector {
    pub unsafe fn read_from(volume_handle: *mut c_void) -> Result<Self, OsError> {
        let data = read_file(volume_handle, 512, None)?;
        let volume: BootSector = *PtrCast::cast(data.as_ptr());

        // Ensure this is actually an NTFS drive by comparing the metadata to the expected signature
        if volume.signature != NTFS {
            Err("attemped to scan non-ntfs drive as if it were ntfs".into())
        } else {
            Ok(volume)
        }
    }
}
