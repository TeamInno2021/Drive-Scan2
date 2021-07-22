use bitflags::bitflags;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::convert::TryFrom;

// ---------- MFT ----------

pub const FILE: u32 = 0x454c4946;

#[derive(Debug, Clone, Copy)]
#[repr(packed(1))]
pub struct MftRecordHeader {
    /// 0x454c4946 = 'FILE'
    pub ty: u32, // todo convert to enum type (must be able to resize enum)
    pub usa_offset: u16,
    pub usa_count: u16,
    pub lsn: u64,
}

#[derive(Debug, Clone, Copy)]
#[repr(packed(1))]
pub struct INodeMeta {
    pub inode_number_low: u32,
    pub inode_number_high: u16,
    pub sequence_number: u16,
}

#[derive(Debug, Clone, Copy)]
#[repr(packed(1))]
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

// ---------- MFT Attribute ----------

bitflags! {
    #[derive(Default,)]
    pub struct MftNodeAttributes: usize {
        const NULL = 0;

        const READ_ONLY = 1 << 0;
        const HIDDEN = 1 << 1;
        const SYSTEM = 1 << 2;
        const DIRECTORY = 1 << 3;
        const ARCHIVE = 1 << 4;
        const DEVICE = 1 << 5;
        const NORMAL = 1 << 6;
        const TEMPORARY = 1 << 7;
        const SPARSE_FILE = 1 << 8;
        const REPARSE_POINT = 1 << 9;
        const COMPRESSED = 1 << 10;
        const OFFLINE =  1 << 11;
        const NOT_CONTENT_INDEXED = 1 << 12;
        const ENCRYPTED = 1 << 13;
    }
}

// impl FromPrimitive for MftNodeAttributes {
//     fn from_u64(n: u64) -> Option<Self> {
//         if n == 0 {
//             Some(MftNodeAttributes::NULL)
//         } else if n == 1 << 0 {
//             Some(MftNodeAttributes::READ_ONLY)
//         } else if n == 1 << 1 {
//             Some(MftNodeAttributes::HIDDEN)
//         } else if n == 1 << 2 {
//             Some(MftNodeAttributes::SYSTEM)
//         } else if n == 1 << 3 {
//             Some(MftNodeAttributes::DIRECTORY)
//         } else if n == 1 << 4 {
//             Some(MftNodeAttributes::ARCHIVE)
//         } else if n == 1 << 5 {
//             Some(MftNodeAttributes::DEVICE)
//         } else if n == 1 << 6 {
//             Some(MftNodeAttributes::NORMAL)
//         } else if n == 1 << 7 {
//             Some(MftNodeAttributes::TEMPORARY)
//         } else if n == 1 << 8 {
//             Some(MftNodeAttributes::SPARSE_FILE)
//         } else if n == 1 << 9 {
//             Some(MftNodeAttributes::REPARSE_POINT)
//         } else if n == 1 << 10 {
//             Some(MftNodeAttributes::COMPRESSED)
//         } else if n == 1 << 11 {
//             Some(MftNodeAttributes::OFFLINE)
//         } else if n == 1 << 12 {
//             Some(MftNodeAttributes::NOT_CONTENT_INDEXED)
//         } else if n == 1 << 13 {
//             Some(MftNodeAttributes::ENCRYPTED)
//         } else {
//             None
//         }
//     }

//     /// Note that this method will always return `None`, use an unsigned function instead
//     fn from_i64(n: i64) -> Option<Self> {
//         MftNodeAttributes::from_u64(u64::try_from(n).ok()?)
//     }
// }

#[derive(Debug, Clone, Copy, FromPrimitive)]
pub enum MftNodeAttributeType {
    Invalid = 0x00,
    StandardInformation = 0x10,
    AttributeList = 0x20,
    FileName = 0x30,
    ObjectId = 0x40,
    SecurityDescriptor = 0x50,
    VolumeName = 0x60,
    VolumeInformation = 0x70,
    Data = 0x80,
    IndexRoot = 0x90,
    IndexAllocation = 0xA0,
    Bitmap = 0xB0,
    ReparsePoint = 0xC0,
    EAInformation = 0xD0,
    EA = 0xE0,
    PropertySet = 0xF0,
    LoggedUtilityStream = 0x100,
}

#[derive(Debug, Clone, Copy)]
#[repr(packed(1))]
pub struct MftNodeAttribute {
    /// `MftNodeAttributeType`
    pub attribute_type: u32, // todo convert to enum type (must be able to resize enum)
    pub length: u32,
    pub non_resident: u8,
    pub name_length: u8,
    pub name_offset: u16,
    /// 0x0001 = Compressed, 0x4000 = Encrypted, 0x8000 = Sparse
    pub flags: u16,
    pub attribute_number: u16,
}

// ----- MFT Resident Attribute -----

#[derive(Debug, Clone, Copy)]
#[repr(packed(1))]
pub struct MftNodeResidentAttribute {
    pub attribute: MftNodeAttribute,
    pub value_length: u32,
    pub value_offset: u16,
    /// 0x0001 = Indexed
    pub flags: u16,
}

// ----- MFT Non-Resident Attribute -----

#[derive(Debug, Clone, Copy)]
#[repr(packed(1))]
pub struct MftNodeNonResidentAttribute {
    pub attribute: MftNodeAttribute,
    pub starting_vsc: u64,
    pub last_vsc: u64,
    pub run_array_offset: u16,
    pub compression_unit: u8,
    _padding: [u8; 5],
    pub allocated_size: u64,
    pub data_size: u64,
    pub initialized_size: u64,
    pub compressed_size: u64, // only when compressed
}

// ----- MFT MftNodeAttributeType::FileName data -----

#[derive(Debug, Clone, Copy)]
#[repr(packed(1))]
pub struct MftNodeAttributeFileName {
    pub parent_directory: INodeMeta,
    pub creation_time: u64,
    pub change_time: u64,
    pub last_write_time: u64,
    pub last_access_time: u64,
    pub allocated_size: u64,
    pub data_size: u64,
    pub file_attributes: u32,
    _padding: u32,
    pub name_length: u8,
    /// NTFS = 0x01, DOS = 0x02
    pub name_type: u8,
    pub name: char,
}

// ----- MFT MftNodeAttributeType::StandardInformation data -----

#[derive(Debug, Clone, Copy)]
#[repr(packed(1))]
pub struct MftNodeAttributeStandardInformation {
    pub creation_time: u64,
    pub file_change_time: u64,
    pub mft_change_time: u64,
    pub last_access_time: u64,
    /// READ_ONLY = 0x01, HIDDEN = 0x02, SYSTEM = 0x04, VOLUME_ID = 0x08, ARCHIVE = 0x20, DEVICE = 0x40
    pub file_attributes: u32,
    pub maximum_versions: u32,
    pub version_number: u32,
    pub class_id: u32,
    pub ownder_id: u32,
    pub security_id: u32,
    pub quota_charge: u64,
    pub usn: u64,
}
