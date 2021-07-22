mod raw;

use super::drive::DriveInfo;
use super::winapi::get_last_error;
use super::OsError;
use raw::MftRecord;

use std::ffi::c_void;
use std::ptr;

use winapi::um::fileapi::ReadFile;
use winapi::um::minwinbase::{OVERLAPPED_u, OVERLAPPED};

pub struct MftNode {
    // todo
}

pub fn process(drive: DriveInfo) -> Result<Vec<MftNode>, OsError> {
    // Read the $MFT record, this is always the first entry in the MFT table and acts as a record of the files in the volume
    let mut mft: Vec<u8> = Vec::with_capacity(64 * 1024); // 64kb

    unsafe {
        let mut size = 0;
        let mut overlap = OVERLAPPED_u::default();
        overlap.s_mut().Offset = drive.boot.mft_cluster as u32
            * drive.boot.bytes_per_sector as u32
            * drive.boot.sectors_per_cluster as u32;

        if ReadFile(
            drive.handle,
            mft.as_mut_ptr() as *mut c_void,
            drive.bytes_per_mft_record as u32,
            &mut size,
            // &mut Overlapped::new(offset) as *mut Overlapped as *mut c_void as *mut OVERLAPPED,
            &mut OVERLAPPED {
                u: overlap,
                hEvent: ptr::null_mut(),
                Internal: 0,
                InternalHigh: 0,
            },
        ) == 0
        {
            return Err(get_last_error().into());
        }

        if drive.bytes_per_mft_record != size as u64 {
            // todo make proper error
            panic!("Could not read complete volume data");
        }

        mft.set_len(size as usize);
    }

    // Parse the $MFT record
    let mft = unsafe { (mft.as_mut_ptr() as *mut MftRecord).as_ref().unwrap() };
    println!("{:#?}", mft);

    Ok(Vec::default())
}
