use std::{
    fs::File,
    io::{BufReader, Read},
};

#[derive(Debug)]
pub struct FORMAT {
    format_bloc_id: [u8; 4],
    bloc_size: [u8; 4],
    audio_format: [u8; 2],
    nbr_channels: [u8; 2],
    frequence: [u8; 4],
    byte_per_sec: [u8; 4],
    byte_per_bloc: [u8; 2],
    bits_per_sample: [u8; 2],
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
// pub fn parse_format(buf: &[u8; 24]) -> FORMAT {
//     FORMAT {
//         format_bloc_id: buf[0..=3].try_into().expect("something failed in format"),
//         bloc_size: buf,
//         audio_format: (),
//         nbr_channels: (),
//         frequence: (),
//         byte_per_sec: (),
//         byte_per_bloc: (),
//         bits_per_sample: (),
//     }
// }
