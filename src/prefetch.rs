/*
    Provides functionality for calculating prefetch hashes
    Author: Harel Segev
    06/10/2022
 */

use std::num::ParseIntError;

pub struct PfHashFunction {
    name: &'static str,
    function: fn(&str) -> u32
}

impl PfHashFunction {
    pub fn scca_vista() -> Self {
        Self {
            name: "SCCA Vista",
            function: hash::scca_vista
        }
    }

    pub fn scca_xp() -> Self {
        Self {
            name: "SCCA XP",
            function: hash::scca_xp
        }
    }

    pub fn hash(&self, filename: &str) -> u32 {
        (self.function)(filename)
    }

    pub fn to_string(&self) -> String {
        self.name.to_owned()
    }
}


mod hash {
    use std::iter::{Flatten, Map};
    use std::str::EncodeUtf16;

    fn encode_char_utf16_le(char: u16) -> [u8; 2] {
        char.to_le_bytes()
    }

    fn encode_utf16_bytes(filename: &str) -> Flatten<Map<EncodeUtf16<'_>, fn(u16) -> [u8; 2]>> {
        filename
            .encode_utf16()
            .map(encode_char_utf16_le as fn(u16) -> [u8; 2])
            .flatten()
    }

    pub fn scca_vista(filename: &str) -> u32 {
        let mut hash_value: u32 = 314159;
        for byte in encode_utf16_bytes(filename) {
            hash_value = hash_value
                .wrapping_mul(37)
                .wrapping_add(byte as u32);
        }

        hash_value
    }

    pub fn scca_xp(filename: &str) -> u32 {
        let mut hash_value: i32 = 0;
        for byte in encode_utf16_bytes(filename) {
            hash_value = hash_value
                .wrapping_mul(37)
                .wrapping_add(byte as i32);
        }

        hash_value = hash_value.wrapping_mul(314159269);

        if hash_value < 0 {
            hash_value = hash_value.wrapping_neg();
        }

        (hash_value as u32) % 1000000007
    }
}

pub fn from_base16(hash: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(hash, 16)
}

pub struct DevicePaths {
    id: i32
}

impl DevicePaths {
    pub fn new() -> Self {
        Self {
            id: 0
        }
    }
}

impl Iterator for DevicePaths {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.id {
            0..=9 => {
                let res = format!("\\DEVICE\\HARDDISKVOLUME{}", self.id);
                self.id += 1;

                Some(res)
            }

            _ => None
        }
    }
}