use super::Blocktype;

#[derive(Debug)]
pub struct SEEKPOINT {
    sample_number: u64,
    offset: u64,
    frame_samples: u16,
}

// TODO: Placeholder points
pub fn resolve_seektable(buf: &[u8], length: usize) -> Blocktype {
    let seek_points = length / 18;
    let mut cursor = 0;
    let mut table: Vec<SEEKPOINT> = vec![];
    for i in 0..seek_points {
        let sample_number = u64::from(buf[cursor]) << 56
            | u64::from(buf[cursor + 1]) << 48
            | u64::from(buf[cursor + 2]) << 40
            | u64::from(buf[cursor + 3]) << 32
            | u64::from(buf[cursor + 4]) << 24
            | u64::from(buf[cursor + 5]) << 16
            | u64::from(buf[cursor + 6]) << 8
            | u64::from(buf[cursor + 7]);
        cursor += 8;

        let offset = u64::from(buf[cursor]) << 56
            | u64::from(buf[cursor + 1]) << 48
            | u64::from(buf[cursor + 2]) << 40
            | u64::from(buf[cursor + 3]) << 32
            | u64::from(buf[cursor + 4]) << 24
            | u64::from(buf[cursor + 5]) << 16
            | u64::from(buf[cursor + 6]) << 8
            | u64::from(buf[cursor + 7]);
        cursor += 8;

        let frame_samples: u16 = u16::from(buf[cursor]) << 8 | u16::from(buf[cursor + 1]);
        cursor += 2;
        table.push(SEEKPOINT {
            sample_number,
            offset,
            frame_samples,
        })
    }
    Blocktype::SEEKTABLE { seekpoints: table }
}
