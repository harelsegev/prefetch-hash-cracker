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
	
	pub fn scca_2008() -> Self {
		Self {
			name: "SCCA 2008",
			function: hash::scca_2008
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
    fn encode_utf16_bytes(filename: &str) -> impl Iterator<Item=u8> + '_ {
        filename
            .encode_utf16()
            .map(|char| char.to_le_bytes())
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
        let mut hash_value: u32 = 0;
        for byte in encode_utf16_bytes(filename) {
            hash_value = hash_value
                .wrapping_mul(37)
                .wrapping_add(byte as u32);
        }

        hash_value = hash_value.wrapping_mul(314159269);

        if (hash_value as i32) < 0 {
            hash_value = hash_value.wrapping_neg();
        }

        hash_value % 1000000007
    }
	
	pub fn scca_2008(filename: &str) -> u32 {
		let mut hash_value: u32 = 314159;
		
		let filename: Vec<_> = encode_utf16_bytes(filename).collect();
		let mut index: usize = 0;
		
		while index + 8 < filename.len() {			
			let mut character_value: u32 = 0;
			
			for n in 1..=6 {
				character_value = character_value.wrapping_add(filename[index + n] as u32);
				character_value = character_value.wrapping_mul(37);
			}
			
			character_value = character_value.wrapping_add(
				(filename[index] as u32).wrapping_mul(442596621)
			);
			
			character_value = character_value.wrapping_add(filename[index + 7] as u32);
			
			hash_value = character_value.wrapping_sub(
				hash_value.wrapping_mul(803794207)
			);
			
			index += 8;
		}
		
		while index < filename.len() {
			hash_value = hash_value
				.wrapping_mul(37)
				.wrapping_add(filename[index] as u32);
			
			index += 1;
		}
		
		hash_value
	}
}

pub fn from_base16(hash: &str) -> Result<u32, ParseIntError> {
    u32::from_str_radix(hash, 16)
}

pub struct DevicePaths<'a> {
    folder: &'a str,
    executable: &'a str,

    id: i32
}

impl<'a> DevicePaths<'a> {
    pub fn new(folder: &'a str, executable: &'a str) -> Self {
        Self {
            folder,
            executable,

            id: 0
        }
    }
}

impl<'a> Iterator for DevicePaths<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.id {
            0..=9 => {
                let (id, folder, executable) = (self.id, self.folder, self.executable);
                let res = format!("\\DEVICE\\HARDDISKVOLUME{id}\\{folder}\\{executable}");

                self.id += 1;
                Some(res)
            }

            _ => None
        }
    }
}