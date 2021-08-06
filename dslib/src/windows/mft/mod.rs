mod record;

use super::drive::DriveInfo;
use super::OsError;
use record::MftRecord;
use std::ops;

use super::winapi::{read_file, PtrMutCast};

/// MFT buffer size
const BUF_SIZE: usize = 256 * 1024; // 256 kb

/// An in-memory representation of the MFT table
pub struct Mft(Vec<u8>);

impl ops::Deref for Mft {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Mft {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct MftScanner {
    mft: Mft,
    drive: DriveInfo,
}

impl MftScanner {
    pub unsafe fn init(drive: DriveInfo) -> Result<Self, OsError> {
        // Read the $MFT record, this is always the first entry in the MFT table
        let mut mft = Mft(read_file(
            &drive.handle,
            drive.bytes_per_mft_record as usize,
            Some(
                drive.boot.mft_cluster as u32
                    * drive.boot.bytes_per_sector as u32
                    * drive.boot.sectors_per_cluster as u32,
            ),
        )?);

        // Expand the buffer past the first record, this prevents us from having to reallocate it later
        mft.reserve(BUF_SIZE);

        Ok(MftScanner { mft, drive })
    }

    pub unsafe fn scan(&mut self) -> Result<(), OsError> {
        // Process the $MFT record
        let mft_record: &mut MftRecord = PtrMutCast::cast(self.mft.as_mut_ptr());
        self.process_record(mft_record)?;

        Ok(())
    }

    unsafe fn process_record(&mut self, record: &mut MftRecord) -> Result<(), OsError> {
        record.fix(self)?;

        Ok(())
    }

    unsafe fn process_run_length(&mut self) {}

    unsafe fn process_run_offset(&mut self) {}

    unsafe fn process_bitmap_data(&mut self) {}
}
