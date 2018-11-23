#![feature(asm)]
extern crate colored;
use colored::*;
pub mod asm;
pub mod calibration;

fn main() {
    println!("{}", "Flush Reload Test!".blue().bold());
    unsafe {
        let threshhold = calibration::get_threshhold();
        println!(
            "{} {}",
            "The threshhold is:".yellow().bold(),
            format!("{} cycles", threshhold).yellow().bold()
        );
    }
}
