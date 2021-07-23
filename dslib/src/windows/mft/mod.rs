mod raw;

use super::drive::DriveInfo;
use super::winapi::get_last_error;
use super::OsError;
use raw::*;

use num_traits::FromPrimitive;
use std::ffi::c_void;
use std::{mem, ptr};

use winapi::um::fileapi::ReadFile;
use winapi::um::minwinbase::{OVERLAPPED_u, OVERLAPPED};

/// Special identifier for the root directory
const ROOT: u32 = 5;

/// Virtual fragment identifier
const VIRTUALFRAGMENT: u64 = u64::MAX;

#[derive(Debug, Default)]
pub struct MftNode {
    /// MftNodeAttributes bitflag
    pub attributes: usize,
    pub parent_node_index: Option<u32>,
    pub size: Option<u64>,
    pub name_index: Option<i32>,
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(packed(1))]
pub struct Fragment {
    pub lcn: u64,
    pub next_vcn: u64,
}

#[derive(Debug)]
pub struct Stream {
    pub name_index: u32,
    pub ty: MftNodeAttributeType,
    pub size: u64,
    pub clusters: u64,
    pub fragments: Vec<Fragment>,
}

pub fn find_stream(
    streams: &mut [Stream],
    ty: MftNodeAttributeType,
    name_index: Option<u32>,
) -> Option<&mut Stream> {
    for stream in streams {
        if stream.ty == ty && stream.name_index == name_index.unwrap_or(stream.name_index) {
            return Some(stream);
        }
    }

    None
}

pub unsafe fn process_run_length(
    run_data: *mut u8,
    run_data_length: u32,
    run_length_size: i32,
    index: &mut u32,
) -> Result<i64, OsError> {
    let mut run_length: [u8; 8] = [0; 8]; // i64

    for i in 0..run_length_size {
        run_length[i as usize] = *run_data.wrapping_add(*index as usize);

        *index += 1;
        if *index >= run_data_length {
            return Err(OsError::from(
                "data is longer than buffer, the mft may be corrupt",
            ));
        }
    }

    Ok(mem::transmute::<[u8; 8], i64>(run_length).to_le())
}

pub unsafe fn process_run_offset(
    run_data: *mut u8,
    run_data_length: u32,
    run_offset_size: i32,
    index: &mut u32,
) -> Result<i64, OsError> {
    let mut run_offset: [u8; 8] = [0; 8]; // i64

    let mut i: usize = 0;
    while (i as i32) < run_offset_size {
        run_offset[i] = *run_data.wrapping_add(*index as usize);

        *index += 1;
        if *index >= run_data_length {
            return Err(OsError::from(
                "data is longer than buffer, the mft may be corrupt",
            ));
        }

        i += 1;
    }

    // Process negative values
    if run_offset[i - 1] >= 0x80 {
        while i < 8 {
            run_offset[i] = 0xFF;
            i += 1;
        }
    }

    Ok(mem::transmute::<[u8; 8], i64>(run_offset).to_le())
}

/// Process the mft of a drive
pub fn process(drive: DriveInfo) -> Result<Vec<MftNode>, OsError> {
    // ----- Read the $MFT record, this is always the first entry in the MFT table and acts as a record of the files in the volume
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

    let mut streams: Vec<Stream> = Vec::new();

    // ---------- Process the $MFT record
    let mft_record = unsafe { (mft.as_mut_ptr() as *mut MftRecord).as_ref().unwrap() };
    // info!("Read $MFT record: {:#?}", mft_record);

    let mft_node = process_record(&drive, &mut mft, &mft_record, &mut streams, true)?;
    // info!("{:#?}", mft_node);
    // info!("{:#?}", streams);

    // ----- Process inode bitmap data
    // let mut vcn: u64 = 0;
    // let mut max_mft_bitmap_bytes: u64 = 0;

    // let stream = find_stream(&mut streams, MftNodeAttributeType::Bitmap, None)
    //     .ok_or_else(|| OsError::from("bitmap data not found"))?;

    // for fragment in &stream.fragments {
    //     println!("{}", { fragment.lcn });
    //     if fragment.lcn != VIRTUALFRAGMENT {
    //         max_mft_bitmap_bytes += (fragment.next_vcn - vcn)
    //             * drive.boot.bytes_per_sector as u64
    //             * drive.boot.sectors_per_cluster as u64;
    //     }

    //     vcn = fragment.next_vcn;
    // }

    // ----- Begin processing the actual mft records

    // // for mft_index in 1..32 {
    //     let rec = unsafe {
    //         (mft.as_mut_ptr()
    //             .wrapping_add(drive.bytes_per_mft_record as usize) as *mut MftRecord)
    //             .as_ref()
    //             .unwrap()
    //     };
    //     trace!("{:#?}", rec);
    //     trace!("{:#?}", process_record(&drive, &mut mft, &rec));
    // // }

    Ok(Vec::default())
}

fn process_record(
    drive: &DriveInfo,
    buffer: &mut [u8],
    record: &MftRecord,
    streams: &mut Vec<Stream>,
    is_mftnode: bool,
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

    // Default this entry to the root
    let mut node = MftNode {
        parent_node_index: Some(ROOT),
        ..Default::default()
    };

    if record.flags & 2 == 2 {
        node.attributes |= MftNodeAttributes::Directory as usize;
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
                        trace!("Name: {}", { attribute_filename.name });
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

                    node.attributes |= attribute_standardinformation.file_attributes as usize;
                }
                Some(MftNodeAttributeType::Data) => {
                    node.size = Some(resident_attribute.value_length as u64);
                }
                _ => (),
            }
        } else {
            let nonresident_attribute = unsafe {
                (attribute as *mut MftNodeAttribute as *mut MftNodeNonResidentAttribute)
                    .as_ref()
                    .unwrap()
            };

            if attribute.attribute_type == MftNodeAttributeType::Data as u32 && node.size == Some(0)
            {
                node.size = Some(nonresident_attribute.data_size);
            }

            let name_index = 0;

            if attribute.name_length > 0 {
                unimplemented!(); // todo L: 1044
            }

            let ty = MftNodeAttributeType::from_u32(attribute.attribute_type)
                .expect("invalid node attribute");
            let stream = find_stream(streams, ty, Some(name_index));

            let stream = match stream {
                Some(stream) => {
                    stream.size = nonresident_attribute.data_size;
                    stream
                }
                None => {
                    streams.push(Stream {
                        name_index,
                        ty,
                        size: nonresident_attribute.data_size,
                        clusters: 0,
                        fragments: vec![Fragment::default(); 5],
                    });
                    streams.last_mut().unwrap()
                }
            };

            if is_mftnode {
                // Process fragments
                let run_data_length =
                    attribute.length - nonresident_attribute.run_array_offset as u32;

                let mut index: u32 = 0;
                let mut lcn: i64 = 0;
                let mut vcn: i64 = nonresident_attribute.starting_vcn as i64;
                let mut run_offset_size: u32;
                let mut run_length_size: u32;

                let run_data = buffer
                    .as_mut_ptr()
                    .wrapping_add(record.attribute_offset as usize)
                    .wrapping_add(offset as usize)
                    .wrapping_add(nonresident_attribute.run_array_offset as usize);

                while unsafe { *(run_data.wrapping_add(index as usize) as *const u8) } != 0 {
                    let data = unsafe { *(run_data.wrapping_add(index as usize) as *const u8) };

                    run_length_size = (data & 0x0F) as u32;
                    run_offset_size = ((data & 0xF0) >> 4) as u32;

                    index += 1;
                    if index >= run_data_length {
                        return Err("data is longer than buffer, the mft may be corrupt".into());
                    }

                    let run_length: i64 = unsafe {
                        process_run_length(
                            run_data,
                            run_data_length,
                            run_length_size as i32,
                            &mut index,
                        )?
                    };

                    let run_offset: i64 = unsafe {
                        process_run_offset(
                            run_data,
                            run_data_length,
                            run_offset_size as i32,
                            &mut index,
                        )?
                    };

                    lcn += run_offset;
                    vcn += run_length;

                    if run_offset != 0 {
                        stream.clusters += run_length as u64;
                    }

                    stream.fragments.push(Fragment {
                        lcn: if run_offset == 0 {
                            VIRTUALFRAGMENT
                        } else {
                            lcn as u64
                        },
                        next_vcn: vcn as u64,
                    });
                }
            }
        }

        offset += attribute.length as u64;
    }

    Ok(node)
}
