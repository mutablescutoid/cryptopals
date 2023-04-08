pub trait Base64 {
    const BINARY_TO_BASE64_TABLE: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

    const BASE64_TO_BINARY_TABLE: [(char, u8); 64] = [
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

    const BYTE_DIVISOR: i32 = 3;
    const BASE64_DIVISOR: usize = 4;
    const BASE64_PADDING: char = '=';

    fn to_base64(&self) -> String;

    fn from_base64(base64_str: String) -> Vec<u8>;

    fn is_base64(&self) -> bool;
}

impl Base64 for Vec<u8> {
    fn to_base64(&self) -> String {
        let mut base64_str = String::from("");

        self.iter().enumerate()
      
        let mut bytes_index = 0;
        let mut scratch_byte = 0x00;
        for byte in self {
            match bytes_index % Self::BYTE_DIVISOR {
                0 => {
                    base64_str.push(Self::BINARY_TO_BASE64_TABLE[(byte >> 2) as usize]); //Utilizes first six bits
                    scratch_byte = byte << 6 >> 2; //Places the remaining two bits at the beginning of the next six bits
                }
                1 => {
                    base64_str
                        .push(Self::BINARY_TO_BASE64_TABLE[((byte >> 4) + scratch_byte) as usize]); //Utilizes first byte's remaining two bits and the next four bits
                    scratch_byte = byte << 4 >> 2; //Places the remaining four bits at the beginning of next the next six bits
                }
                2 => {
                    base64_str
                        .push(Self::BINARY_TO_BASE64_TABLE[((byte >> 6) + scratch_byte) as usize]); //Utilizes second byte's remaining four bits and the next two bits
                    base64_str.push(Self::BINARY_TO_BASE64_TABLE[(byte << 2 >> 2) as usize]); //Utilizes the last six bits
                    scratch_byte = 0x00;
                }
                _ => {}
            }
            bytes_index += 1;
        }

        if bytes_index % Self::BYTE_DIVISOR != 0 {
            //If the scratch byte contains still relevant info
            base64_str.push(Self::BINARY_TO_BASE64_TABLE[scratch_byte as usize]);
        }

        while base64_str.len() % Self::BASE64_DIVISOR != 0 {
            base64_str.push(Self::BASE64_PADDING);
        }

        base64_str
    }

    fn from_base64(base64_str: String) -> Vec<u8> {
        let bytes = Vec::new();
        for base64_byte in base64_str.into_bytes() {
            
        }
        bytes
    }

    fn is_base64(&self) -> bool {
      self.iter().all(|x| HashSet::from(BINARY_TO_BASE64_TABLE).contains(x as char))
    }
}
