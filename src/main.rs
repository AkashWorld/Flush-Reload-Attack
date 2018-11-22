#![feature(asm)]
pub mod asm;
pub mod calibration;

fn main() {
    println!("Hello, world!");
    let time_1: u64 = asm::get_rdtsc();
    println!("Time 1: {}", time_1);
    let time_2: u64 = asm::get_rdtsc();
    println!("Time 2: {}", time_2);
}
