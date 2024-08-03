// use std::{
//     fs::File,
//     io::{BufReader, Read},
// };

// pub fn read_n_u32<R>(reader: R, bits_to_read: u64) -> u32
// where
//     R: Read,
// {
//     let mut n: u32 = 0;
//     let mut offset = bits_to_read - 8;
//     let mut buf: [u8; 1];

//     while offset > 0 {
//         reader.read_exact(&mut buf);
//         n = n << offset | u32::from(buf[0])
//     }

//     n
// }

// pub fn read_n_u64<R>(reader: R, bytes_to_read: u64) -> u64
// where
//     R: Read,
// {
//     let mut buf: u64 = 0;
//     let mut chunk = reader.take(bytes_to_read);

//     let n = chunk.read_to_end(&mut buf).expect("Couldn't read enough");

//     while bytes_to_read

//     assert_eq!(bytes_to_read as usize, n);
//     u64::from(buf[0]) << 8 | u64::from(buf[1])
// }

// pub fn read_n_u16<R>(reader: R, bytes_to_read: u64) -> u16
// where
//     R: Read,
// {
//     let mut buf = vec![];
//     let mut chunk = reader.take(bytes_to_read);

//     let n = chunk.read_to_end(&mut buf).expect("Couldn't read enough");

//     assert_eq!(bytes_to_read as usize, n);
//     u16::from(buf[0]) << 8 | u16::from(buf[1])
// }

// pub fn read_u8<R>(reader: R) -> u8
// where
//     R: Read,
// {
//     let mut buf = vec![];
//     let mut chunk = reader.take(1);

//     let n = chunk.read_to_end(&mut buf).expect("Couldn't read enough");

//     assert_eq!(1usize, n);
//     buf[0]
// }
