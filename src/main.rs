#![feature(asm)]
extern crate colored;
extern crate scheduler;
use colored::*;
pub mod asm;
pub mod calibration;
pub mod probe;
use probe::gpg_probe;

fn main() {
    /*Make current thread run on the 0th CPU*/
    let cpu: scheduler::CpuSet = scheduler::CpuSet::single(0);
    scheduler::set_affinity(std::process::id() as i32, cpu).unwrap();
    println!("{}", "Flush Reload Test!".blue().bold());
    /*Get threshhold that signifies that a memory access missed*/
    let threshhold = unsafe { calibration::get_threshhold() };
    println!(
        "{} {}",
        "The threshhold is:".yellow().bold(),
        format!("{} cycles", threshhold).yellow().bold()
    );
    gpg_probe::probe(threshhold);
}
