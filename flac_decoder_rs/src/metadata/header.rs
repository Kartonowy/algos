pub fn read_header(buffer: &[u8; 4]) -> METADATA_BLOCK_HEADER {
    let is_last = buffer[0] & 128 != 0;
    let block_type = buffer[0] & 127;

    let length: usize =
        usize::from(buffer[1]) << 16 | usize::from(buffer[2]) << 8 | usize::from(buffer[3]);

    METADATA_BLOCK_HEADER {
        is_last,
        block_type,
        length,
    }
}

#[allow(non_camel_case_types)]
pub struct METADATA_BLOCK_HEADER {
    pub is_last: bool,
    pub block_type: u8,
    pub length: usize,
}
