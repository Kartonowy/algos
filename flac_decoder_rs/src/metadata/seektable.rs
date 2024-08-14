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
