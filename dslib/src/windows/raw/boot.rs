use super::winapi::get_last_error;
use super::OsError;
use std::ffi::c_void;
use std::{fmt, ptr};
use winapi::um::fileapi::ReadFile;

/// NTFS boot sector information
/// See http://ntfs.com/ntfs-partition-boot-sector.htm for extra information
#[derive(Clone, Copy)]
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

impl fmt::Debug for BootSector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe {
            f.debug_struct("BootSector")
                .field("bytes_per_sector", &self.bytes_per_sector)
                .field("sectors_per_cluster", &self.sectors_per_cluster)
                .field("total_sectors", &self.total_sectors)
                .field("mft_cluster", &self.mft_cluster)
                .field("mftmirr_cluster", &self.mftmirr_cluster)
                .field("clusters_per_mft_record", &self.clusters_per_mft_record)
                .field("clusters_per_index_record", &self.clusters_per_index_record)
                .finish()
        }
    }
}

impl BootSector {
    pub unsafe fn read_from(volume_handle: *mut c_void) -> Result<Self, OsError> {
        let mut volume_data: Vec<u8> = Vec::with_capacity(512);
        let mut size = 0;

        if ReadFile(
            volume_handle,
            volume_data.as_mut_ptr() as *mut c_void,
            512,
            &mut size,
            ptr::null_mut(),
        ) == 0
        {
            return Err(get_last_error().into());
        }

        volume_data.set_len(size as usize);

        let volume_data: &mut BootSector = (volume_data.as_mut_ptr() as *mut BootSector)
            .as_mut()
            .unwrap();

        // Ensure this is actually an NTFS drive by comparing the metadata to the expected signature
        // NTFS OEM ID taken from https://opensource.apple.com/source/ntfs/ntfs-94/newfs/layout.h.auto.html
        if volume_data.signature != 0x202020205346544E {
            Err("attemped to scan non-ntfs drive as if it were ntfs, this is likely a bug".into())
        } else {
            Ok(*volume_data)
        }
    }
}
