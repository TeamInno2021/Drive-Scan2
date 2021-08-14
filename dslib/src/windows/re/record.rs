use super::super::winapi::PtrMutCast;
use super::{MftScanner, OsError};
use std::mem::size_of;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MftRecordHeaderType {
    File = 0x454c4946,
}

#[repr(packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct MftRecordHeader {
    pub ty: MftRecordHeaderType,
    pub usa_offset: u16,
    pub usa_count: u16,
    pub lsn: u64,
}

#[repr(packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct INodeMeta {
    pub inode_number_low: u32,
    pub inode_number_high: u16,
    pub sequence_number: u16,
}

#[repr(packed(1))]
#[derive(Debug, Clone, Copy)]
pub struct MftRecord {
    pub header: MftRecordHeader,
    pub sequence_number: u16,
    pub link_count: u16,
    pub attribute_offset: u16,
    pub flags: u16,
    pub bytes_in_use: u32,
    pub bytes_allocated: u32,
    pub base_file_record: INodeMeta,
    pub next_attribute_number: u16,
    _padding: u16,
    pub mft_record_number: u32,
    pub update_seq_num: u16,
}

impl MftRecord {
    pub unsafe fn fix(&mut self, scanner: &mut MftScanner) -> Result<(), OsError> {
        // Ignore anything that isn't a file
        if { self.header.ty } != MftRecordHeaderType::File {
            return Ok(());
        }

        let buffer: *mut u16 = PtrMutCast::cast(scanner.mft.as_mut_ptr());
        let usa = buffer.offset(self.header.usa_count as isize);
        let increment = scanner.drive.boot.bytes_per_sector as usize / size_of::<u16>();

        let mut index = increment - 1;

        for i in 1..self.header.usa_count {
            // Ensure we are in the buffer
            if index * size_of::<u16>() >= scanner.drive.bytes_per_mft_record as usize {
                return Err("usa indicates data is missing, the mft may be corrupt".into());
            }

            // Ensure the last two bytes of the sector match the usn
            if *buffer.offset(index as isize) != *usa {
                return Err("usa fixup word does not match usn, the mft may be corrupt".into());
            }

            // Replace the last two bytes in the sector with the usa value
            *buffer.offset(index as isize) = *usa.offset(i as isize);

            index += increment;
        }

        Ok(())
    }
}
