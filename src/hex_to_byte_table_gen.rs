/*const HEX_TO_BYTES_TABLE: [(char, u8); 22] = [
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
];*/

//let array: [u8: u8::MAX * u8::MAX] = ;

    (0..=u8::MAX).flat_map(|x| 
        (0..=u8::MAX).map(|y|
        (match std::collections::HashMap::from([
            ('0', 0: u8),
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
        ]).get(&(x as char)) {
                    Some(value) => {
                        *value
                    }
                    None => {
                        0x00
                    }
                }) * 0x10 + 
                match std::collections::HashMap::from([
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
    ]).get(&(y as char)) {
                    Some(value) => {
                        *value
                    }
                    None => {
                        0x00
                    }
                }
        ).collect::<Vec<u8>>()).collect::<Vec<u8>>().try_into().unwrap()