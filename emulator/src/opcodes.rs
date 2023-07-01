use crate::consts::{
    DISPLAY_UPDATE_SLEEP_TIME_MICROS, FONT_START_ADDRESS, RNG_INCREMENT, RNG_MULTIPLIER,
};
use crate::display::Display;
use crate::state::{CPU, RAM};
use std::{thread, time};

pub fn sys(cpu: &mut CPU, ram: &mut RAM, display: &mut Display, instr: [u8; 2]) {
    match (instr[0], instr[1]) {
        (0x00, 0x0e0) => {
            display.cls();
        }
        (0x00, 0xee) => {
            let addr_h: u16 = ram.0[cpu.sp as usize] as u16;
            let addr_l: u16 = ram.0[cpu.sp as usize + 1] as u16;

            let addr = (addr_h << 8) | addr_l;

            cpu.sp -= 2;

            cpu.pc = addr;
        }
        _ => (),
    }
}

pub fn jp1(cpu: &mut CPU, instr: [u8; 2]) {
    cpu.pc = (((instr[0] & 0xf) as u16) << 8) | instr[1] as u16;
}

pub fn call(cpu: &mut CPU, ram: &mut RAM, instr: [u8; 2]) {
    cpu.sp += 2;

    ram.0[cpu.sp as usize] = (cpu.pc >> 8) as u8;
    ram.0[cpu.sp as usize + 1] = (cpu.pc & 0xff) as u8;

    cpu.pc = (((instr[0] & 0xf) as u16) << 8) | instr[1] as u16;
}

pub fn se3(cpu: &mut CPU, instr: [u8; 2]) {
    if cpu.r[(instr[0] & 0xf) as usize] == instr[1] {
        cpu.pc += 2;
    }
}

pub fn sne4(cpu: &mut CPU, instr: [u8; 2]) {
    if cpu.r[(instr[0] & 0xf) as usize] != instr[1] {
        cpu.pc += 2;
    }
}

pub fn se5(cpu: &mut CPU, instr: [u8; 2]) {
    if cpu.r[(instr[0] & 0xf) as usize] == cpu.r[(instr[1] >> 4) as usize] && instr[1] & 0xf == 0 {
        cpu.pc += 2;
    }
}

pub fn ld6(cpu: &mut CPU, instr: [u8; 2]) {
    cpu.r[(instr[0] & 0xf) as usize] = instr[1];
}

pub fn add7(cpu: &mut CPU, instr: [u8; 2]) {
    cpu.r[(instr[0] & 0xf) as usize] += instr[1];
}

pub fn ld8(cpu: &mut CPU, x: usize, y: usize) {
    cpu.r[x] = cpu.r[y];
}

pub fn or8(cpu: &mut CPU, x: usize, y: usize) {
    cpu.r[x] |= cpu.r[y];
}

pub fn and8(cpu: &mut CPU, x: usize, y: usize) {
    cpu.r[x] &= cpu.r[y];
}

pub fn xor8(cpu: &mut CPU, x: usize, y: usize) {
    cpu.r[x] ^= cpu.r[y];
}

pub fn add8(cpu: &mut CPU, x: usize, y: usize) {
    let res = cpu.r[x] as u16 + cpu.r[y] as u16;

    if res > 0xff {
        cpu.r[0xf] = 1;
    }

    cpu.r[x] = (res & 0xff) as u8;
}

pub fn sub8(cpu: &mut CPU, x: usize, y: usize) {
    let res = cpu.r[x] - cpu.r[y];

    if cpu.r[x] > cpu.r[y] {
        cpu.r[0xf] = 1;
    }

    cpu.r[x] = res;
}

pub fn shr8(cpu: &mut CPU, x: usize) {
    cpu.r[0xf] = 0x1 & cpu.r[x];

    cpu.r[x] >>= 1;
}

pub fn subn8(cpu: &mut CPU, x: usize, y: usize) {
    let res = cpu.r[y] - cpu.r[x];

    if cpu.r[y] > cpu.r[x] {
        cpu.r[0xf] = 1;
    }

    cpu.r[x] = res;
}

pub fn shl8(cpu: &mut CPU, x: usize) {
    cpu.r[0xf] = cpu.r[x] >> 7;

    cpu.r[x] <<= 1;
}

pub fn op8(cpu: &mut CPU, instr: [u8; 2]) {
    let x = (instr[0] & 0xf) as usize;
    let y = (instr[1] >> 4) as usize;

    match instr[1] & 0xf {
        0x0 => ld8(cpu, x, y),
        0x1 => or8(cpu, x, y),
        0x2 => and8(cpu, x, y),
        0x3 => xor8(cpu, x, y),
        0x4 => add8(cpu, x, y),
        0x5 => sub8(cpu, x, y),
        0x6 => shr8(cpu, x),
        0x7 => subn8(cpu, x, y),
        0xe => shl8(cpu, x),
        _ => (),
    }
}

pub fn sne9(cpu: &mut CPU, instr: [u8; 2]) {
    if cpu.r[(instr[0] & 0xf) as usize] != cpu.r[(instr[1] >> 4) as usize] && instr[1] & 0xf == 0 {
        cpu.pc += 2;
    }
}

pub fn lda(cpu: &mut CPU, instr: [u8; 2]) {
    cpu.i = (((instr[0] & 0xf) as u16) << 8) | instr[1] as u16;
}

pub fn jpb(cpu: &mut CPU, instr: [u8; 2]) {
    cpu.pc = (((instr[0] & 0xf) as u16) << 8) | instr[1] as u16 + cpu.r[0] as u16;
}

pub fn rnd(cpu: &mut CPU, instr: [u8; 2]) {
    cpu.rng = (RNG_MULTIPLIER * cpu.rng + RNG_INCREMENT) & 0xffffffff;

    cpu.r[(instr[0] & 0xf) as usize] = cpu.rng as u8 & instr[1];
}

pub fn drw(cpu: &mut CPU, ram: &mut RAM, display: &mut Display, instr: [u8; 2]) {
    let x = cpu.r[(instr[0] & 0xf) as usize];
    let y = cpu.r[(instr[1] >> 4) as usize];
    let n = (instr[1] & 0xf) as usize;

    let mut col = false;

    for i in 0..n {
        col |= display.draw(x, y + i as u8, ram.0[cpu.i as usize + i]);
    }

    cpu.r[0xf] = if col { 1 } else { 0 };
}

pub fn skp(cpu: &mut CPU, display: &mut Display, x: usize) {
    if display.check_key_press(cpu.r[x]) {
        cpu.pc += 2;
    }
}

pub fn sknp(cpu: &mut CPU, display: &mut Display, x: usize) {
    if !display.check_key_press(cpu.r[x]) {
        cpu.pc += 2;
    }
}

pub fn ope(cpu: &mut CPU, display: &mut Display, instr: [u8; 2]) {
    let x = (instr[0] & 0xf) as usize;

    match instr[1] {
        0x9e => skp(cpu, display, x),
        0xa1 => sknp(cpu, display, x),
        _ => (),
    }
}

pub fn ldf07(cpu: &mut CPU, x: usize) {
    cpu.r[x] = cpu.dt;
}

pub fn ldf0a(cpu: &mut CPU, display: &mut Display, x: usize) {
    loop {
        for i in 0..0x10 {
            if display.check_key_press(i) {
                cpu.r[x] = i;
                return;
            }
        }
        thread::sleep(time::Duration::from_micros(
            DISPLAY_UPDATE_SLEEP_TIME_MICROS,
        ));
        display.update();
    }
}

pub fn ldf15(cpu: &mut CPU, x: usize) {
    cpu.dt = cpu.r[x];
}

pub fn ldf18(cpu: &mut CPU, x: usize) {
    cpu.st = cpu.r[x];
}

pub fn addf1e(cpu: &mut CPU, x: usize) {
    cpu.i += cpu.r[x] as u16;
}

pub fn ldf29(cpu: &mut CPU, x: usize) {
    cpu.i = FONT_START_ADDRESS + (5 * (cpu.r[x] & 0xf)) as u16;
}

pub fn ldf33(cpu: &mut CPU, ram: &mut RAM, x: usize) {
    ram.0[cpu.i as usize] = cpu.r[x] / 100;
    ram.0[cpu.i as usize + 1] = cpu.r[x] % 100 / 10;
    ram.0[cpu.i as usize + 2] = cpu.r[x] % 10;
}

pub fn ldf55(cpu: &mut CPU, ram: &mut RAM, x: usize) {
    for i in 0..(x + 1) {
        ram.0[cpu.i as usize + i] = cpu.r[i]
    }
}

pub fn ldf65(cpu: &mut CPU, ram: &mut RAM, x: usize) {
    for i in 0..(x + 1) {
        cpu.r[i] = ram.0[cpu.i as usize + i]
    }
}

pub fn opf(cpu: &mut CPU, ram: &mut RAM, display: &mut Display, instr: [u8; 2]) {
    let x = (instr[0] & 0xf) as usize;

    match instr[1] {
        0x07 => ldf07(cpu, x),
        0x0a => ldf0a(cpu, display, x),
        0x15 => ldf15(cpu, x),
        0x18 => ldf18(cpu, x),
        0x1e => addf1e(cpu, x),
        0x29 => ldf29(cpu, x),
        0x33 => ldf33(cpu, ram, x),
        0x55 => ldf55(cpu, ram, x),
        0x65 => ldf65(cpu, ram, x),
        _ => (),
    }
}
