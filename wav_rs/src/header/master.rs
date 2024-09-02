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
    pub fn from_bufreader(
        mut reader: BufReader<File>,
    ) -> std::io::Result<(Master, BufReader<File>)> {
        let mut buf2 = [0; 4];
        let mut master = Master {
            file_format_id: [0; 4],
            file_size: 0,
            file_type_bloc_id: [0; 4],
        };
        reader.read_exact(&mut master.file_type_bloc_id)?;
        reader.read_exact(&mut buf2)?;
        reader.read_exact(&mut master.file_format_id)?;
        master.file_size = u32::from_le_bytes(buf2) - 8;

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
        Ok((master, reader))
    }
}
