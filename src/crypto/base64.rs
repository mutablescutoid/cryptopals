pub trait Base64 {
    const BINARY_TO_BASE64_TABLE: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];

/* const BASE64_TO_BINARY_TABLE: [(char, u8); 64] = [
        ('A', , 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'p', 'o', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];*/

    const BYTE_DIVISOR: i32 = 3;
    const BASE64_DIVISOR: usize = 4;
    const BASE64_PADDING: char = '=';

    fn to_base64(&self) -> String;

//  fn from_base64(base64_str: String) -> Vec<u8>;

//  fn is_base64() -> bool;
}

impl Base64 for Vec<u8> {
    fn to_base64(&self) -> String {
        let mut base64_str = String::from("");

        let mut bytes_index = 0;
        let mut scratch_byte = 0x00;
        for byte in self {
            match bytes_index % Self::BYTE_DIVISOR {
                0 => {
                    base64_str.push(Self::BINARY_TO_BASE64_TABLE[(byte >> 2) as usize]); //Utilizes first six bits
                    scratch_byte = byte << 6 >> 2; //Places the remaining two bits at the beginning of the next six bits
                }
                1 => {
                    base64_str.push(Self::BINARY_TO_BASE64_TABLE[((byte >> 4) + scratch_byte) as usize]); //Utilizes first byte's remaining two bits and the next four bits
                    scratch_byte = byte << 4 >> 2; //Places the remaining four bits at the beginning of next the next six bits
                }
                2 => {
                    base64_str.push(Self::BINARY_TO_BASE64_TABLE[((byte >> 6) + scratch_byte) as usize]); //Utilizes second byte's remaining four bits and the next two bits
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

    /*fn from_base64(base64_str: String) -> Vec<u8> {
        let bytes = Vec::new();
        for base64_byte in base64_str.into_bytes() {
            bytes.push(base64_byte)
        }
    }*/
}
