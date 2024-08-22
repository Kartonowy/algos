pub mod header;

struct FRAME {
    header: FRAME_HEADER,
    subframe: (),
    footer: FRAME_FOOTER,
}

struct FRAME_HEADER {}

struct FRAME_FOOTER {}
