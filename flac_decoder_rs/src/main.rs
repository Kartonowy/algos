use std::{
    fs::File,
    io::{BufReader, Read},
};

mod metadata;
use metadata::seektable::resolve_seektable;
use metadata::streaminfo::resolve_streaminfo;
use metadata::vorbis_comment::resolve_vorbis_comment;
use metadata::Blocktype::*;
use metadata::{padding::resolve_padding, Block};

mod frame;

fn main() {
    let nokotan = File::open("shojo.flac").unwrap();

    let mut reader = BufReader::new(nokotan);

    let mut buf = vec![];
    let _ = reader.read_to_end(&mut buf);
    let mut cursor = 0;

    assert_eq!(
        &String::from_utf8(buf[cursor..cursor + 4].to_vec()).unwrap(),
        "fLaC",
        "not a flac file"
    );
    cursor += 4;

    let mut is_last = false;
    let mut n_metablock = 0;

    //superimportanto
    let mut METADATA: Vec<Block> = vec![];

    while !is_last {
        is_last = buf[cursor] & 128 != 0;
        let block_type = buf[cursor] & 127;
        cursor += 1;

        let meta_len: usize = usize::from(buf[cursor]) << 16
            | usize::from(buf[cursor + 1]) << 8
            | usize::from(buf[cursor + 2]);
        cursor += 3;

        println!("METADATA block #{n_metablock}");
        println!("  is last: {}", is_last);
        println!("  length: {}", meta_len);
        n_metablock += 1;

        print!("  type {block_type}: ");
        let mut current_block = Block::new(is_last, meta_len);
        match block_type {
            0 => {
                println!("STREAMINFO");
                current_block.set_type(resolve_streaminfo(&buf[cursor..cursor + meta_len]))
            }
            1 => {
                println!("PADDING");
                current_block.set_type(resolve_padding(&buf[cursor..cursor + meta_len]))
            }
            2 => println!("APPLICATION"),
            3 => {
                println!("SEEKTABLE");
                current_block.set_type(resolve_seektable(&buf[cursor..cursor + meta_len], meta_len))
            }
            4 => {
                println!("VORBIS_COMMENT");
                current_block.set_type(resolve_vorbis_comment(&buf[cursor..cursor + meta_len]))
            }
            5 => println!("CUESHEET"),
            6 => println!("PICTURE"),
            7..=126 => println!("reserved"),
            127 => unreachable!(),
            _ => {}
        }
        METADATA.push(current_block);
        cursor += meta_len as usize;
    }
    println!("{:#?}", METADATA)
}
