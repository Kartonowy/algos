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
