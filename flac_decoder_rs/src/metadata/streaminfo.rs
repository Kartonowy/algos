pub fn resolve_streaminfo(buf: &[u8]) {
    let mut cursor: usize = 0;

    let min_block_size = u16::from(buf[cursor]) << 8 | u16::from(buf[cursor + 1]);
    cursor += 2;
    let max_block_size = u16::from(buf[cursor]) << 8 | u16::from(buf[cursor + 1]);
    cursor += 2;
    assert_eq!(max_block_size, min_block_size);
    println!("min_size: {}", min_block_size);
    println!("max_size: {}", max_block_size);

    let min_frame_size =
        u32::from(buf[cursor]) << 16 | u32::from(buf[cursor + 1]) << 8 | u32::from(buf[cursor + 2]);
    cursor += 3;
    let max_frame_size =
        u32::from(buf[cursor]) << 16 | u32::from(buf[cursor + 1]) << 8 | u32::from(buf[cursor + 2]);
    cursor += 3;
    println!("min_frame_size: {}", min_frame_size);
    println!("max_frame_size: {}", max_frame_size);

    let sample_rate = u32::from(buf[cursor]) << 12
        | u32::from(buf[cursor + 1]) << 4
        | (u32::from(buf[cursor + 2]) >> 4);
    cursor += 2;
    println!("sample_rate: {}", sample_rate);

    let n_of_channels: u8 = u8::from(buf[cursor] >> 1 & 7);
    println!("n_of_channels: {}", n_of_channels + 1);
    let bits_per_sample = (buf[cursor] & 1) << 4 | buf[cursor + 1] >> 4;
    cursor += 1;
    println!("bits_per_sample: {}", bits_per_sample + 1);

    let n_samples = (u64::from(buf[cursor]) & 0xF) << 32
        | u64::from(buf[cursor + 1]) << 24
        | u64::from(buf[cursor + 2]) << 16
        | u64::from(buf[cursor + 3]) << 8
        | u64::from(buf[cursor + 4]);
    println!("n_samples: {}", n_samples);
    cursor += 5;

    let mut md5_signature: u128 = 0;
    for i in buf[cursor..cursor + 16usize].iter() {
        md5_signature = md5_signature << 8 | u128::from(*i);
    }
    println!("MD5 signature: {:#x}", md5_signature);
}
