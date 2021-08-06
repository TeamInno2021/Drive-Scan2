mod raw;

use super::drive::DriveInfo;
use super::OsError;
use raw::*;

use super::winapi::read_file;

/// MFT buffer size
const BUF_SIZE: usize = 256 * 1024; // 256 kb

pub struct MftScanner {
    drive: DriveInfo,
}

impl MftScanner {
    pub fn new(drive: DriveInfo) -> Self {
        MftScanner { drive }
    }

    pub fn scan(&mut self) -> Result<(), OsError> {
        let drive = &self.drive;

        // Read the $MFT record, this is always the first entry in the MFT table
        let mut mft = unsafe {
            read_file(
                &drive.handle,
                drive.bytes_per_mft_record as usize,
                Some(
                    drive.boot.mft_cluster as u32
                        * drive.boot.bytes_per_sector as u32
                        * drive.boot.sectors_per_cluster as u32,
                ),
            )
        }?;

        mft.reserve(BUF_SIZE - mft.len());

        Ok(())
    }

    unsafe fn process_run_length(&mut self) {}

    unsafe fn process_run_offset(&mut self) {}

    unsafe fn fix_record(&mut self) {}

    unsafe fn process_record(&mut self) {}

    unsafe fn process_bitmap_data(&mut self) {}
}
