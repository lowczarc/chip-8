pub const STACK_START_ADDRESS: u8 = 0x002;
pub const FONT_START_ADDRESS: u16 = 0x022;
pub const PROGRAM_START_ADDRESS: u16 = 0x200;

pub const INITIAL_ROM_FILE: &str = "./assets/initial_rom.ch8";

pub const RNG_MULTIPLIER: u32 = 1103515245;
pub const RNG_INCREMENT: u32 = 12345;

pub const DISPLAY_UPDATE_RATE: u64 = 60; // Hertz
pub const DISPLAY_UPDATE_SLEEP_TIME_MICROS: u64 = 1000000 / DISPLAY_UPDATE_RATE;

pub const CPU_SPEED: u64 = 1000; // Hertz
pub const CPU_SLEEP_TIME_MICROS: u64 = 1000000 / CPU_SPEED;
