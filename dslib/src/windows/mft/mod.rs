mod raw;

use super::drive::DriveInfo;
use super::winapi::read_file;
use super::OsError;
use raw::*;

use num_traits::FromPrimitive;

use std::ffi::OsString;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use std::{cmp, mem};

/// Debug value
static DEBUG: AtomicUsize = AtomicUsize::new(1); // start at record 1 because the $MFT is record 0

/// Special identifier for the root directory
const ROOT: u32 = 5;

/// Virtual fragment identifier
const VIRTUALFRAGMENT: u64 = u64::MAX;

/// Mft buffer size
const BUF_SIZE: usize = 256 * 1024; // 256 kb

#[derive(Debug, Default, Clone)]
pub struct MftNode {
    /// MftNodeAttributes bitflag
    pub attributes: usize,
    pub parent_node_index: Option<u32>,
    pub size: Option<u64>,
    pub name_index: Option<usize>,
}

#[derive(Default, Debug, Clone, Copy)]
#[repr(packed(1))]
pub struct Fragment {
    pub lcn: u64,
    pub next_vcn: u64,
}

#[derive(Clone)]
pub struct Stream {
    pub name_index: usize,
    pub ty: MftNodeAttributeType,
    pub size: u64,
    pub clusters: u64,
    fragments: Option<Vec<Fragment>>,
}

impl Stream {
    pub fn push_fragment(&mut self, fragment: Fragment) {
        if self.fragments.is_none() {
            self.fragments = Some(vec![fragment]);
        } else if let Some(ref mut fragments) = self.fragments {
            fragments.push(fragment);
        }
    }

    pub fn fragments(&mut self) -> &[Fragment] {
        if let Some(ref mut fragments) = self.fragments {
            fragments
        } else {
            self.fragments = Some(vec![Fragment::default(); 5]);
            self.fragments()
        }
    }
}

pub fn find_stream(
    streams: &mut [Stream],
    ty: MftNodeAttributeType,
    name_index: Option<usize>,
) -> Option<&mut Stream> {
    for stream in streams {
        if stream.ty == ty && stream.name_index == name_index.unwrap_or(stream.name_index) {
            return Some(stream);
        }
    }

    None
}

// lazy_static! {
//     // Preallocate string index
//     static ref NAME_INDEX: Mutex<HashMap<OsString, usize>> = Mutex::new(HashMap::with_capacity(128 * 1024));
//     static ref NAMES: Mutex<Vec<OsString>> = Mutex::new(Vec::new());
// }

// fn get_name_index(name: OsString) -> usize {
//     let mut name_index = NAME_INDEX.lock().unwrap();
//     let mut names = NAMES.lock().unwrap();

//     if let Some(current) = name_index.get(&name) {
//         *current
//     } else {
//         names.push(name.clone());
//         name_index.insert(name, names.len() - 1);

//         names.len() - 1
//     }
// }

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

    Ok(mem::transmute::<[u8; 8], i64>(run_length).to_le()) // todo replace with i64::from_le_bytes()
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

    Ok(mem::transmute::<[u8; 8], i64>(run_offset).to_le()) // todo replace with i64::from_le_bytes()
}

/// Process the mft of a drive
pub fn process(drive: DriveInfo) -> Result<Vec<MftNode>, OsError> {
    // Allocate a buffer to store the mft table in
    let mut mft: Vec<u8> = Vec::with_capacity(BUF_SIZE);
    // let mut mft: Vec<u8> = vec![0; BUF_SIZE]; // rest in peace ram

    // ----- Read the $MFT record, this is always the first entry in the MFT table and acts as a record of the files in the volume
    let dmft = unsafe {
        read_file(
            &drive.handle,
            drive.bytes_per_mft_record as usize,
            Some(
                drive.boot.mft_cluster as u32
                    * drive.boot.bytes_per_sector as u32
                    * drive.boot.sectors_per_cluster as u32,
            ),
        )?
    };

    // Insert the $MFT record at the start of the buffer
    for i in 0..dmft.len() {
        mft.insert(i, 0);
    }
    mft.splice(..dmft.len(), dmft);

    let mut streams: Vec<Stream> = Vec::new();

    // ---------- Process the $MFT record
    let mft_record = unsafe { (mft.as_mut_ptr() as *mut MftRecord).as_ref().unwrap() };
    fix_mft_data(&drive, mft.as_mut_ptr())?;
    // info!("Read $MFT record: {:#?}", mft_record);

    let mft_node = process_record(&drive, mft.as_mut_ptr(), &mft_record, &mut streams, true)?
        .ok_or_else(|| OsString::from("unable to interpret $MFT record"))?;

    // ----- Process inode bitmap data
    let mut vcn: u64 = 0;
    let mut max_mft_bitmap_bytes: u64 = 0;

    let stream = find_stream(&mut streams, MftNodeAttributeType::Bitmap, None)
        .ok_or_else(|| OsError::from("bitmap data not found"))?;

    for fragment in stream.fragments() {
        if fragment.lcn != VIRTUALFRAGMENT {
            max_mft_bitmap_bytes += (fragment.next_vcn - vcn)
                * drive.boot.bytes_per_sector as u64
                * drive.boot.sectors_per_cluster as u64;
        }

        vcn = fragment.next_vcn;
    }

    let mut bitmap_data: Vec<u8> = vec![0; max_mft_bitmap_bytes as usize];
    vcn = 0;
    let mut real_vcn: u64 = 0;

    for fragment in stream.fragments() {
        if fragment.lcn != VIRTUALFRAGMENT {
            let data = unsafe {
                read_file(
                    &drive.handle,
                    (fragment.next_vcn - vcn) as usize
                        * drive.boot.bytes_per_sector as usize
                        * drive.boot.sectors_per_cluster as usize,
                    Some(
                        (fragment.lcn
                            * drive.boot.bytes_per_sector as u64
                            * drive.boot.sectors_per_cluster as u64) as u32,
                    ),
                )?
            };

            let start = real_vcn as usize
                * drive.boot.bytes_per_sector as usize
                * drive.boot.sectors_per_cluster as usize;

            for i in 0..data.len() {
                mft.insert(start + i, 0);
            }
            bitmap_data.splice(start..start + data.len(), data);

            real_vcn += fragment.next_vcn - vcn;
        }

        vcn = fragment.next_vcn;
    }

    // ----- Begin processing the actual mft records
    let mut data_stream = find_stream(&mut streams, MftNodeAttributeType::Data, None)
        .expect("data stream missing")
        .clone();

    let mut max_inode: u32 = bitmap_data.len() as u32 * 8;
    if max_inode > data_stream.size as u32 / drive.bytes_per_mft_record as u32 {
        max_inode = data_stream.size as u32 / drive.bytes_per_mft_record as u32;
    }

    // Preallocate buffer for the mft nodes
    let mut nodes: Vec<MftNode> = vec![Default::default(); max_inode as usize];
    nodes[0] = mft_node;
    info!("Parsing {} mft nodes...", nodes.len());
    // Store the next node index so we can directly write to it when we encounter a new node
    let mut next_node_index: usize = 1;

    let mut block_start: u64 = 0;
    let mut block_end: u64 = 0;
    let mut real_vcn: u64 = 0;
    let mut vcn: u64 = 0;

    let mut total_bytes_read: u64 = 0;
    let fragment_index: u32 = 0;

    let start_time = Instant::now();

    // Node that the node index starts at `1` to skip the $MFT entry
    for node_index in 1..max_inode {
        // Ignore inode entry if it is not in use
        if (bitmap_data[node_index as usize >> 3]
            & [1, 2, 3, 4, 8, 16, 32, 64, 128][node_index as usize % 8])
            == 0
        {
            continue;
        }

        if node_index >= block_end as u32 {
            // Read the next chunk
            {
                let mut fragment_index = fragment_index.clone();
                block_start = node_index as u64;
                block_end =
                    block_start as u64 + BUF_SIZE as u64 / drive.bytes_per_mft_record as u64;

                if block_end > data_stream.size * 8 {
                    block_end = data_stream.size * 8;
                }

                let mut u1: u64 = 0;
                let fragment_count: u32 = data_stream.fragments().len() as u32;
                while fragment_index < fragment_count {
                    let fragment = data_stream.fragments()[fragment_index as usize];

                    // Calculate inode at end of fragment
                    u1 = (real_vcn + fragment.next_vcn - vcn)
                        * drive.boot.bytes_per_sector as u64
                        * drive.boot.sectors_per_cluster as u64
                        * drive.bytes_per_mft_record as u64;

                    if u1 > node_index as u64 {
                        break;
                    }

                    loop {
                        if fragment.lcn != VIRTUALFRAGMENT {
                            real_vcn += fragment.next_vcn - vcn;
                        }

                        vcn = fragment.next_vcn;

                        fragment_index += 1;

                        if fragment_index >= fragment_count || fragment.lcn != VIRTUALFRAGMENT {
                            break;
                        }
                    }
                }

                if fragment_index >= fragment_count {
                    // Run out of nodes to read
                    break;
                }

                if block_end >= u1 {
                    block_end = u1;
                }

                let position = (data_stream.fragments()[fragment_index as usize].lcn as u32
                    - real_vcn as u32)
                    * drive.boot.bytes_per_sector as u32
                    * drive.boot.sectors_per_cluster as u32
                    + block_start as u32 * drive.bytes_per_mft_record as u32;

                let data = unsafe {
                    read_file(
                        &drive.handle,
                        (block_end - block_start) as usize * drive.bytes_per_mft_record as usize,
                        Some(position),
                    )?
                };

                mft.splice(..data.len(), data);
            }

            total_bytes_read += (block_end - block_start) * drive.bytes_per_mft_record;
        }

        let e = fix_mft_data(
            &drive,
            mft.as_mut_ptr().wrapping_add(
                (node_index as u64 - block_start) as usize * drive.bytes_per_mft_record as usize,
            ),
        );
        // e?;
        if e.is_err() {
            // fixme these errors should cause an error to be returned
            error!(
                "id: {} : {:?}, skipping...",
                DEBUG.fetch_add(1, Ordering::SeqCst),
                e
            );
            continue;
        }

        DEBUG.fetch_add(1, Ordering::SeqCst);

        let record = unsafe {
            (mft.as_mut_ptr().wrapping_add(
                (node_index as u64 - block_start) as usize * drive.bytes_per_mft_record as usize,
            ) as *mut MftRecord)
                .as_ref()
                .unwrap()
        };

        let node = process_record(
            &drive,
            mft.as_mut_ptr().wrapping_add(
                (node_index as u64 - block_start) as usize * drive.bytes_per_mft_record as usize,
            ),
            record,
            &mut streams,
            false,
        )?;

        if let Some(node) = node {
            let index = DEBUG.fetch_sub(1, Ordering::SeqCst);
            if next_node_index % 100 == 0
                || cmp::max(next_node_index, index)
                    .checked_sub(10000)
                    .unwrap_or_else(|| cmp::min(next_node_index, index))
                    > cmp::min(next_node_index, index)
                || next_node_index > 200000
            {
                trace!("{} | {}", next_node_index, index);
            }
            DEBUG.fetch_add(1, Ordering::SeqCst);
            nodes[next_node_index] = node;
            next_node_index += 1;
        }
    }

    nodes.truncate(next_node_index - 1);

    let mut elapsed_time = start_time.elapsed().as_secs();
    if elapsed_time == 0 {
        elapsed_time = 1;
    }
    info!(
        "Read {} MB of data in {} second(s) at a rate of {} MB/s",
        total_bytes_read / 1024 / 1024,
        elapsed_time,
        (total_bytes_read / 1024 / 1024) / elapsed_time
    );
    info!("Total nodes: {}", DEBUG.fetch_add(1, Ordering::SeqCst));
    info!("Total entries: {}", nodes.len());
    info!(
        "Skipped {} nodes",
        DEBUG.fetch_add(1, Ordering::SeqCst) - 1 - nodes.len()
    );

    Ok(nodes)
}

fn process_record(
    drive: &DriveInfo,
    buffer: *mut u8,
    record: &MftRecord,
    streams: &mut Vec<Stream>,
    is_mftnode: bool,
) -> Result<Option<MftNode>, OsError> {
    // trace!("a:inner");

    // Sanity checks
    {
        let base = ((record.base_file_record.inode_number_high as u64) << 32)
            + record.base_file_record.inode_number_low as u64; // note that this should always be 0

        // trace!("{:#?}", { record.flags });
        // Ensure this is an active file record (otherwise we skip it)
        if base != 0 || record.header.ty != raw::FILE || record.flags & 1 != 1 {
            return Ok(None);
        }

        if record.attribute_offset as u64 >= drive.bytes_per_mft_record {
            return Err("record attribute outside expected range, the mft may be corrupt".into());
        }
        if record.bytes_in_use as u64 > drive.bytes_per_mft_record {
            return Err("record large than buffer, the mft may be corrupt".into());
        }
    }

    // trace!("a:outer");

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
                .wrapping_add(record.attribute_offset as usize) // fixme [IMPORTANT]: remember to add this to `buffer` when using as `ptr` translation vector
                .wrapping_add(offset as usize) as *mut MftNodeAttribute)
                .as_mut()
                .unwrap()
        };

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
        // fixme this check causes a crash
        // if (offset + 4 > buf_length)
        //     || attribute.length < 3
        //     || (offset + attribute.length as u64 > buf_length)
        // {
        //     return Err("attribute larger than data, the mft may be corrupt".into());
        // }

        // Attribute list must be processed last
        if attribute.attribute_type == MftNodeAttributeType::AttributeList as u32 {
            offset += attribute.length as u64;
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
                        // let name = unsafe {
                        //     OsString::from_wide(std::slice::from_raw_parts(
                        //         attribute_filename.name as *const u16,
                        //         attribute_filename.name_length as usize,
                        //     ))
                        // };
                        // println!("{:?}", name);
                        // node.name_index = Some(get_name_index(name));
                        //fixme improve
                        // print!("File name ");
                        // for c_index in 0..attribute_filename.name_length as usize * 2
                        // // note multiplied by two because of utf-16 double byte characters
                        // // step two bytes at a time and parse as utf 16
                        // {
                        //     print!("{}", attribute_filename.name[c_index as usize] as char);
                        // }
                        // println!(" with size {} bytes", { attribute_filename.data_size });
                    }
                }
                Some(MftNodeAttributeType::StandardInformation) => {
                    let attribute_standardinformation = unsafe {
                        (buffer
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
                // let name = unsafe {
                //     OsString::from_wide(std::slice::from_raw_parts(
                //         buffer
                //             .wrapping_add(record.attribute_offset as usize)
                //             .wrapping_add(offset as usize)
                //             .wrapping_add(nonresident_attribute.data_size as usize)
                //             as *const u16,
                //         attribute.name_offset as usize,
                //     ))
                // };
                // println!("{:?}", name);
                // name_index = get_name_index(name);
                // fixme improve
                // this is a directory
                // let name_pos = buffer
                //     .wrapping_add(record.attribute_offset as usize)
                //     .wrapping_add(offset as usize)
                //     .wrapping_add(attribute.name_offset as usize);

                // let name: &[u16] = unsafe {
                //     ::std::slice::from_raw_parts(
                //         name_pos as *const u16,
                //         attribute.name_length as usize,
                //     )
                // };
                // let name = OsString::from_wide(name);
                // println!("File name {:?}", name);
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
                        fragments: None,
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

                    stream.push_fragment(Fragment {
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

    Ok(Some(node))
}

fn fix_mft_data(drive: &DriveInfo, mft: *mut u8) -> Result<(), OsError> {
    let record = unsafe { (mft as *mut MftRecord).as_ref().unwrap() };
    // let header = unsafe { (mft as *mut MftRecordHeader).as_ref().unwrap() };

    if record.header.ty != FILE {
        return Ok(());
    }

    let buffer = mft as *mut u16;
    let update_sequence_array = mft.wrapping_add(record.header.usa_offset as usize) as *mut u16;
    let increment = drive.boot.bytes_per_sector as usize / mem::size_of::<u16>();

    let mut index = increment - 1;

    for i in 1..record.header.usa_count {
        // Check if we are in the buffer
        if index * mem::size_of::<u16>() >= drive.bytes_per_mft_record as usize {
            return Err("usa data indicates data is missing, the mft may be corrupt".into());
        }

        // Ensure the last 2 bytes of the sector contain the usn
        if unsafe { buffer.wrapping_add(index).as_ref().unwrap() }
            != unsafe { update_sequence_array.as_ref().unwrap() }
        {
            // fixme error misfires
            debug!(
                "{} != {} | {} -> {}",
                unsafe { buffer.wrapping_add(index).as_ref().unwrap() },
                unsafe { update_sequence_array.as_ref().unwrap() },
                unsafe { *buffer.wrapping_add(index) },
                unsafe { *update_sequence_array.wrapping_add(i as usize) }
            );
            // if unsafe { *buffer.wrapping_add(index) } == 2661 {
            //     info!("{:#?}", record);
            // }
            return Err("usa fixup word does not match usn, the mft may be corrupt".into());
        }

        // Replace the last 2 bytes in the sector with the usa value
        unsafe { *buffer.wrapping_add(index) = *update_sequence_array.wrapping_add(i as usize) };
        assert_eq!(unsafe { *buffer.wrapping_add(index) }, unsafe {
            *update_sequence_array.wrapping_add(i as usize)
        });
        index += increment;
    }

    Ok(())
}
