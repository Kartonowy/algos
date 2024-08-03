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

pub fn resolve_vorbis_comment(buf: &[u8]) {
    let mut cursor = 0;
    let vendor_length = u32::from(buf[cursor + 3]) << 24
        | u32::from(buf[cursor + 2]) << 16
        | u32::from(buf[cursor + 1]) << 8
        | u32::from(buf[cursor]);
    // i really hate the fact that this is little endian
    // instead of big endian like everywhere here
    println!("vendor length: {}", vendor_length);
    cursor += 4;

    let mut vendor_string: Vec<u8> = vec![];
    for i in &buf[cursor..cursor + vendor_length as usize] {
        vendor_string.push(*i)
    }
    println!(
        "vendor string: {}",
        String::from_utf8(vendor_string).expect("Couldnt get vendor_string")
    );
    cursor += vendor_length as usize;

    let user_comment_list_length: u32 = u32::from(buf[cursor + 3]) << 24
        | u32::from(buf[cursor + 2]) << 16
        | u32::from(buf[cursor + 1]) << 8
        | u32::from(buf[cursor]);
    println!("user comment list length: {}", user_comment_list_length);
    cursor += 4; // 4 bytes read
    for i in 0..user_comment_list_length {
        let length: usize = usize::from(buf[cursor + 3]) << 24
            | usize::from(buf[cursor + 2]) << 16
            | usize::from(buf[cursor + 1]) << 8
            | usize::from(buf[cursor]);
        cursor += 4;
        println!(
            "comment[{i}]: {}",
            String::from_utf8(Vec::from(&buf[cursor..cursor + length]))
                .expect("Somehow failed to convert Vec<u8> to String")
        );
        cursor += length;
    }
}

pub fn resolve_padding(buf: &[u8]) {
    let length = buf.len();
    assert!(length % 8 == 0, "Padding must be a multiple of 8");
    println!("length: {}", buf.len());
}

pub fn resolve_seektable(buf: &[u8], length: usize) {
    let seek_points = length / 18;
    println!("seek points: {}", seek_points);
    let mut cursor = 0;
    for i in 0..seek_points {
        print!("point {i}: ");
        let sample_number = u64::from(buf[cursor]) << 56
            | u64::from(buf[cursor + 1]) << 48
            | u64::from(buf[cursor + 2]) << 40
            | u64::from(buf[cursor + 3]) << 32
            | u64::from(buf[cursor + 4]) << 24
            | u64::from(buf[cursor + 5]) << 16
            | u64::from(buf[cursor + 6]) << 8
            | u64::from(buf[cursor + 7]);
        cursor += 8;
        print!("sample number: {},", sample_number);

        let offset = u64::from(buf[cursor]) << 56
            | u64::from(buf[cursor + 1]) << 48
            | u64::from(buf[cursor + 2]) << 40
            | u64::from(buf[cursor + 3]) << 32
            | u64::from(buf[cursor + 4]) << 24
            | u64::from(buf[cursor + 5]) << 16
            | u64::from(buf[cursor + 6]) << 8
            | u64::from(buf[cursor + 7]);
        cursor += 8;
        print!("offset: {offset},");

        let n_samples_in_frame: u16 = u16::from(buf[cursor]) << 8 | u16::from(buf[cursor + 1]);
        print!("frame_samples: {}\n", n_samples_in_frame);
        cursor += 2;
    }
}

pub fn resolve_picture(buf: &[u8]) {}
