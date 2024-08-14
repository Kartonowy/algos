use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Read;

pub mod application;
pub mod cuesheet;
pub mod header;
pub mod padding;
pub mod picture;
pub mod seektable;
pub mod streaminfo;
pub mod vorbis_comment;

use crate::metadata::padding::resolve_padding;
use crate::metadata::seektable::resolve_seektable;
use crate::metadata::seektable::SEEKPOINT;
use crate::metadata::{application::resolve_application, vorbis_comment::resolve_vorbis_comment};
use crate::metadata::{cuesheet::resolve_cuesheet, header::read_header};
use crate::metadata::{picture::resolve_picture, streaminfo::resolve_streaminfo};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Blocktype {
    STREAMINFO {
        min_block_size: u16,
        max_block_size: u16,
        min_frame_size: u32,
        max_frame_size: u32,
        sample_rate: u32,
        n_of_channels: u8,
        bits_per_sample: u8,
        total_samples: u64,
        md5_signature: u128,
    },
    PADDING {
        padding_length: usize,
    },
    APPLICATION,
    SEEKTABLE {
        seekpoints: Vec<SEEKPOINT>,
    },
    VORBIS_COMMENT {
        vendor_length: u32,
        vendor_string: String,
        user_comment_list_length: u32,
        user_comments: Vec<String>,
    },
    CUESHEET,
    PICTURE,
    OTHER,
}

#[derive(Debug)]
pub struct Block {
    is_last: bool,
    length: usize,
    block_type: Blocktype,
}

impl Block {
    pub fn new(is_last: bool, length: usize) -> Block {
        Block {
            is_last,
            length,
            block_type: Blocktype::OTHER,
        }
    }

    pub fn set_type(&mut self, block_type: Blocktype) {
        self.block_type = block_type
    }
}

pub fn collect_metadata(
    mut reader: BufReader<File>,
) -> Result<(Vec<Block>, BufReader<File>), Error> {
    let mut is_last = false;

    //superimportanto
    let mut METADATA: Vec<Block> = vec![];
    while !is_last {
        //META_HEADER
        let mut buffer = [0; 4];
        let _ = reader.read_exact(&mut buffer)?;
        let meta_header = read_header(&buffer);
        if meta_header.is_last {
            is_last = true
        }

        let mut buffer = vec![0; meta_header.length];
        let _ = reader.read_exact(&mut buffer)?;
        let mut current_block = Block::new(is_last, meta_header.length);
        match meta_header.block_type {
            0 => current_block.set_type(resolve_streaminfo(&buffer)),
            1 => current_block.set_type(resolve_padding(&buffer)),
            2 => current_block.set_type(resolve_application(&buffer)),
            3 => current_block.set_type(resolve_seektable(&buffer, meta_header.length)),
            4 => current_block.set_type(resolve_vorbis_comment(&buffer)),
            5 => current_block.set_type(resolve_cuesheet(&buffer)),
            6 => current_block.set_type(resolve_picture(&buffer)),
            7..=126 => {}
            127 => unreachable!(),
            _ => {}
        }
        METADATA.push(current_block);
    }
    Ok((METADATA, reader))
}
