#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::{
    fs::{read, File},
    io::{BufReader, Error, ErrorKind, Read},
};

mod metadata;
use frame::header;
use metadata::collect_metadata;

mod frame;

fn main() -> std::io::Result<()> {
    let f = File::open("shojo.flac")?;

    let mut reader = BufReader::new(f);

    let mut buf = [0; 4];

    let _ = reader.read(&mut buf);
    assert_eq!(&String::from_utf8_lossy(&buf), "fLaC", "not a flac file");
    let (METADATA, mut reader) = collect_metadata(reader)?;
    println!("{:#?}", METADATA);

    let mut buf = [0; 2];
    let _ = reader.read(&mut buf)?;

    let sync_code = u16::from(buf[0]) << 6 | (u16::from(buf[1]) >> 2);
    print!("Sync code: {:b}", sync_code);
    assert_eq!(sync_code, 0b11111111111110);

    let reserved = bool::from(buf[1] << 6 >> 1);
    Ok(())
}
