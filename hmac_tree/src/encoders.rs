pub mod BinaryRep {
    use std::fmt::Binary;

    pub fn bytes_to_binary(s: &Vec<u8>) -> String {
        let mut res = String::new();

        for c in s {
            res = format!("{}{:08b}", res, c);
        }

        res
    }

    pub fn binary_to_int(s: &String) -> u64 {
        let int = {
                match u64::from_str_radix(s, 2) {
                    Ok(i) => i,
                    Err(e) => panic!("{:?}", e),
                }
        };

        int
    }

    pub fn integer_to_binary<T: Binary>(int: T, padding: u8) -> String {
        match padding {
            8 => format!("{:08b}", int),
            16 => format!("{:016b}", int),
            32 => format!("{:032b}", int),
            64 => format!("{:064b}", int),
            128 => format!("{:0128b}", int),
            _ => format!("{:b}", int)
        }

    }
}

pub mod HexRep {
    pub fn decimal_to_hex(dec: u64) -> String {
        format!("{:016x}", dec)
    }

    pub fn decode_hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect::<Vec<u8>>()
    }
}

