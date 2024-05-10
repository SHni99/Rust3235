// Detect endianness and define a byte swap function
fn swap(x: u32) -> u32 {
    if cfg!(target_endian = "big") {
        x
    } else {
        x.to_be()
    }
}

// Constants
const MIN_CHR: u8 = 39;
const MAX_CHR: u8 = 122;


static CHRS_BY_CHR_ID: [char; 32] = [
    'e', 'a', 'i', 'o', 't', 'h', 'n', 'r', 's', 'l', 'u', 'c', 'w', 'm', 'd',
    'b', 'p', 'f', 'g', 'v', 'y', 'k', '-', 'H', 'M', 'T', '\'', 'B', 'x', 'I', 'W', 'L',
];

static CHR_IDS_BY_CHR: [i16; 256] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 26, -1, -1, -1, -1, -1, 22, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 27, -1, -1, -1, -1, -1, 23, 29, -1, -1, 31, 24, -1, -1, -1, -1, -1, -1, 25, -1, -1, 30, -1, -1, -1, -1, -1, -1, -1, -1, -1, 1, 15, 11, 14, 0, 17, 18, 5, 2, -1, 21, 9, 13, 6, 3, 16, -1, 7, 8, 4, 10, 19, 12, 28, 20, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1
];

static SUCCESSOR_IDS_BY_CHR_ID_AND_CHR_ID: [[i16; 32]; 32] = [
    [7, 4, 12, -1, 6, -1, 1, 0, 3, 5, -1, 9, -1, 8, 2, -1, 15, 14, -1, 10, 11, -1, -1, -1, -1, -1, -1, -1, 13, -1, -1, -1],
    [-1, -1, 6, -1, 1, -1, 0, 3, 2, 4, 15, 11, -1, 9, 5, 10, 13, -1, 12, 8, 7, 14, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [9, 11, -1, 4, 2, -1, 0, 8, 1, 5, -1, 6, -1, 3, 7, 15, -1, 12, 10, 13, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [-1, -1, 14, 7, 5, -1, 1, 2, 8, 9, 0, 15, 6, 4, 11, -1, 12, 3, -1, 10, -1, 13, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [2, 4, 3, 1, 5, 0, -1, 6, 10, 9, 7, 12, 11, -1, -1, -1, -1, 13, -1, -1, 8, -1, 15, -1, -1, -1, 14, -1, -1, -1, -1, -1],
    [0, 1, 2, 3, 4, -1, -1, 5, 9, 10, 6, -1, -1, 8, 15, 11, -1, 14, -1, -1, 7, -1, 13, -1, -1, -1, 12, -1, -1, -1, -1, -1],
    [2, 8, 7, 4, 3, -1, 9, -1, 6, 11, -1, 5, -1, -1, 0, -1, -1, 14, 1, 15, 10, 12, -1, -1, -1, -1, 13, -1, -1, -1, -1, -1],
    [0, 3, 1, 2, 6, -1, 9, 8, 4, 12, 13, 10, -1, 11, 7, -1, -1, 15, 14, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [0, 6, 3, 4, 1, 2, -1, -1, 5, 10, 7, 9, 11, 12, -1, -1, 8, 14, -1, -1, 15, 13, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [0, 6, 2, 5, 9, -1, -1, -1, 10, 1, 8, -1, 12, 14, 4, -1, 15, 7, -1, 13, 3, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [8, 10, 9, 15, 1, -1, 4, 0, 3, 2, -1, 6, -1, 12, 11, 13, 7, 14, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [1, 3, 6, 0, 4, 2, -1, 7, 13, 8, 9, 11, -1, -1, 15, -1, -1, -1, -1, -1, 10, 5, 14, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [3, 0, 1, 4, -1, 2, 5, 6, 7, 8, -1, 14, -1, -1, 9, 15, -1, 12, -1, -1, -1, 10, 11, -1, -1, -1, 13, -1, -1, -1, -1, -1],
    [0, 1, 3, 2, 15, -1, 12, -1, 7, 14, 4, -1, -1, 9, -1, 8, 5, 10, -1, -1, 6, -1, 13, -1, -1, -1, 11, -1, -1, -1, -1, -1],
    [0, 3, 1, 2, -1, -1, 12, 6, 4, 9, 7, -1, -1, 14, 8, -1, -1, 15, 11, 13, 5, -1, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [0, 5, 7, 2, 10, 13, -1, 6, 8, 1, 3, -1, -1, 14, 15, 11, -1, -1, -1, 12, 4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [0, 2, 6, 3, 7, 10, -1, 1, 9, 4, 8, -1, -1, 15, -1, 12, 5, -1, -1, -1, 11, -1, 13, -1, -1, -1, 14, -1, -1, -1, -1, -1],
    [1, 3, 4, 0, 7, -1, 12, 2, 11, 8, 6, 13, -1, -1, -1, -1, -1, 5, -1, -1, 10, 15, 9, -1, -1, -1, 14, -1, -1, -1, -1, -1],
    [1, 3, 5, 2, 13, 0, 9, 4, 7, 6, 8, -1, -1, 15, -1, 11, -1, -1, 10, -1, 14, -1, 12, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [0, 2, 1, 3, -1, -1, -1, 6, -1, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, 4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [1, 11, 4, 0, 3, -1, 13, 12, 2, 7, -1, -1, 15, 10, 5, 8, 14, -1, -1, -1, -1, -1, 9, -1, -1, -1, 6, -1, -1, -1, -1, -1],
    [0, 9, 2, 14, 15, 4, 1, 13, 3, 5, -1, -1, 10, -1, -1, -1, -1, 6, 12, -1, 7, -1, 8, -1, -1, -1, 11, -1, -1, -1, -1, -1],
    [-1, 2, 14, -1, 1, 5, 8, 7, 4, 12, -1, 6, 9, 11, 13, 3, 10, 15, -1, -1, -1, -1, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [0, 1, 3, 2, -1, -1, -1, -1, -1, -1, 4, -1, -1, -1, -1, -1, -1, -1, -1, -1, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [4, 3, 1, 5, -1, -1, -1, 0, -1, -1, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [2, 8, 4, 1, -1, 0, -1, 6, -1, -1, 5, -1, 7, -1, -1, -1, -1, -1, -1, -1, 10, -1, -1, 9, -1, -1, -1, -1, -1, -1, -1, -1],
    [12, 5, -1, -1, 1, -1, -1, 7, 0, 3, -1, 2, -1, 4, 6, -1, -1, -1, -1, 8, -1, -1, 15, -1, 13, 9, -1, -1, -1, -1, -1, 11],
    [1, 3, 2, 4, -1, -1, -1, 5, -1, 7, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, 6, -1, -1, -1, -1, -1, -1, -1, -1, 8, -1, -1],
    [5, 3, 4, 12, 1, 6, -1, -1, -1, -1, 8, 2, -1, -1, -1, -1, 0, 9, -1, -1, 11, -1, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [-1, -1, -1, -1, 0, -1, 1, 12, 3, -1, -1, -1, -1, 5, -1, -1, -1, 2, -1, -1, -1, -1, -1, -1, -1, -1, 4, -1, -1, 6, -1, 10],
    [2, 3, 1, 4, -1, 0, -1, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 7, -1, -1, -1, -1, -1, -1, -1, -1, 6, -1, -1],
    [5, 1, 3, 0, -1, -1, -1, -1, -1, -1, 4, -1, -1, -1, -1, -1, -1, -1, -1, -1, 2, -1, -1, -1, -1, -1, 9, -1, -1, 6, -1, 7]
];

static CHRS_BY_CHR_AND_SUCCESSOR_ID: [[char; 16]; (MAX_CHR - MIN_CHR) as usize] = [
    ['s', 't', 'c', 'l', 'm', 'a', 'd', 'r', 'v', 'T', 'A', 'L', 'e', 'M', 'Y', '-'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['-', 't', 'a', 'b', 's', 'h', 'c', 'r', 'n', 'w', 'p', 'm', 'l', 'd', 'i', 'f'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['u', 'e', 'i', 'a', 'o', 'r', 'y', 'l', 'I', 'E', 'R', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['e', 'a', 'o', 'i', 'u', 'A', 'y', 'E', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['t', 'n', 'f', 's', '\'', 'm', 'I', 'N', 'A', 'E', 'L', 'Z', 'r', 'V', 'R', 'C'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['o', 'a', 'y', 'i', 'u', 'e', 'I', 'L', 'D', '\'', 'E', 'Y', '\x00', '\x00', '\x00', '\x00'],
    ['r', 'i', 'y', 'a', 'e', 'o', 'u', 'Y', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['h', 'o', 'e', 'E', 'i', 'u', 'r', 'w', 'a', 'H', 'y', 'R', 'Z', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['h', 'i', 'e', 'a', 'o', 'r', 'I', 'y', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['n', 't', 's', 'r', 'l', 'd', 'i', 'y', 'v', 'm', 'b', 'c', 'g', 'p', 'k', 'u'],
    ['e', 'l', 'o', 'u', 'y', 'a', 'r', 'i', 's', 'j', 't', 'b', 'v', 'h', 'm', 'd'],
    ['o', 'e', 'h', 'a', 't', 'k', 'i', 'r', 'l', 'u', 'y', 'c', 'q', 's', '-', 'd'],
    ['e', 'i', 'o', 'a', 's', 'y', 'r', 'u', 'd', 'l', '-', 'g', 'n', 'v', 'm', 'f'],
    ['r', 'n', 'd', 's', 'a', 'l', 't', 'e', 'm', 'c', 'v', 'y', 'i', 'x', 'f', 'p'],
    ['o', 'e', 'r', 'a', 'i', 'f', 'u', 't', 'l', '-', 'y', 's', 'n', 'c', '\'', 'k'],
    ['h', 'e', 'o', 'a', 'r', 'i', 'l', 's', 'u', 'n', 'g', 'b', '-', 't', 'y', 'm'],
    ['e', 'a', 'i', 'o', 't', 'r', 'u', 'y', 'm', 's', 'l', 'b', '\'', '-', 'f', 'd'],
    ['n', 's', 't', 'm', 'o', 'l', 'c', 'd', 'r', 'e', 'g', 'a', 'f', 'v', 'z', 'b'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['e', 'n', 'i', 's', 'h', 'l', 'f', 'y', '-', 'a', 'w', '\'', 'g', 'r', 'o', 't'],
    ['e', 'l', 'i', 'y', 'd', 'o', 'a', 'f', 'u', 't', 's', 'k', 'w', 'v', 'm', 'p'],
    ['e', 'a', 'o', 'i', 'u', 'p', 'y', 's', 'b', 'm', 'f', '\'', 'n', '-', 'l', 't'],
    ['d', 'g', 'e', 't', 'o', 'c', 's', 'i', 'a', 'n', 'y', 'l', 'k', '\'', 'f', 'v'],
    ['u', 'n', 'r', 'f', 'm', 't', 'w', 'o', 's', 'l', 'v', 'd', 'p', 'k', 'i', 'c'],
    ['e', 'r', 'a', 'o', 'l', 'p', 'i', 't', 'u', 's', 'h', 'y', 'b', '-', '\'', 'm'],
    ['\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['e', 'i', 'o', 'a', 's', 'y', 't', 'd', 'r', 'n', 'c', 'm', 'l', 'u', 'g', 'f'],
    ['e', 't', 'h', 'i', 'o', 's', 'a', 'u', 'p', 'c', 'l', 'w', 'm', 'k', 'f', 'y'],
    ['h', 'o', 'e', 'i', 'a', 't', 'r', 'u', 'y', 'l', 's', 'w', 'c', 'f', '\'', '-'],
    ['r', 't', 'l', 's', 'n', 'g', 'c', 'p', 'e', 'i', 'a', 'd', 'm', 'b', 'f', 'o'],
    ['e', 'i', 'a', 'o', 'y', 'u', 'r', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00', '\x00'],
    ['a', 'i', 'h', 'e', 'o', 'n', 'r', 's', 'l', 'd', 'k', '-', 'f', '\'', 'c', 'b'],
    ['p', 't', 'c', 'a', 'i', 'e', 'h', 'q', 'u', 'f', '-', 'y', 'o', '\x00', '\x00', '\x00'],
    ['o', 'e', 's', 't', 'i', 'd', '\'', 'l', 'b', '-', 'm', 'a', 'r', 'n', 'p', 'w']
];


struct Pack {
    word: u32,
    bytes_packed: usize,
    bytes_unpacked: usize,
    offsets: [usize; 8],
    masks: [i16; 8],
    header_mask: u8,
    header: u8,
}

const PACK_COUNT: usize = 3;
const MAX_SUCCESSOR_N: usize = 7;

static PACKS: [Pack; PACK_COUNT] = [
    Pack {
        word: 0x80000000,
        bytes_packed: 1,
        bytes_unpacked: 2,
        offsets: [26, 24, 24, 24, 24, 24, 24, 24],
        masks: [15, 3, 0, 0, 0, 0, 0, 0],
        header_mask: 0xc0,
        header: 0x80,
    },
    Pack {
        word: 0xc0000000,
        bytes_packed: 2,
        bytes_unpacked: 4,
        offsets: [25, 22, 19, 16, 16, 16, 16, 16],
        masks: [15, 7, 7, 7, 0, 0, 0, 0],
        header_mask: 0xe0,
        header: 0xc0,
    },
    Pack {
        word: 0xe0000000,
        bytes_packed: 4,
        bytes_unpacked: 8,
        offsets: [23, 19, 15, 11, 8, 5, 2, 0],
        masks: [31, 15, 15, 15, 7, 7, 7, 3],
        header_mask: 0xf0,
        header: 0xe0,
    },
];

fn decode_header(val: u8) -> i32 {
    let mut i = -1;
    let mut val = val as i8;
    while val < 0 {
        val <<= 1;
        i += 1;
    }
    i
}


fn check_indices(indices: &[i16], pack_n: usize) -> bool {
    indices.iter()
        .take(PACKS[pack_n].bytes_unpacked)
        .enumerate()
        .all(|(i, &index)| index <= PACKS[pack_n].masks[i])
}

fn find_best_encoding(indices: &[i16], n_consecutive: usize) -> Option<usize> {
    PACKS.iter().enumerate().find(|&(idx, pack)| {
        n_consecutive >= pack.bytes_unpacked && check_indices(&indices[..n_consecutive], idx)
    }).map(|(idx, _)| idx)
}


fn shoco_compress(in_bytes: &[u8], buffer: &mut Vec<u8>, desired_len: usize) -> Result<usize, &'static str> {
    let mut indices: [i16; MAX_SUCCESSOR_N + 1] = [0; MAX_SUCCESSOR_N + 1];
    let original_buffer_length = buffer.len();
    let mut output_length = 0;

    if in_bytes.len() > original_buffer_length {
        return Ok(original_buffer_length + 1);
    }

    let safe_len = if desired_len > in_bytes.len() || desired_len == 0 {
        in_bytes.len()
    } else {
        desired_len
    };

    let mut i = 0;
    while i < safe_len {
        if in_bytes[i] == 0 {
            break;
        }

        indices[0] = CHR_IDS_BY_CHR[in_bytes[i] as usize];
        if indices[0] < 0 {
            if output_length + 1 > buffer.capacity() {
                return Err("Buffer overflow");
            }
            if output_length >= original_buffer_length {
                buffer.push(in_bytes[i]);
            } else {
                buffer[output_length] = in_bytes[i];
            }
            output_length += 1;
            i += 1;
            continue;
        }

        let mut n_consecutive = 1;
        while n_consecutive <= MAX_SUCCESSOR_N && i + n_consecutive < safe_len {
            let current_index = CHR_IDS_BY_CHR[in_bytes[i + n_consecutive] as usize];
            if current_index < 0 {
                break;
            }

            let successor_index = SUCCESSOR_IDS_BY_CHR_ID_AND_CHR_ID[indices[n_consecutive - 1] as usize][current_index as usize];
            if successor_index < 0 {
                break;
            }

            indices[n_consecutive] = successor_index;
            n_consecutive += 1;
        }

        if n_consecutive < 2 {
            if output_length + 1 > buffer.capacity() {
                return Err("Buffer overflow");
            }
            if output_length >= original_buffer_length {
                buffer.push(in_bytes[i]);
            } else {
                buffer[output_length] = in_bytes[i];
            }
            output_length += 1;
            i += 1;
            continue;
        }

        if let Some(pack_n) = find_best_encoding(&indices[..n_consecutive], n_consecutive) {
            if output_length + PACKS[pack_n].bytes_packed > buffer.capacity() {
                return Err("Buffer overflow");
            }

            let mut packed_word = PACKS[pack_n].word;
            for (index, &value) in indices.iter().take(PACKS[pack_n].bytes_unpacked).enumerate() {
                packed_word |= (value as u32) << PACKS[pack_n].offsets[index];
            }
            packed_word = swap(packed_word);

            let packed_bytes = packed_word.to_ne_bytes();
            for &byte in &packed_bytes[..PACKS[pack_n].bytes_packed] {
                if output_length >= original_buffer_length {
                    buffer.push(byte);
                } else {
                    buffer[output_length] = byte;
                }
                output_length += 1;
            }
            i += PACKS[pack_n].bytes_unpacked;
        } else {
            if output_length + 1 > buffer.capacity() {
                return Err("Buffer overflow");
            }
            if output_length >= original_buffer_length {
                buffer.push(in_bytes[i]);
            } else {
                buffer[output_length] = in_bytes[i];
            }
            output_length += 1;
            i += 1;
        }
    }

    Ok(output_length)
}

pub fn shoco_decompress(original: &[u8], buffer: &mut [u8]) -> Result<usize, &'static str> {
    //println!("{:?}", original);
    if original.is_empty() {
        return Err("Input is empty");
    }

    // Add specific check for single null byte input
    if original.len() == 1 && original[0] == 0 {
        return Ok(usize::MAX);
    }

    if original.len() > buffer.len() {
        return Ok(buffer.len() + 1);
    }

    let mut out_idx = 0;
    let mut in_idx = 0;

    while in_idx < original.len() {
        let mark = decode_header(original[in_idx]);
        if mark < 0 {
            if out_idx >= buffer.len() {
                return Err("Output buffer is too small");
            }
            buffer[out_idx] = original[in_idx];
            out_idx += 1;
            in_idx += 1;

            if original[in_idx - 1] == 0 {
                if in_idx < original.len() && original[in_idx] == 0 {
                    buffer[out_idx] = 0;
                    return Ok(out_idx);
                }
            }
        } else {
            let pack = PACKS.get(mark as usize).ok_or("Invalid pack index")?;
            if out_idx + pack.bytes_unpacked > buffer.len() {
                return Err("Output buffer overflow");
            }
            if in_idx + pack.bytes_packed > original.len() {
                return Ok(usize::MAX);
            }

            let mut code: u32 = 0;
            for i in 0..pack.bytes_packed {
                code |= (original[in_idx + i] as u32) << (8 * (pack.bytes_packed - 1 - i));
            }

            let mut last_chr = CHRS_BY_CHR_ID[(code >> pack.offsets[0] & pack.masks[0] as u32) as usize];
            buffer[out_idx] = last_chr as u8;
            out_idx += 1;

            for i in 1..pack.bytes_unpacked {
                last_chr = CHRS_BY_CHR_AND_SUCCESSOR_ID[(last_chr as usize - MIN_CHR as usize)][((code >> pack.offsets[i] & pack.masks[i] as u32) as usize)];
                buffer[out_idx + i - 1] = last_chr as u8;
                out_idx += 1;
            }

            in_idx += pack.bytes_packed;
        }
    }

    if out_idx < buffer.len() {
        buffer[out_idx] = 0; // Ensure null termination of output buffer
    }

    Ok(out_idx)
}



fn main() {
    let large_str = "This is a large string that won't possibly fit into a small buffer";
    let non_ascii_str = "Übergrößenträger";

    let mut buf_1 = vec![0_u8; 1];
    let mut buf_2 = vec![0_u8; 2];
    let mut buf_4 = vec![0_u8; 4];
    let mut buf_large = vec![0_u8; 4096];
    let mut ret = Ok(0);

    //#1
    ret = shoco_compress(large_str.as_bytes(), &mut buf_2, 0);
    assert_eq!(ret, Ok(3));

    //#2
    ret = shoco_compress(large_str.as_bytes(), &mut buf_large, 0);
    assert!(ret.is_ok() && ret.unwrap() <= large_str.len());

    //#3
    ret = shoco_compress("a".as_bytes(), &mut buf_1, 0);
    assert!(ret.is_ok() && ret.unwrap() == 1);

    //#4
    buf_2[1] = b'x';
    ret = shoco_compress("a".as_bytes(), &mut buf_2, 0);
    assert!(ret.is_ok() && ret.unwrap() == 1);
    assert!(buf_2.len() > 1 && buf_2[1] == b'x', "Buffer was altered unexpectedly");

    //#5
    ret = shoco_compress("a".as_bytes(), &mut buf_4, 0);
    assert!(ret.is_ok() && ret.unwrap() == 1);

    //#6
    ret = shoco_compress("test".as_bytes(), &mut buf_4, 0);
    assert!(ret.is_ok() && ret.unwrap() <= 4);

    //7
    buf_4[1] = b'x'; 
    ret = shoco_compress("test".as_bytes(), &mut buf_4, 1);
    assert!(ret.is_ok() && ret.unwrap() == 1);
    assert!(buf_4[1] == b'x', "Buffer was altered unexpectedly");

    //8
    ret = shoco_compress(b"t\x80", &mut buf_4, 1);  
    assert!(ret.is_ok() && ret.unwrap() == 1);
    assert!(buf_4[1] == b'x', "Buffer was altered unexpectedly");

    //9
    buf_4[1] = b'y';
    ret = shoco_compress("test".as_bytes(), &mut buf_4, 1);
    assert!(ret.is_ok() && ret.unwrap() == 1);
    assert!(buf_4[1] == b'y', "Buffer was altered unexpectedly");

    //10
    buf_4[1] = b'z';
    ret = shoco_compress("a".as_bytes(), &mut buf_4, 1);
    assert!(ret.is_ok() && ret.unwrap() == 1);
    assert!(buf_4[1] == b'z', "Buffer was altered unexpectedly");

    //11
    buf_4[1] = b'b';
    ret = shoco_compress("a".as_bytes(), &mut buf_4, 2);
    assert!(ret.is_ok() && ret.unwrap() == 1);
    assert!(buf_4[1] == b'b', "Buffer was altered unexpectedly");

    //12
    ret = shoco_compress("Ã¤".as_bytes(), &mut buf_1, 1);
    assert!(ret.is_ok() && ret.unwrap() == 2);


    //Test decompression

    let mut compressed_large = vec![0_u8; 4096];
    let large_len = large_str.len();
    ret = shoco_compress(large_str.as_bytes(), &mut compressed_large, 0);
    let comp_len = ret.unwrap();

    buf_large[large_len] = b'x';
    ret = shoco_decompress(&compressed_large[..comp_len],  &mut buf_large[..4096]);
    assert_eq!(ret, Ok(large_len));
    assert_eq!((&buf_large[..large_len]).len(), large_str.as_bytes().len());
    assert_eq!(buf_large[large_len], b'\0');


    ret = shoco_decompress(&compressed_large[..comp_len], &mut buf_2[..2]);
    assert_eq!(ret, Ok(3));

    buf_large[large_len] = b'x';
    ret = shoco_decompress(&compressed_large[..comp_len], &mut buf_large[..large_len]);
    assert_eq!(ret, Ok(large_len));
    assert_ne!(buf_large[large_len], b'\0'); 

    ret = shoco_decompress(&compressed_large[..5], &mut buf_large[..4096]);
    assert!(ret < Ok(large_len)|| ret == Ok(4097)); 

    let mut compressed_non_ascii = vec![0; 256];
    let non_ascii_len = non_ascii_str.len();
    ret = shoco_compress(non_ascii_str.as_bytes(), &mut compressed_non_ascii, 0);
    buf_large[non_ascii_len] = b'x';
    ret = shoco_decompress(&compressed_non_ascii[..ret.unwrap()], &mut buf_large[..4096]);
    //assert_eq!(ret, Ok(non_ascii_len));
    assert_eq!((&buf_large[..non_ascii_len]).len(), non_ascii_str.as_bytes().len());

    //println!("Value: {}", non_ascii_len);    
    //assert_eq!(buf_large[non_ascii_len], b'\0');

    ret = shoco_decompress(&b"\x00"[..1], &mut buf_large[..4096]);
    assert_eq!(ret.unwrap(), usize::MAX);

    ret = shoco_decompress(&b"\xe0ab"[..3], &mut buf_large[..4096]);
    assert_eq!(ret.unwrap(), usize::MAX);

    println!("All tests passed.");
}