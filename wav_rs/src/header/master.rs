use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug)]
pub struct Master {
    file_type_bloc_id: [u8; 4],
    file_size: u32,
    file_format_id: [u8; 4],
}
impl Master {
    pub fn from_slice(buf: &[u8; 12]) -> std::io::Result<Master> {
        let mut master = Master {
            file_type_bloc_id: buf[0..4].try_into().unwrap(),
            file_size: u32::from_le_bytes(buf[4..8].try_into().unwrap()),
            file_format_id: buf[8..12].try_into().unwrap(),
        };
        assert_eq!(
            master.file_type_bloc_id,
            [0x52, 0x49, 0x46, 0x46] as [u8; 4],
            "File type bloc id should be 'RIFF', is {:?}",
            master.file_type_bloc_id
        );

        assert_eq!(
            master.file_format_id,
            [0x57, 0x41, 0x56, 0x45] as [u8; 4],
            "File format id should be 'WAVE', is {:?}",
            master.file_format_id
        );
        Ok(master)
    }
}
