pub fn resolve_padding(buf: &[u8]) {
    let length = buf.len();
    assert!(length % 8 == 0, "Padding must be a multiple of 8");
    println!("length: {}", buf.len());
}
