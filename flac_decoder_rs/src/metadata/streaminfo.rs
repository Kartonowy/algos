use super::Blocktype::{self, STREAMINFO};

pub fn resolve_streaminfo(buf: &[u8]) -> Blocktype {
    let mut cursor: usize = 0;

    let min_block_size = u16::from(buf[cursor]) << 8 | u16::from(buf[cursor + 1]);
    cursor += 2;
    let max_block_size = u16::from(buf[cursor]) << 8 | u16::from(buf[cursor + 1]);
    cursor += 2;
    assert_eq!(max_block_size, min_block_size);

    let min_frame_size =
        u32::from(buf[cursor]) << 16 | u32::from(buf[cursor + 1]) << 8 | u32::from(buf[cursor + 2]);
    cursor += 3;
    let max_frame_size =
        u32::from(buf[cursor]) << 16 | u32::from(buf[cursor + 1]) << 8 | u32::from(buf[cursor + 2]);
    cursor += 3;

    let sample_rate = u32::from(buf[cursor]) << 12
        | u32::from(buf[cursor + 1]) << 4
        | (u32::from(buf[cursor + 2]) >> 4);
    cursor += 2;

    let n_of_channels: u8 = u8::from(buf[cursor] >> 1 & 7) + 1;

    let bits_per_sample = ((buf[cursor] & 1) << 4 | buf[cursor + 1] >> 4) + 1;
    cursor += 1;

    let total_samples = (u64::from(buf[cursor]) & 0xF) << 32
        | u64::from(buf[cursor + 1]) << 24
        | u64::from(buf[cursor + 2]) << 16
        | u64::from(buf[cursor + 3]) << 8
        | u64::from(buf[cursor + 4]);
    cursor += 5;

    let mut md5_signature: u128 = 0;
    for i in buf[cursor..cursor + 16usize].iter() {
        md5_signature = md5_signature << 8 | u128::from(*i);
    }
    Blocktype::STREAMINFO {
        min_block_size,
        max_block_size,
        min_frame_size,
        max_frame_size,
        sample_rate,
        n_of_channels,
        bits_per_sample,
        total_samples,
        md5_signature,
    }
}
