use num_derive::FromPrimitive;

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

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum MftNodeAttributes {
    ReadOnly = 1 << 0,
    Hidden = 1 << 1,
    System = 1 << 2,
    Directory = 1 << 3,
    Archive = 1 << 4,
    Device = 1 << 5,
    Normal = 1 << 6,
    Temporary = 1 << 7,
    SparseFile = 1 << 8,
    ReparsePoint = 1 << 9,
    Compressed = 1 << 10,
    Offline = 1 << 11,
    NotContentIndexed = 1 << 12,
    Encrypted = 1 << 13,
}

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, Eq)]
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
    pub starting_vcn: u64,
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
    pub name: [u8; winapi::shared::minwindef::MAX_PATH], // todo read up to name_type then read name_length bytes manually
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
