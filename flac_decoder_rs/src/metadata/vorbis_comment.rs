use crate::metadata::Blocktype;

#[allow(non_camel_case_types)]
pub fn resolve_vorbis_comment(buf: &[u8]) -> Blocktype {
    let mut cursor = 0;
    let vendor_length = u32::from(buf[cursor + 3]) << 24
        | u32::from(buf[cursor + 2]) << 16
        | u32::from(buf[cursor + 1]) << 8
        | u32::from(buf[cursor]);
    // i really hate the fact that this is little endian
    // instead of big endian like everywhere here
    cursor += 4;

    let mut vendor_string: String = "".to_string();
    for i in &buf[cursor..cursor + vendor_length as usize] {
        vendor_string.push(*i as char)
    }
    cursor += vendor_length as usize;

    let user_comment_list_length: u32 = u32::from(buf[cursor + 3]) << 24
        | u32::from(buf[cursor + 2]) << 16
        | u32::from(buf[cursor + 1]) << 8
        | u32::from(buf[cursor]);
    cursor += 4; // 4 bytes read
    let mut user_comments: Vec<String> = vec![];
    for i in 0..user_comment_list_length {
        let length: usize = usize::from(buf[cursor + 3]) << 24
            | usize::from(buf[cursor + 2]) << 16
            | usize::from(buf[cursor + 1]) << 8
            | usize::from(buf[cursor]);
        cursor += 4;
        user_comments.push(String::from_utf8_lossy(&buf[cursor..cursor + length]).to_string());
        cursor += length;
    }
    Blocktype::VORBIS_COMMENT {
        vendor_length,
        vendor_string,
        user_comment_list_length,
        user_comments,
    }
}
