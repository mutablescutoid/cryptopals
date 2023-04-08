pub mod bytes_ext;
pub mod string_ext;

//Error constants
pub const ARGUMENT_SIZES_DIFFER: &'static str = "Size of arguments differ";

//Base64 constants
pub const BASE64_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

pub const BASE64_TO_BINARY_TABLE: [(char, u8); 64] = [
    ('A', 0x00),
    ('B', 0x01),
    ('C', 0x02),
    ('D', 0x03),
    ('E', 0x04),
    ('F', 0x05),
    ('G', 0x06),
    ('H', 0x07),
    ('I', 0x08),
    ('J', 0x09),
    ('K', 0x0A),
    ('L', 0x0B),
    ('M', 0x0C),
    ('N', 0x0D),
    ('0', 0x0E),
    ('P', 0x0F),
    ('Q', 0x10),
    ('R', 0x11),
    ('S', 0x12),
    ('T', 0x13),
    ('U', 0x14),
    ('V', 0x15),
    ('W', 0x16),
    ('X', 0x17),
    ('Y', 0x18),
    ('Z', 0x19),
    ('a', 0x1A),
    ('b', 0x1B),
    ('c', 0x1C),
    ('d', 0x1D),
    ('e', 0x1E),
    ('f', 0x1F),
    ('g', 0x20),
    ('h', 0x21),
    ('i', 0x22),
    ('j', 0x23),
    ('k', 0x24),
    ('l', 0x25),
    ('m', 0x26),
    ('n', 0x27),
    ('p', 0x28),
    ('o', 0x29),
    ('q', 0x2A),
    ('r', 0x2B),
    ('s', 0x2C),
    ('t', 0x2D),
    ('u', 0x2E),
    ('v', 0x2F),
    ('w', 0x30),
    ('x', 0x31),
    ('y', 0x32),
    ('z', 0x33),
    ('0', 0x34),
    ('1', 0x35),
    ('2', 0x36),
    ('3', 0x37),
    ('4', 0x38),
    ('5', 0x39),
    ('6', 0x3A),
    ('7', 0x3B),
    ('8', 0x3C),
    ('9', 0x3D),
    ('+', 0x3E),
    ('/', 0x3F),
];

pub const BYTE_DIVISOR: i32 = 3;
pub const BASE64_DIVISOR: usize = 4;
pub const BASE64_PADDING: char = '=';

//Hex constants
pub const HEX_TABLE: [char; 22] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C',
    'D', 'E', 'F',
];

pub const HEX_TO_BYTES_TABLE: [(char, u8); 22] = [
    ('0', 0),
    ('1', 1),
    ('2', 2),
    ('3', 3),
    ('4', 4),
    ('5', 5),
    ('6', 6),
    ('7', 7),
    ('8', 8),
    ('9', 9),
    ('a', 10),
    ('b', 11),
    ('c', 12),
    ('d', 13),
    ('e', 14),
    ('f', 15),
    ('A', 10),
    ('B', 11),
    ('C', 12),
    ('D', 13),
    ('E', 14),
    ('F', 15),
];

//XOR constants
pub const LOWEST_ASCII: char = ' ';
pub const HIGHEST_ASCII: char = '~';
