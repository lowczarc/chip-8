pub mod audio;
pub mod display;
pub mod opcodes;
pub mod state;

use audio::Audio;
use display::Display;
use state::{CPU, RAM};
use std::time::SystemTime;
use std::env;

pub fn exec(cpu: &mut CPU, ram: &mut RAM, display: &mut Display, instr: [u8; 2]) {
    let opcode = instr[0] >> 4;

    match opcode {
        0x0 => opcodes::sys(cpu, ram, display, instr),
        0x1 => opcodes::jp1(cpu, instr),
        0x2 => opcodes::call(cpu, ram, instr),
        0x3 => opcodes::se3(cpu, instr),
        0x4 => opcodes::sne4(cpu, instr),
        0x5 => opcodes::se5(cpu, instr),
        0x6 => opcodes::ld6(cpu, instr),
        0x7 => opcodes::add7(cpu, instr),
        0x8 => opcodes::op8(cpu, instr),
        0x9 => opcodes::sne9(cpu, instr),
        0xa => opcodes::lda(cpu, instr),
        0xb => opcodes::jpb(cpu, instr),
        0xc => opcodes::rnd(cpu, instr),
        0xd => opcodes::drw(cpu, ram, display, instr),
        0xe => opcodes::ope(cpu, display, instr),
        0xf => opcodes::opf(cpu, ram, display, instr),
        _ => (),
    }
}

fn main() {
    let mut cpu = CPU::new();
    let mut ram = RAM::new();

    if env::args().len() != 2 {
        println!("Usage: chip-8-emulator <rom.ch8>");
        return
    }

    let rom = env::args().nth(1);

    println!("Starting {:?}...", rom.clone().unwrap());

    ram.load_program_from_file(&rom.unwrap()).unwrap();

    let mut last_dt = SystemTime::now();
    let mut display = Display::new();
    let mut audio = Audio::new();

    while cpu.pc < 0xfff {
        let instr_h = ram.0[cpu.pc as usize];
        let instr_l = ram.0[cpu.pc as usize + 1];

        cpu.pc += 2;

        if SystemTime::now()
            .duration_since(last_dt)
            .unwrap()
            .as_micros()
            > 16666
        {
            if cpu.dt != 0 {
                cpu.dt -= 1;
            }

            if cpu.st != 0 {
                cpu.st -= 1;
                audio.beep_start();
            } else {
                audio.beep_stop();
            }
            display.update();
            last_dt = SystemTime::now();
        }

        exec(&mut cpu, &mut ram, &mut display, [instr_h, instr_l]);
    }

    println!("{:?}", cpu);
}
