#![feature(asm)]
extern crate colored;
use colored::*;
pub mod asm;
pub mod calibration;
pub mod probe;
use probe::gpg_probe;

fn main() {
    println!("{}", "Flush Reload Test!".blue().bold());
    let threshhold = unsafe { calibration::get_threshhold() };
    println!(
        "{} {}",
        "The threshhold is:".yellow().bold(),
        format!("{} cycles", threshhold).yellow().bold()
    );
    gpg_probe::probe(threshhold);
}
