mod raw;

use super::drive::DriveInfo;
use super::winapi::get_last_error;
use super::OsError;
use raw::*;

use num_traits::FromPrimitive;
use std::ffi::c_void;
use std::ptr;

use winapi::um::fileapi::ReadFile;
use winapi::um::minwinbase::{OVERLAPPED_u, OVERLAPPED};

/// Special identifier for the root directory
const ROOT: u32 = 5;

#[derive(Debug, Default)]
pub struct MftNode {
    pub attributes: MftNodeAttributes,
    pub parent_node_index: Option<u32>,
    pub size: Option<u64>,
    pub name_index: Option<i32>,
}

pub fn process(drive: DriveInfo) -> Result<Vec<MftNode>, OsError> {
    // Read the $MFT record, this is always the first entry in the MFT table and acts as a record of the files in the volume
    let mut mft: Vec<u8> = Vec::with_capacity(256 * 1024); // 256kb

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
            return Err("could not read complete volume data".into());
        }

        mft.set_len(size as usize);
    }

    // Process the $MFT record
    let mft_record = unsafe { (mft.as_mut_ptr() as *mut MftRecord).as_ref().unwrap() };
    info!("Read $MFT record: {:#?}", mft_record);

    let node = process_record(&drive, &mut mft, &mft_record)?;
    info!("{:#?}", node);

    Ok(Vec::default())
}

fn process_record(
    drive: &DriveInfo,
    buffer: &mut [u8],
    record: &MftRecord,
) -> Result<MftNode, OsError> {
    // Sanity checks
    {
        let base = ((record.base_file_record.inode_number_high as u64) << 32)
            + record.base_file_record.inode_number_low as u64; // note that this should always be 0

        // Ensure this is an active file record
        if base != 0 || record.header.ty != raw::FILE || record.flags & 1 != 1 {
            return Err("unable to interpret mft record".into());
        }

        if record.attribute_offset as u64 >= drive.bytes_per_mft_record {
            return Err("record attribute outside expected range, the mft may be corrupt".into());
        }
        if record.bytes_in_use as u64 > drive.bytes_per_mft_record {
            return Err("record large than buffer, the mft may be corrupt".into());
        }
    }

    // ---------- Process node

    let mut node = MftNode::default();

    // Default this entry to the root
    node.parent_node_index = Some(ROOT);

    if record.flags & 2 == 2 {
        node.attributes |= MftNodeAttributes::DIRECTORY;
    }

    // ----- Process attributes

    let buf_length = drive.bytes_per_mft_record - record.attribute_offset as u64;

    let mut offset = 0;
    while offset < buf_length {
        // Parse attribute
        let attribute = unsafe {
            (buffer
                .as_mut_ptr()
                .wrapping_add(record.attribute_offset as usize) // fixme [IMPORTANT]: remember to add this to `buffer` when using as `ptr` translation vector
                .wrapping_add(offset as usize) as *mut MftNodeAttribute)
                .as_mut()
                .unwrap()
        };

        // trace!("Attribute {} + {}", offset, unsafe { *&attribute.length });

        // Exit loop at end marker
        if (offset + 4 <= buf_length)
            && *unsafe {
                (attribute as *mut MftNodeAttribute as *mut u32)
                    .as_ref()
                    .unwrap()
            } == 0xFFFFFFFF
        {
            break;
        }

        // Sanity check
        if (offset + 4 > buf_length)
            || attribute.length < 3
            || (offset + attribute.length as u64 > buf_length)
        {
            return Err("attribute large than data, the mft may be corrupt".into());
        }

        // Attribute list must be processed last
        if attribute.attribute_type == MftNodeAttributeType::AttributeList as u32 {
            continue;
        }

        // Parse extended attribute info
        if attribute.non_resident == 0 {
            let resident_attribute = unsafe {
                (attribute as *mut MftNodeAttribute as *mut MftNodeResidentAttribute)
                    .as_ref()
                    .unwrap()
            };

            match FromPrimitive::from_u32(attribute.attribute_type) {
                Some(MftNodeAttributeType::FileName) => {
                    let attribute_filename = unsafe {
                        (buffer
                            .as_mut_ptr()
                            .wrapping_add(record.attribute_offset as usize)
                            .wrapping_add(offset as usize)
                            .wrapping_add(resident_attribute.value_offset as usize)
                            as *mut MftNodeAttributeFileName)
                            .as_ref()
                            .unwrap()
                    };

                    // todo remove this restriction
                    if attribute_filename.parent_directory.inode_number_high > 0 {
                        return Err("unsupported mft inode format (48 bit)".into());
                    }

                    node.parent_node_index =
                        Some(attribute_filename.parent_directory.inode_number_low);

                    if attribute_filename.name_type == 1 || node.name_index == Some(0) {
                        println!("{}", unsafe { *&attribute_filename.name });
                        // L: 1007
                        unimplemented!(); // todo
                    }
                }
                Some(MftNodeAttributeType::StandardInformation) => {
                    let attribute_standardinformation = unsafe {
                        (buffer
                            .as_mut_ptr()
                            .wrapping_add(record.attribute_offset as usize)
                            .wrapping_add(offset as usize)
                            .wrapping_add(resident_attribute.value_offset as usize)
                            as *mut MftNodeAttributeStandardInformation)
                            .as_ref()
                            .unwrap()
                    };

                    // fixme this does not work either
                    // println!("{}", unsafe {
                    //     *&attribute_standardinformation.file_attributes as u64
                    // });
                    // let num = unsafe { *&attribute_standardinformation.file_attributes as usize };
                    // // println!("{:?}", unsafe {
                    //     (num as *const usize as *const MftNodeAttributes)
                    //         .as_ref()
                    //         .unwrap()
                    // });
                    // todo
                    // fixme causes segfault when trying to print node object
                    // node.attributes |= *unsafe {
                    //     (attribute_standardinformation.file_attributes as *const usize
                    //         as *const MftNodeAttributes)
                    //         .as_ref()
                    //         .unwrap()
                    // };
                }
                Some(MftNodeAttributeType::Data) => {
                    node.size = Some(resident_attribute.value_length as u64);
                }
                _ => (),
            }
        } else {
            // todo
        }

        offset += attribute.length as u64;
    }

    Ok(node)
}
