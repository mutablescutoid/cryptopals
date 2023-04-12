pub mod bytes_ext;
pub mod str_ext;

//Base64 constants

//Utilizing six bit values as indicies on this table will yield the corresponding Base64 character
pub const BASE64_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

pub const BASE64_TO_BINARY_TABLE: [u8; 256] = [
    include!("BASE64_TO_BINARY_TABLE")
];

pub const BYTE_DIVISOR: i32 = 3;
pub const BASE64_DIVISOR: usize = 4;
pub const BASE64_PADDING: char = '=';

//Hex constants
pub const HEX_TABLE: [char; 22] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C',
    'D', 'E', 'F',
];

pub const HEX_TO_BYTE_TABLE: [u8; u8::MAX as usize * u8::MAX as usize] = [
    include!("HEX_TO_BYTE_TABLE")
];

//Crypto constants

//Each percentage is stored as itself times the max i64 size, in order to avoid the oddities of floating point numbers
pub const ENGLISH_FREQUENCY_TABLE: [(char, i64); 26] = [
    ('a', 82389258 * (i64::MAX / 1000000000)),
    ('b', 15051398 * (i64::MAX / 1000000000)),
    ('c', 28065007 * (i64::MAX / 1000000000)),
    ('d', 42904556 * (i64::MAX / 1000000000)),
    ('e', 12813865 * (i64::MAX / 1000000000)),
    ('f', 22476217 * (i64::MAX / 1000000000)),
    ('g', 20327458 * (i64::MAX / 1000000000)),
    ('h', 61476691 * (i64::MAX / 1000000000)),
    ('i', 61476691 * (i64::MAX / 1000000000)),
    ('j', 01543474 * (i64::MAX / 1000000000)),
    ('k', 07787989 * (i64::MAX / 1000000000)),
    ('l', 40604477 * (i64::MAX / 1000000000)),
    ('m', 24271893 * (i64::MAX / 1000000000)),
    ('n', 68084376 * (i64::MAX / 1000000000)),
    ('o', 75731132 * (i64::MAX / 1000000000)),
    ('p', 19459884 * (i64::MAX / 1000000000)),
    ('q', 00958366 * (i64::MAX / 1000000000)),
    ('r', 60397268 * (i64::MAX / 1000000000)),
    ('s', 63827211 * (i64::MAX / 1000000000)),
    ('t', 91357551 * (i64::MAX / 1000000000)),
    ('u', 27822893 * (i64::MAX / 1000000000)),
    ('v', 09866131 * (i64::MAX / 1000000000)),
    ('w', 23807842 * (i64::MAX / 1000000000)),
    ('x', 01513210 * (i64::MAX / 1000000000)),
    ('y', 19913847 * (i64::MAX / 1000000000)),
    ('z', 00746517 * (i64::MAX / 1000000000)),
];
