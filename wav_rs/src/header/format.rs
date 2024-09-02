use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug)]
pub struct Format {
    format_bloc_id: [u8; 4], // Identifier, should be "fmt_"
    bloc_size: u32,          // Chunk size minus 8 bytes
    audio_format: u16,       // Audio format
    nbr_channels: u16,
    frequence: u32,     // Sample rate
    byte_per_sec: u32,  // Frequence * Byte per Bloc
    byte_per_bloc: u16, // NbrChannels * Bits per Sample / 8
    bits_per_sample: u16,
}
impl Format {
    pub fn from_slice(buf: [u8; 24]) -> std::io::Result<Format> {
        let fformat = Format {
            format_bloc_id: buf[0..4].try_into().unwrap(),
            bloc_size: u32::from_le_bytes(buf[4..8].try_into().unwrap()) - 8,
            audio_format: u16::from_le_bytes(buf[8..10].try_into().unwrap()),
            nbr_channels: u16::from_le_bytes(buf[10..12].try_into().unwrap()),
            frequence: u32::from_le_bytes(buf[12..16].try_into().unwrap()),
            byte_per_sec: u32::from_le_bytes(buf[16..20].try_into().unwrap()),
            byte_per_bloc: u16::from_le_bytes(buf[20..22].try_into().unwrap()),
            bits_per_sample: u16::from_le_bytes(buf[22..24].try_into().unwrap()),
        };

        assert_eq!(
            fformat.format_bloc_id,
            [0x66, 0x6D, 0x74, 0x20] as [u8; 4],
            "Format identifier should be 'fmt_', is: {:?}",
            fformat.format_bloc_id
        );
        // assert bytepersec = freq * byteperbloc
        // assert byteperbloc = nbrchan * bitspersampl / 8
        Ok(fformat)
    }
}

// pub fn parse_format(mut reader: BufReader<File>) -> std::io::Result<(FORMAT, BufReader<File>)> {
//     let mut master = Master {
//         file_format_id: [0; 4],
//         file_size: [0; 4],
//         file_type_bloc_id: [0; 4],
//     };
//     reader.read_exact(&mut master.file_type_bloc_id)?;
//     reader.read_exact(&mut master.file_size)?;
//     reader.read_exact(&mut master.file_format_id)?;

//     Ok((master, reader))
// }
