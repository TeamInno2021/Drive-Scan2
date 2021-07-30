use std::io::{SeekFrom, Error, ErrorKind};
use std::fs::{File, OpenOptions, read_to_string};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use nix::sys::statvfs::{statvfs, Statvfs};


#[repr(packed(1))]
#[derive(Copy, Clone)]
pub struct SuperBlock {
    
}

#[repr(packed(1))]
#[derive(Copy, Clone)]
pub struct BlockGroup {
    ///# BlockGroup
    /// Makes parsing blockgroup info into BlockGroup easier
    _padding1:                  [u8;8],
    pub inode_table_lo:         [u8;4],
    pub free_blocks_count_lo:   [u8;2],
    pub free_inodes_count_lo:   [u8;2],
    pub used_dirs_count_lo:     [u8;2],
    _padding2:                  [u8;14],
    pub inode_table_hi:         [u8;4],
    pub free_blocks_count_hi:   [u8;2],
    pub free_inodes_count_hi:   [u8;2],
    pub used_dirs_count_hi:     [u8;2],
    pub inode_table:            u64,
    pub free_blocks_count:      u32,
    pub free_inodes_count:      u32,
    pub used_dirs_count:        u32,
}

impl BlockGroup {
    ///Will move 
    pub fn from_pointer(seek: SeekFrom, mut handle: File) -> Result<BlockGroup, Box<dyn std::error::Error>> {
        let mut buf: Vec<u8> = vec![0x00, 40];
        //Bypass the 1024B padding
        handle.seek(seek)?;
        handle.read_exact(&mut buf)?;
        let mut block_group = unsafe { *(buf.as_mut_ptr() as *mut BlockGroup).as_ref().unwrap() };
        block_group.inode_table = lohi_u64(block_group.inode_table_lo, block_group.inode_table_hi);
        block_group.free_blocks_count = lohi_u32(block_group.free_blocks_count_lo, block_group.free_blocks_count_hi);
        block_group.used_dirs_count = lohi_u32(block_group.used_dirs_count_lo, block_group.used_dirs_count_hi);
        block_group.free_inodes_count = lohi_u32(block_group.free_inodes_count_lo, block_group.free_inodes_count_hi);
        Ok(block_group)
    }
}

enum Endian {
    Big,
    Little
}

//Get the device blkid (e.g /dev/nvme0n1p3) of the partition on which a folder resides
pub fn blkid_from_dir(dir: &Path) -> Result<String, Box::<dyn std::error::Error>> {
    //Get the filesystem id
    let fsid = statvfs(dir)?.filesystem_id();

    //TODO: use BufReader
    //Find the partition with that id in /proc/mounts
    let mounts = read_to_string("/proc/mounts")?; 
    for record in mounts.lines() {
        let mount_info: Vec<&str> = record.split(" ").collect();
        //Check the fsid of the current mountpoint (2nd field in each mount record) against the fsid of the file
        let mountpoint_fsid = statvfs(mount_info[1])?.filesystem_id();
        if mountpoint_fsid == fsid {
            return Ok(mount_info[0].to_string()); //We have the file we're looking for
        }
    }
    Err(Box::new(std::io::Error::new(ErrorKind::NotFound, format!("No mounted filesystem with fsid: {}", fsid))))
}

pub fn get_first_blockgroup(dir: &Path) -> Result<BlockGroup, Box<dyn std::error::Error>> {
    //Open the partition
    let mut partitionhandle = File::open(dir)?;
    partitionhandle.seek(SeekFrom::Start(1024+0x104))?;  //Skip past padding at the start
    let bg_offset = read_u32(&mut partitionhandle)?;
    BlockGroup::from_pointer(SeekFrom::Start(1024 + bg_offset as u64), partitionhandle)
}   

/// # le_read_u32
/// Reads a little endian u32 from a file stream
pub fn read_u32(handle: &mut File) -> Result<u32, Box<dyn std::error::Error>> {
    let mut buffer: [u8;4] = [0x00, 0x00, 0x00, 0x00];
    handle.read_exact(&mut buffer)?;
    Ok(u32::from_le_bytes(buffer))
}

/// # le_read_u16
/// Reads a little endian u16 from a file stream
pub fn read_u16(handle: &mut File) -> Result<u16, Box<dyn std::error::Error>> {
    let mut buffer: [u8;2] = [0x00, 0x00];
    handle.read_exact(&mut buffer)?;
    Ok(u16::from_le_bytes(buffer))
}

/// # lohi_u32
/// Combines two arrays of two u8s into 1 u32
pub fn lohi_u32(lo: [u8;2], hi: [u8;2]) -> u32 {
    let buf: [u8;4] = [lo[0], lo[1], hi[0], hi[1]];
    u32::from_le_bytes(buf)
}

/// # lohi_u64
/// Combines two arrays of four u8s into 1 u64
pub fn lohi_u64(lo: [u8;4], hi: [u8;4]) -> u64 {
    let buf: [u8;8] = [lo[0], lo[1], lo[2], lo[3], hi[0], hi[1], hi[2], hi[3]];
    u64::from_le_bytes(buf)
}

//  # le_merge_u32_u64

// impl SuperBlock {
//     macro_rules! from_pointer -> SuperBlock {
//         ($source: ident, [ $(($field:expr, $endian:tt)),+ ]) => {
//             let data: SuperBlock = unsafe { ($ident as *mut SuperBlock).as_ref().unwrap() } 
//             for conversion_target in field {
//                 match $endian {
//                     Endian::Big => data.conversion_target = data.conversion_target.to_big(),
//                     Endian::Little => data.conversion_target = data.conversion_target.to_little()
//                 }
//             }
//             *data;
//         };
//     }
// }

// #[repr(packed(1))]
// #[derive(Copy, Clone)]
// pub struct SuperBlock {
//     u32:        s_inodes_count,
//     u32:        s_blocks_count_lo,
//     u32:        s_r_blocks_count_lo,
//     u32:        s_free_blocks_count_lo,
//     u32:        s_free_inodes_count,
//     u32:        s_first_data_block,
//     u32:        s_log_block_size,
//     u32:        s_log_cluster_size,
//     u32:        s_blocks_per_group,
//     u32:        s_clusters_per_group,
//     u32:        s_inodes_per_group,
//     u32:        s_mtime,
//     u32:        s_wtime,
//     u16:        s_mnt_count,
//     u16:        s_max_mnt_count,
//     u16:        s_magic,
//     u16:        s_state,
//     u16:        s_errors,
//     u16:        s_minor_rev_level,
//     u32:        s_lastcheck,
//     u32:        s_checkinterval,
//     u32:        s_creator_os,
//     u32:        s_rev_level,
//     u16:        s_def_resuid,
//     u16:        s_def_resgid,
//     u32:        s_first_ino,
//     u16:        s_inode_size,
//     u16:        s_block_group_nr,
//     u32:        s_feature_compat,
//     u32:        s_feature_incompat,
//     u32:        s_feature_ro_compat,
//     [u8;16]:    s_uuid,
//     [char;16]:  s_volume_name,
//     [char;64]:  s_last_mounted,
//     u32:        s_algorithm_usage_bitmap,
//     u8:         s_prealloc_blocks,
//     u8:         s_prealloc_dir_blocks,
//     u16:        s_prealloc_gdt_blocks,
//     [u8;16]:    s_journal_uuid,
//     u32:        s_journal_inum,
//     u32:        s_journal_death,
//     u32:        s_last_orphan,
//     [u32;4]:    s_hash_seed,
//     u8:         s_def_hash_version,
//     u8:         s_jnl_backup_type,
//     u16:        s_desc_size,
//     u32:        s_default_mount_opts,
//     u32:        s_first_meta_bg,
//     u32:        s_mkfs_time,
//     [u32;17]:   s_jnl_blocks,
//     u32:        s_blocks_count_hi,
//     u32:        s_r_blocks_count_hi,
//     u32:        s_free_blocks_count_hi,
//     u16:        s_min_extra_isize,
//     u16:        s_want_extra_isize,
//     u32:        s_flags,
//     u16:        s_raid_stride,
//     u16:        s_mmp_interval,
//     u64:        s_mmp_block,
//     u32:        s_raid_stripe_width,
//     u8:         s_log_groups_per_flex,
//     u8:         s_checksum_type,
//     u16:        s_reserved_pad,
//     u64:        s_kbytes_written,
//     u32:        s_snapshot_inum,
//     u32:        s_snapshot_id,
//     u64:        s_snapshot_r_blocks_count,
//     u32:        s_snapshot_list,
//     u32:        s_error_count,
//     u32:        s_first_error_time,
//     u32:        s_first_error_ino,
//     u32:        s_first_error_block,
//     [u8;32]:    s_first_error_func,
//     u32:        s_first_error_line,
//     u32:        s_last_error_time,
//     u32:        s_last_error_ino,
//     u32:        s_last_error_line,
//     u64:        s_last_error_block,
//     [u8;32]:    s_last_error_func,
//     [u8;64]:    s_mount_opts,
//     u32:        s_usr_quota_inum,
//     u32:        s_grp_quota_inum,
//     u32:        s_overhead_blocks,
//     [u32;2]:    s_backup_bgs,
//     [u8;4]:     s_encrypt_algos,
//     [u8;16]:    s_encrypt_pw_salt,
//     u32:        s_lpf_ino,
//     u32:        s_prj_quota_inum,
//     u32:        s_checksum_seed,
//     u8:         s_wtime_hi,
//     u8:         s_mtime_hi,
//     u8:         s_mkfs_time_hi,
//     u8:         s_lastcheck_hi,
//     u8:         s_first_error_time_hi,
//     u8:         s_last_error_time_hi,
//     [u8;2]:     s_pad,
//     u16:        s_encoding,
//     u16:        s_encoding_flags,
//     [u32;95]:   s_reserved,
//     u32:        s_checksum
// }
