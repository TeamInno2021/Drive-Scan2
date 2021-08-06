use super::winapi::{read_file, Handle, PtrCast};
use super::OsError;

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

impl BootSector {
    pub unsafe fn read_from(volume: &Handle) -> Result<Self, OsError> {
        let data = read_file(volume, 512, None)?;
        let boot: BootSector = *PtrCast::cast(data.as_ptr());

        // Ensure this is actually an NTFS drive by comparing the metadata to the expected signature
        if boot.signature != NTFS {
            Err("attemped to scan non-ntfs drive as if it were ntfs".into())
        } else {
            Ok(boot)
        }
    }
}
