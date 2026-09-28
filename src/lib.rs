use exarmo_aarch64;
use byteorder::{ByteOrder, LittleEndian};
use memchr::memmem;
use std::str::Utf8Error;
use std::{fs, process};

pub struct IBoot {
    iboot: Vec<u8>,
    pub base_addr: u64,
}

impl IBoot {
    pub fn init(file: &String) -> Result<Self, std::io::Error> {
        let iboot = fs::read(file)?;

        Ok(Self {
            iboot: iboot,
            base_addr: 0x0,
        })
    }

    pub fn is_iboot(&mut self) -> bool {
        match memmem::find(&self.iboot, b"iBoot-") {
            Some(..) => return true,
            None => return false,
        };
    }

    fn get_iboot_version(&mut self) -> Result<&str, Utf8Error> {
        match std::str::from_utf8(&self.iboot[0x286..0x300]) {
            Ok(iboot_version) => return Ok(iboot_version),
            Err(e) => return Err(e),
        };
    }

    pub fn get_base_addr(&mut self) -> u64 {
        let mut base_addr_offset: usize = 0x318;
        let base_addr: u64;
        let iboot_vers: String;

        match self.get_iboot_version() {
            Ok(iboot_version_str) => iboot_vers = iboot_version_str.to_string(),
            Err(e) => {
                println!("Error getting iboot version : {e}");
                process::exit(1);
            }
        };

        println!("iBoot version : {iboot_vers}");
        let v: Vec<&str> = iboot_vers.split(".").collect();
        if v[0].parse::<u64>().unwrap() >= 6603 {
            base_addr_offset = 0x300
        }

        base_addr = LittleEndian::read_u64(&self.iboot[base_addr_offset..base_addr_offset + 8]);

        return base_addr;
    }

    pub fn disassemble(&mut self, count: u32, skip: usize) {
        let mut i = skip as u32;
        for (n, word) in self.iboot[skip * 4..].chunks_exact(4).enumerate() {
            if count != 0 && count == i {
                break;
            }
            let address = self.base_addr + (n as u64) * 4;
            match exarmo_aarch64::decode(LittleEndian::read_u32(word)) {
                Ok(decoded) => {
                    println!("{:04x}: {}", address, decoded.at(address));
                }
                Err(_) => println!("{:04x}: (bad)", address),
            };

            i += 1;
        }
    }
}
