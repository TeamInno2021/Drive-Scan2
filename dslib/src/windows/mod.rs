mod boot;
mod drive;
mod error;
mod filesystem;
mod mft;
mod winapi;

use super::File;
use drive::DriveInfo;
use error::OsError;
use std::path::{Path, PathBuf};

pub fn verify(dir: &Path) -> Result<bool, Box<dyn ::std::error::Error>> {
    Ok(filesystem::identify(dir)? == "NTFS")
}

pub fn scan(dir: PathBuf) -> Result<File, Box<dyn ::std::error::Error>> {
    let drive = DriveInfo::parse(dir.clone())?;
    info!("Fetched metadata for {:#?}", drive);

    let _nodes = mft::process(drive)?;

    // Time to read the actual MFT,
    // here we go

    // use ::std::ffi::c_void;
    // use ::std::ptr;

    // use crate::interface::raw::mft::MftFileRecordHeader;
    // use crate::interface::winapi::get_last_error;

    // use ::winapi::um::fileapi::ReadFile;
    // use ::winapi::um::minwinbase::{OVERLAPPED_u, OVERLAPPED};

    // // ---------- Read the $MFT record, this is always the first record in the MFT
    // let offset = drive.boot.mft_cluster as u32
    //     * drive.boot.bytes_per_sector as u32
    //     * drive.boot.sectors_per_cluster as u32;

    // let mut overlap = OVERLAPPED_u::default();
    // unsafe { overlap.s_mut().Offset = offset }

    // let mut buffer: Vec<u8> = Vec::with_capacity(256 * 1024); // 256kb
    // let mut size = 0;

    // unsafe {
    //     if ReadFile(
    //         drive.handle,
    //         buffer.as_mut_ptr() as *mut c_void,
    //         drive.bytes_per_mft_record as u32,
    //         &mut size,
    //         &mut OVERLAPPED {
    //             u: overlap,
    //             hEvent: ptr::null_mut(),
    //             Internal: 0,
    //             InternalHigh: 0,
    //         },
    //     ) == 0
    //     {
    //         return Err(Box::new(OsError::from(get_last_error())));
    //     }

    //     buffer.set_len(size as usize);
    // }

    // // ---------- Format the data and check it is a proper mft record
    // // let len = drive.bytes_per_mft_record
    // let header: &mut MftFileRecordHeader = unsafe {
    //     (buffer.as_mut_ptr() as *mut MftFileRecordHeader)
    //         .as_mut()
    //         .unwrap()
    // };
    // info!("Read $MFT record: {:#?}", header);

    // if header.header.ty != raw::mft::MftRecordType::File {
    //     // todo return error
    //     panic!("invalid $MFT record type");
    // }

    // fixme break

    // let mut wide_buffer: Vec<u16> = buffer
    //     .chunks_exact(2)
    //     .into_iter()
    //     .map(|a| u16::from_ne_bytes([a[0], a[1]]))
    //     .collect();

    // let update_sequence_array = unsafe {
    //     ::std::slice::from_raw_parts(
    //         buffer.as_mut_ptr().add(header.header.usa_offset as usize),
    //         header.header.usa_count as usize,
    //     )
    // };
    // let increment = (drive.boot.bytes_per_sector / size_of::<u16>() as u16) as usize;

    // let mut index = increment - 1;

    // for i in 1..header.header.usa_count {
    //     if index * size_of::<u16>() >= drive.bytes_per_mft_record as usize {
    //         /todo return error
    //         panic!("incorrect data something something your mft might be corrupt");
    //     }

    //     if wide_buffer[index] != update_sequence_array[0] as u16 {
    //         /todo return error
    //         panic!("more errors more mft corruption this isn't looking too good for your computer");
    //     }

    //     /todo
    //     wide_buffer[index] = update_sequence_array[i as usize] as u16;
    //     index += increment;
    // }

    Ok(File::File { path: dir, size: 0 })
}

/*
private unsafe void FixupRawMftdata(byte* buffer, UInt64 len)
{
    FileRecordHeader* ntfsFileRecordHeader = (FileRecordHeader*)buffer;

    if (ntfsFileRecordHeader->RecordHeader.Type != RecordType.File)
        return;

    UInt16* wordBuffer = (UInt16*)buffer;

    UInt16* UpdateSequenceArray = (UInt16*)(buffer + ntfsFileRecordHeader->RecordHeader.UsaOffset);
    UInt32 increment = (UInt32)_diskInfo.BytesPerSector / sizeof(UInt16);

    UInt32 Index = increment - 1;

    for (int i = 1; i < ntfsFileRecordHeader->RecordHeader.UsaCount; i++)
    {
        /* Check if we are inside the buffer. */
        if (Index * sizeof(UInt16) >= len)
            throw new Exception("USA data indicates that data is missing, the MFT may be corrupt.");

        // Check if the last 2 bytes of the sector contain the Update Sequence Number.
        if (wordBuffer[Index] != UpdateSequenceArray[0])
            throw new Exception("USA fixup word is not equal to the Update Sequence Number, the MFT may be corrupt.");

        /* Replace the last 2 bytes in the sector with the value from the Usa array. */
        wordBuffer[Index] = UpdateSequenceArray[i];
        Index = Index + increment;
    }
}
 */
