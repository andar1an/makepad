use crate::font::ROMFont;
const _FONT:[u8;0x1800] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x03, 0xc0, 0x07, 0xe0, 0x07, 0xe0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x07, 0xe0, 0x07, 0xe0,
    0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x1c, 0x38, 0x1c, 0x38, 0x0c, 0x30, 0x0c, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1c, 0x38, 0x1c, 0x38, 0x1c, 0x38, 0x1c, 0x38, 0x7f, 0xfe, 0x7f, 0xfe, 0x1c, 0x38, 0x1c, 0x38, 0x1c, 0x38, 0x1c, 0x38,
    0x1c, 0x38, 0x1c, 0x38, 0x1c, 0x38, 0x1c, 0x38, 0x7f, 0xfe, 0x7f, 0xfe, 0x1c, 0x38, 0x1c, 0x38, 0x1c, 0x38, 0x1c, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x07, 0xe0, 0x07, 0xe0, 0x1e, 0x78, 0x1e, 0x78, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x00, 0x3c, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x07, 0xe0, 0x07, 0xe0,
    0x00, 0x78, 0x00, 0x78, 0x00, 0x3c, 0x00, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x1e, 0x78, 0x1e, 0x78, 0x07, 0xe0, 0x07, 0xe0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x78, 0x3c, 0x78, 0x00, 0xf0, 0x00, 0xf0, 0x01, 0xe0, 0x01, 0xe0,
    0x03, 0xc0, 0x03, 0xc0, 0x07, 0x80, 0x07, 0x80, 0x0f, 0x00, 0x0f, 0x00, 0x1e, 0x3c, 0x1e, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xc0, 0x07, 0xc0, 0x1e, 0xf0, 0x1e, 0xf0, 0x3c, 0x78, 0x3c, 0x78, 0x1e, 0xf0, 0x1e, 0xf0, 0x07, 0xc0, 0x07, 0xc0, 0x0f, 0x9e, 0x0f, 0x9e,
    0x3f, 0xfc, 0x3f, 0xfc, 0x78, 0xf8, 0x78, 0xf8, 0x78, 0x78, 0x78, 0x78, 0x3c, 0xfc, 0x3c, 0xfc, 0x0f, 0x9e, 0x0f, 0x9e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xe0, 0x01, 0xe0, 0x03, 0xc0, 0x03, 0xc0, 0x07, 0x80, 0x07, 0x80, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00,
    0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x07, 0x80, 0x07, 0x80, 0x03, 0xc0, 0x03, 0xc0, 0x01, 0xe0, 0x01, 0xe0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xe0, 0x01, 0xe0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0x78, 0x00, 0x78, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c,
    0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x78, 0x00, 0x78, 0x00, 0xf0, 0x00, 0xf0, 0x01, 0xe0, 0x01, 0xe0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x3c, 0x3c, 0x3c, 0x0f, 0xf0, 0x0f, 0xf0, 0x7f, 0xfe, 0x7f, 0xfe,
    0x0f, 0xf0, 0x0f, 0xf0, 0x3c, 0x3c, 0x3c, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x7f, 0xfe, 0x7f, 0xfe,
    0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0x80, 0x03, 0x80, 0x07, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0xfe, 0x7f, 0xfe,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x78, 0x00, 0x78, 0x00, 0xf0, 0x00, 0xf0, 0x01, 0xe0, 0x01, 0xe0, 0x03, 0xc0, 0x03, 0xc0,
    0x07, 0x80, 0x07, 0x80, 0x0f, 0x00, 0x0f, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xe0, 0x07, 0xe0, 0x1e, 0x78, 0x1e, 0x78, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x7c, 0x3c, 0x7c, 0x3c, 0xfc, 0x3c, 0xfc, 0x3d, 0xbc, 0x3d, 0xbc,
    0x3f, 0x3c, 0x3f, 0x3c, 0x3e, 0x3c, 0x3e, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x1e, 0x78, 0x1e, 0x78, 0x07, 0xe0, 0x07, 0xe0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x03, 0xc0, 0x0f, 0xc0, 0x0f, 0xc0, 0x3f, 0xc0, 0x3f, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0,
    0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x3f, 0xfc, 0x3f, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xe0, 0x0f, 0xe0, 0x3c, 0x78, 0x3c, 0x78, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x78, 0x00, 0x78, 0x00, 0xf0, 0x00, 0xf0,
    0x03, 0xc0, 0x03, 0xc0, 0x0f, 0x00, 0x0f, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x3c, 0x3c, 0x3c, 0x3c, 0x3f, 0xfc, 0x3f, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xf0, 0x0f, 0xf0, 0x3c, 0x3c, 0x3c, 0x3c, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x3c, 0x00, 0x3c, 0x03, 0xf0, 0x03, 0xf0,
    0x00, 0x3c, 0x00, 0x3c, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x3c, 0x3c, 0x3c, 0x3c, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xf0, 0x01, 0xf0, 0x03, 0xf0, 0x03, 0xf0, 0x07, 0xf0, 0x07, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x1e, 0xf0, 0x1e, 0xf0, 0x3c, 0xf0, 0x3c, 0xf0,
    0x3f, 0xfc, 0x3f, 0xfc, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x03, 0xfc, 0x03, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0xfe, 0x3f, 0xfe, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3f, 0xf0, 0x3f, 0xf0, 0x00, 0x3c, 0x00, 0x3c,
    0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x3c, 0x3c, 0x3c, 0x3c, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xf0, 0x07, 0xf0, 0x1e, 0x00, 0x1e, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3f, 0xf0, 0x3f, 0xf0,
    0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x3c, 0x1e, 0x3c, 0x07, 0xf0, 0x07, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0xfc, 0x3f, 0xfc, 0x3c, 0x3c, 0x3c, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x78, 0x00, 0x78, 0x00, 0xf0, 0x00, 0xf0, 0x01, 0xe0, 0x01, 0xe0,
    0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xf0, 0x07, 0xf0, 0x1e, 0x3c, 0x1e, 0x3c, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x3c, 0x1e, 0x3c, 0x07, 0xf0, 0x07, 0xf0,
    0x1e, 0x3c, 0x1e, 0x3c, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x3c, 0x1e, 0x3c, 0x07, 0xf0, 0x07, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xf0, 0x07, 0xf0, 0x1e, 0x3c, 0x1e, 0x3c, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x07, 0xfe, 0x07, 0xfe,
    0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x3c, 0x00, 0x3c, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x07, 0x80, 0x07, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xe0, 0x01, 0xe0, 0x03, 0xc0, 0x03, 0xc0, 0x07, 0x80, 0x07, 0x80, 0x0f, 0x00, 0x0f, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x3c, 0x00, 0x3c, 0x00,
    0x1e, 0x00, 0x1e, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x07, 0x80, 0x07, 0x80, 0x03, 0xc0, 0x03, 0xc0, 0x01, 0xe0, 0x01, 0xe0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0xfc, 0x3f, 0xfc, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x3f, 0xfc, 0x3f, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x80, 0x07, 0x80, 0x03, 0xc0, 0x03, 0xc0, 0x01, 0xe0, 0x01, 0xe0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0x78, 0x00, 0x78, 0x00, 0x3c, 0x00, 0x3c,
    0x00, 0x78, 0x00, 0x78, 0x00, 0xf0, 0x00, 0xf0, 0x01, 0xe0, 0x01, 0xe0, 0x03, 0xc0, 0x03, 0xc0, 0x07, 0x80, 0x07, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xe0, 0x07, 0xe0, 0x1e, 0x78, 0x1e, 0x78, 0x3c, 0x3c, 0x3c, 0x3c, 0x00, 0x78, 0x00, 0x78, 0x00, 0xf0, 0x00, 0xf0, 0x01, 0xe0, 0x01, 0xe0,
    0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xfc, 0x0f, 0xfc, 0x3c, 0x1e, 0x3c, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x79, 0xfe, 0x79, 0xfe, 0x7b, 0x8e, 0x7b, 0x8e, 0x7b, 0x8e, 0x7b, 0x8e,
    0x7b, 0x8e, 0x7b, 0x8e, 0x79, 0xfc, 0x79, 0xfc, 0x78, 0x00, 0x78, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x0f, 0xfc, 0x0f, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x03, 0xc0, 0x07, 0xe0, 0x07, 0xe0, 0x0f, 0xf0, 0x0f, 0xf0, 0x1e, 0x78, 0x1e, 0x78, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c,
    0x3f, 0xfc, 0x3f, 0xfc, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0xf0, 0x7f, 0xf0, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x3c, 0x1e, 0x3c, 0x1f, 0xf0, 0x1f, 0xf0,
    0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x3c, 0x1e, 0x3c, 0x7f, 0xf0, 0x7f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xf0, 0x07, 0xf0, 0x1e, 0x3c, 0x1e, 0x3c, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00,
    0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x3c, 0x1e, 0x3c, 0x07, 0xf0, 0x07, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0xf0, 0x7f, 0xf0, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e,
    0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x3c, 0x1e, 0x3c, 0x7f, 0xf0, 0x7f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0xfe, 0x7f, 0xfe, 0x1e, 0x0e, 0x1e, 0x0e, 0x1e, 0x06, 0x1e, 0x06, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x60, 0x1e, 0x60, 0x1f, 0xe0, 0x1f, 0xe0,
    0x1e, 0x60, 0x1e, 0x60, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x06, 0x1e, 0x06, 0x1e, 0x0e, 0x1e, 0x0e, 0x7f, 0xfe, 0x7f, 0xfe, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0xfe, 0x7f, 0xfe, 0x1e, 0x0e, 0x1e, 0x0e, 0x1e, 0x06, 0x1e, 0x06, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x60, 0x1e, 0x60, 0x1f, 0xe0, 0x1f, 0xe0,
    0x1e, 0x60, 0x1e, 0x60, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x7f, 0x80, 0x7f, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xf0, 0x07, 0xf0, 0x1e, 0x3c, 0x1e, 0x3c, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00,
    0x3c, 0x7e, 0x3c, 0x7e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x3e, 0x1e, 0x3e, 0x07, 0xf6, 0x07, 0xf6, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3f, 0xfe, 0x3f, 0xfe,
    0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xf0, 0x0f, 0xf0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0,
    0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xfe, 0x03, 0xfe, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78,
    0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x1f, 0xf0, 0x1f, 0xf0, 0x07, 0xc0, 0x07, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x3c, 0x7e, 0x3c, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0xf0, 0x1e, 0xf0, 0x1f, 0xe0, 0x1f, 0xe0, 0x1f, 0xc0, 0x1f, 0xc0, 0x1f, 0xc0, 0x1f, 0xc0,
    0x1f, 0xe0, 0x1f, 0xe0, 0x1e, 0xf0, 0x1e, 0xf0, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x3c, 0x1e, 0x3c, 0x7e, 0x1e, 0x7e, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0x80, 0x7f, 0x80, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00,
    0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x06, 0x1e, 0x06, 0x1e, 0x0e, 0x1e, 0x0e, 0x7f, 0xfe, 0x7f, 0xfe, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0x1e, 0x78, 0x1e, 0x7c, 0x3e, 0x7c, 0x3e, 0x7e, 0x7e, 0x7e, 0x7e, 0x7f, 0xfe, 0x7f, 0xfe, 0x7b, 0xde, 0x7b, 0xde, 0x79, 0x9e, 0x79, 0x9e,
    0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3e, 0x1e, 0x3e, 0x1e, 0x3f, 0x1e, 0x3f, 0x1e, 0x3f, 0x9e, 0x3f, 0x9e, 0x3d, 0xde, 0x3d, 0xde,
    0x3c, 0xfe, 0x3c, 0xfe, 0x3c, 0x7e, 0x3c, 0x7e, 0x3c, 0x3e, 0x3c, 0x3e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xf0, 0x07, 0xf0, 0x1e, 0x3c, 0x1e, 0x3c, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e,
    0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x3c, 0x1e, 0x3c, 0x07, 0xf0, 0x07, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0xf0, 0x7f, 0xf0, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x3c, 0x1e, 0x3c, 0x1f, 0xf0, 0x1f, 0xf0,
    0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x7f, 0x80, 0x7f, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xf0, 0x07, 0xf0, 0x1e, 0x3c, 0x1e, 0x3c, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e,
    0x3c, 0x1e, 0x3c, 0x1e, 0x3d, 0xde, 0x3d, 0xde, 0x3c, 0xfe, 0x3c, 0xfe, 0x1e, 0x7c, 0x1e, 0x7c, 0x07, 0xf8, 0x07, 0xf8, 0x00, 0x1c, 0x00, 0x1c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0xf0, 0x7f, 0xf0, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x3c, 0x1e, 0x3c, 0x1f, 0xf0, 0x1f, 0xf0,
    0x1f, 0xe0, 0x1f, 0xe0, 0x1e, 0xf0, 0x1e, 0xf0, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x3c, 0x1e, 0x3c, 0x7e, 0x1e, 0x7e, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xf0, 0x0f, 0xf0, 0x3c, 0x3c, 0x3c, 0x3c, 0x78, 0x1e, 0x78, 0x1e, 0x3c, 0x00, 0x3c, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x03, 0xc0, 0x03, 0xc0,
    0x00, 0xf0, 0x00, 0xf0, 0x00, 0x3c, 0x00, 0x3c, 0x78, 0x1e, 0x78, 0x1e, 0x3c, 0x3c, 0x3c, 0x3c, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0xfe, 0x7f, 0xfe, 0x73, 0xce, 0x73, 0xce, 0x63, 0xc6, 0x63, 0xc6, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0,
    0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e,
    0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x3c, 0x1e, 0x3c, 0x07, 0xf0, 0x07, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c,
    0x3c, 0x3c, 0x3c, 0x3c, 0x1e, 0x78, 0x1e, 0x78, 0x0f, 0xf0, 0x0f, 0xf0, 0x07, 0xe0, 0x07, 0xe0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e,
    0x79, 0x9e, 0x79, 0x9e, 0x7b, 0xde, 0x7b, 0xde, 0x7f, 0xfe, 0x7f, 0xfe, 0x3e, 0x7c, 0x3e, 0x7c, 0x1c, 0x38, 0x1c, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x1e, 0x78, 0x1e, 0x78, 0x0f, 0xf0, 0x0f, 0xf0, 0x07, 0xe0, 0x07, 0xe0, 0x03, 0xc0, 0x03, 0xc0,
    0x07, 0xe0, 0x07, 0xe0, 0x0f, 0xf0, 0x0f, 0xf0, 0x1e, 0x78, 0x1e, 0x78, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x1e, 0x78, 0x1e, 0x78, 0x0f, 0xf0, 0x0f, 0xf0,
    0x07, 0xe0, 0x07, 0xe0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0xfc, 0x3f, 0xfc, 0x38, 0x3c, 0x38, 0x3c, 0x30, 0x78, 0x30, 0x78, 0x00, 0xf0, 0x00, 0xf0, 0x01, 0xe0, 0x01, 0xe0, 0x03, 0xc0, 0x03, 0xc0,
    0x07, 0x80, 0x07, 0x80, 0x0f, 0x00, 0x0f, 0x00, 0x1e, 0x0c, 0x1e, 0x0c, 0x3c, 0x1c, 0x3c, 0x1c, 0x3f, 0xfc, 0x3f, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00,
    0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x07, 0x80, 0x07, 0x80, 0x03, 0xc0, 0x03, 0xc0,
    0x01, 0xe0, 0x01, 0xe0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0x78, 0x00, 0x78, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0,
    0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x03, 0xc0, 0x03, 0xc0, 0x07, 0xe0, 0x07, 0xe0, 0x0f, 0xf0, 0x0f, 0xf0, 0x1e, 0x78, 0x1e, 0x78, 0x3c, 0x3c, 0x3c, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7f, 0xff, 0x7f, 0xff, 0x00, 0x00, 0x00, 0x00,
    0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x01, 0xe0, 0x01, 0xe0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xe0, 0x0f, 0xe0, 0x00, 0x78, 0x00, 0x78,
    0x0f, 0xf8, 0x0f, 0xf8, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x0f, 0x9e, 0x0f, 0x9e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x3f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0x3c, 0x0f, 0x3c,
    0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x3c, 0xf8, 0x3c, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xf8, 0x0f, 0xf8, 0x3c, 0x1e, 0x3c, 0x1e,
    0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x1e, 0x3c, 0x1e, 0x0f, 0xf8, 0x0f, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xf8, 0x01, 0xf8, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x07, 0xf8, 0x07, 0xf8, 0x1e, 0x78, 0x1e, 0x78,
    0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x0f, 0x9e, 0x0f, 0x9e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xf8, 0x0f, 0xf8, 0x3c, 0x1e, 0x3c, 0x1e,
    0x3c, 0x1e, 0x3c, 0x1e, 0x3f, 0xfe, 0x3f, 0xfe, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x1e, 0x3c, 0x1e, 0x0f, 0xf8, 0x0f, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xf0, 0x03, 0xf0, 0x0f, 0x3c, 0x0f, 0x3c, 0x0f, 0x0c, 0x0f, 0x0c, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x3f, 0xf0, 0x3f, 0xf0,
    0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x3f, 0xc0, 0x3f, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x9e, 0x0f, 0x9e, 0x3c, 0x78, 0x3c, 0x78,
    0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x0f, 0xf8, 0x0f, 0xf8, 0x00, 0x78, 0x00, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x0f, 0xe0, 0x0f, 0xe0, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x3f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x78, 0x0f, 0x78, 0x0f, 0x9e, 0x0f, 0x9e,
    0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x3f, 0x1e, 0x3f, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x03, 0xf0, 0x03, 0xf0, 0x00, 0xf0, 0x00, 0xf0,
    0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x03, 0xfc, 0x03, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfc, 0x00, 0xfc, 0x00, 0x3c, 0x00, 0x3c,
    0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x00, 0x3c, 0x3c, 0x3c, 0x3c, 0x3c, 0x1e, 0x78, 0x1e, 0x78, 0x07, 0xe0, 0x07, 0xe0, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x3f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x3c, 0x0f, 0x3c,
    0x0f, 0x78, 0x0f, 0x78, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0x78, 0x0f, 0x78, 0x0f, 0x3c, 0x0f, 0x3c, 0x3f, 0x1e, 0x3f, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xf0, 0x03, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0,
    0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x00, 0xf0, 0x03, 0xfc, 0x03, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x7e, 0x7c, 0x7e, 0x7c, 0x7f, 0xfe, 0x7f, 0xfe,
    0x7b, 0xde, 0x7b, 0xde, 0x7b, 0xde, 0x7b, 0xde, 0x7b, 0xde, 0x7b, 0xde, 0x7b, 0xde, 0x7b, 0xde, 0x7b, 0xde, 0x7b, 0xde, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0xf8, 0x3c, 0xf8, 0x0f, 0x3c, 0x0f, 0x3c,
    0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0xf0, 0x07, 0xf0, 0x1e, 0x3c, 0x1e, 0x3c,
    0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x3c, 0x1e, 0x3c, 0x07, 0xf0, 0x07, 0xf0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0xf0, 0x3c, 0xf0, 0x0f, 0x3c, 0x0f, 0x3c,
    0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x1e, 0x0f, 0x3c, 0x0f, 0x3c, 0x0f, 0xf0, 0x0f, 0xf0, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x3f, 0xc0, 0x3f, 0xc0, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x07, 0x9e, 0x07, 0x9e, 0x1e, 0x78, 0x1e, 0x78,
    0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x07, 0xf8, 0x07, 0xf8, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x00, 0x78, 0x00, 0xfe, 0x00, 0xfe, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0xf8, 0x3c, 0xf8, 0x0f, 0x9e, 0x0f, 0x9e,
    0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x0f, 0x00, 0x3f, 0xc0, 0x3f, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0xf8, 0x0f, 0xf8, 0x3c, 0x1e, 0x3c, 0x1e,
    0x3c, 0x00, 0x3c, 0x00, 0x0f, 0xf8, 0x0f, 0xf8, 0x00, 0x1e, 0x00, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x0f, 0xf8, 0x0f, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x80, 0x01, 0x80, 0x03, 0x80, 0x03, 0x80, 0x07, 0x80, 0x07, 0x80, 0x07, 0x80, 0x07, 0x80, 0x7f, 0xf8, 0x7f, 0xf8, 0x07, 0x80, 0x07, 0x80,
    0x07, 0x80, 0x07, 0x80, 0x07, 0x80, 0x07, 0x80, 0x07, 0x80, 0x07, 0x80, 0x07, 0x9e, 0x07, 0x9e, 0x01, 0xf8, 0x01, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78,
    0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x3c, 0x78, 0x0f, 0x9e, 0x0f, 0x9e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e,
    0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x1e, 0x78, 0x1e, 0x78, 0x07, 0xe0, 0x07, 0xe0, 0x01, 0x80, 0x01, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e, 0x78, 0x1e,
    0x78, 0x1e, 0x78, 0x1e, 0x79, 0x9e, 0x79, 0x9e, 0x7b, 0xde, 0x7b, 0xde, 0x3f, 0xfc, 0x3f, 0xfc, 0x1e, 0x78, 0x1e, 0x78, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x3c, 0x3c, 0x3c, 0x0e, 0x70, 0x0e, 0x70,
    0x07, 0xe0, 0x07, 0xe0, 0x03, 0xc0, 0x03, 0xc0, 0x07, 0xe0, 0x07, 0xe0, 0x0e, 0x70, 0x0e, 0x70, 0x3c, 0x3c, 0x3c, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e,
    0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x3c, 0x1e, 0x1e, 0x1e, 0x1e, 0x1e, 0x07, 0xfe, 0x07, 0xfe, 0x00, 0x1e, 0x00, 0x1e, 0x00, 0x3c, 0x00, 0x3c, 0x0f, 0xf0, 0x0f, 0xf0, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0xfc, 0x3f, 0xfc, 0x3c, 0x3c, 0x3c, 0x3c,
    0x00, 0xf0, 0x00, 0xf0, 0x03, 0xc0, 0x03, 0xc0, 0x0f, 0x00, 0x0f, 0x00, 0x3c, 0x3c, 0x3c, 0x3c, 0x3f, 0xfc, 0x3f, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfc, 0x00, 0xfc, 0x01, 0xe0, 0x01, 0xe0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x3f, 0x80, 0x3f, 0x80,
    0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x01, 0xe0, 0x01, 0xe0, 0x00, 0xfc, 0x00, 0xfc, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00,
    0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3f, 0x00, 0x3f, 0x00, 0x07, 0x80, 0x07, 0x80, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x01, 0xfc, 0x01, 0xfc,
    0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x07, 0x80, 0x07, 0x80, 0x3f, 0x00, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0f, 0x9e, 0x0f, 0x9e, 0x3c, 0xf8, 0x3c, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, 0x00, 0xc0, 0x03, 0xf0, 0x03, 0xf0, 0x0f, 0x3c, 0x0f, 0x3c, 0x3c, 0x0f, 0x3c, 0x0f,
    0x3c, 0x0f, 0x3c, 0x0f, 0x3f, 0xff, 0x3f, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub const VGA1_BOLD_16X32:ROMFont = ROMFont::new(&_FONT, 32, 16, (32 as char)..(127 as char));

