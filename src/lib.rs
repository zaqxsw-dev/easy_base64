use std::iter::repeat;

const BASE64CHARS: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const BASE64DECODE_TABLE: [u32; 128] = [
    80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80,
    80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 80, 62, 80, 80, 80, 63,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 80, 80, 80, 64, 80, 80, 80, 0, 1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 80, 80, 80, 80, 80, 80, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 80, 80, 80, 80, 80,
];
const ENCODE_MASKS: [u32; 4] = [
    0b11111100_00000000_00000000_00000000,
    0b00000011_11110000_00000000_00000000,
    0b00000000_00001111_11000000_00000000,
    0b00000000_00000000_00111111_00000000,
];
const ENCODE_SHIFTS: [u32; 4] = [26, 20, 14, 8];
const DECODE_MASKS: [u32; 3] = [
    0b00111111_11000000_00000000_00000000,
    0b00000000_00111111_11000000_00000000,
    0b00000000_00000000_00111111_11000000,
];
const DECODE_SHIFTS: [u32; 3] = [22, 14, 6];

pub fn encode(input: &[u8]) -> String {
    let end_string_correction_pad = input.len() % 3;
    let padding_len = if end_string_correction_pad > 0 {
        3 - end_string_correction_pad
    } else {
        0
    };
    let mut i = 0;
    let mut output = String::new();

    while i < input.len() {
        let b1 = *input.get(i).unwrap_or(&0) as u32;
        let b2 = *input.get(i + 1).unwrap_or(&0) as u32;
        let b3 = *input.get(i + 2).unwrap_or(&0) as u32;
        let val: u32 = b1 << 24 | b2 << 16 | b3 << 8;

        let b64_size = match (b2, b3) {
            (0, 0) => 2,
            (_, 0) => 3,
            _ => 4,
        };
        for (mask, shift) in ENCODE_MASKS.iter().zip(ENCODE_SHIFTS.iter()).take(b64_size) {
            output.push(BASE64CHARS[((val & mask) >> shift) as usize] as char);
        }
        i += 3;
    }

    output.extend(repeat('=').take(padding_len));

    return output;
}

pub fn decode(input: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut i = 0;

    while i < input.len() {
        let b1 = *input.get(i).unwrap_or(&0) as usize;
        let b2 = *input.get(i + 1).unwrap_or(&0) as usize;
        let b3 = *input.get(i + 2).unwrap_or(&0) as usize;
        let b4 = *input.get(i + 3).unwrap_or(&0) as usize;
        let val: u32 = BASE64DECODE_TABLE[b1] << 24
            | BASE64DECODE_TABLE[b2] << 18
            | BASE64DECODE_TABLE[b3] << 12
            | BASE64DECODE_TABLE[b4] << 6;

        let b64_size = match (b3, b4) {
            (0, 0) => 1,
            (61, 61) => 1,
            (_, 0) => 2,
            (_, 61) => 2,
            _ => 3,
        };

        for (mask, shift) in DECODE_MASKS.iter().zip(DECODE_SHIFTS.iter()).take(b64_size) {
            output.push(((val & mask) >> shift) as u8);
        }

        i += 4;
    }

    return output;
}
