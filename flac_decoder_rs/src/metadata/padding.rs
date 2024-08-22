use super::Blocktype;

pub fn resolve_padding(buf: &[u8]) -> Blocktype {
    let length = buf.len();
    assert!(length % 8 == 0, "Padding must be a multiple of 8");
    Blocktype::PADDING {
        padding_length: length,
    }
}
