extern crate libc;
use self::libc::sched_yield;
use asm;

fn only_reload_time(addr: *const usize) -> u64 {
    let time_1 = asm::get_rdtsc();
    asm::mem_access(addr);
    asm::get_rdtsc() - time_1 
}

fn flush_reload_time(addr: *const usize) -> u64 {
    let time_1 = asm::get_rdtsc();
    asm::mem_access(addr);
    let time_2 = asm::get_rdtsc();
    asm::flush_cache_line(addr);
    time_2 - time_1
}

pub fn get_threshhold() -> u32 {
    let mut threshhold: u32 = 0;
    let mut hits: [usize; 600] = [0; 600];
    let mut misses: [usize; 600] = [0; 600];
    let mut arr: [usize; 5 * 1024] = [0; 5 * 1024];
    /*TODO: Implement*/
    threshhold
}