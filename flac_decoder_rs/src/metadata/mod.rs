pub mod application;
pub mod cuesheet;
pub mod padding;
pub mod picture;
pub mod seektable;
pub mod streaminfo;
pub mod vorbis_comment;
use seektable::SEEKPOINT;

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
