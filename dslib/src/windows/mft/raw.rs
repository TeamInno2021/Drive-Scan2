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
