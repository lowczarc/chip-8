use std::fs::File;
use std::io::Read;
use std::time::SystemTime;

#[derive(Debug)]
pub struct CPU {
    pub r: [u8; 0x10], // general purpose registers

    pub i: u16, // address pointer

    pub dt: u8, // delay timer
    pub st: u8, // sound timer

    pub pc: u16, // program counter
    pub sp: u8,  // stack pointer

    pub rng: u32, // rng next seed
}

impl CPU {
    pub fn new() -> Self {
        Self {
            r: [0; 0x10],

            i: 0x000,

            dt: 0,
            st: 0,

            pc: 0x200,
            sp: 0x000,

            rng: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32,
        }
    }
}

#[derive(Debug)]
pub struct RAM(pub [u8; 0x1000]);

impl RAM {
    pub fn new() -> Self {
        Self([0; 0x1000])
    }

    pub fn load_program_from_file(&mut self, filename: &str) -> Result<(), std::io::Error> {
        let mut f = File::open(filename)?;

        f.read(&mut self.0[0x200..])?;

        Ok(())
    }
}
