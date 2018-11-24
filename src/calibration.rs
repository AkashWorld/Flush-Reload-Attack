extern crate colored;
extern crate libc;
extern crate term_size;
use self::libc::sched_yield;
use asm;
use colored::*;
use std::thread;

fn print_histogram_array(arr: &[u64]) {
    let (w, _) = term_size::dimensions_stdout().unwrap();
    let mut width: usize = 0;
    if (w > 20) {
        width = w - 20;
    }
    for i in 0..arr.len() {
        print!("{}:", format!("{} cycles({})", i, arr[i]).purple());
        for j in 0..arr[i] / 22 {
            if j > width as u64 {
                break;
            }
            print!("{}", "#".blue().bold());
        }
        println!("");
    }
}

pub unsafe fn get_threshhold() -> u32 {
    const INITIAL_THRESHHOLD: usize = 500;
    const TEST_ARR_SIZE: usize = 1024;
    let mut hits: [u64; INITIAL_THRESHHOLD] = [0; INITIAL_THRESHHOLD];
    let mut misses: [u64; INITIAL_THRESHHOLD] = [0; INITIAL_THRESHHOLD];
    let mut arr: [u64; 1024] = [std::u64::MAX; 1024];
    let arr_ptr: *mut u8 = &mut (arr[0] as u8);
    asm::mem_access(arr_ptr.add(TEST_ARR_SIZE/2));
    sched_yield();
    for _ in 0..10 * TEST_ARR_SIZE * TEST_ARR_SIZE {
        let time = asm::full_reload_time(arr_ptr.add(TEST_ARR_SIZE/2));
        hits[std::cmp::min(time as usize, hits.len() - 1)] += 1;
    }
    /*Printing thread*/
    let hits_thread = thread::spawn(move || {
        let hits_copy = hits.clone();
        println!("{}", format!("Hits Histogram").yellow().bold());
        print_histogram_array(&hits_copy[..]);
    });
    asm::flush_cache_line(arr_ptr.add(TEST_ARR_SIZE/2));
    for _ in 0..10 * TEST_ARR_SIZE * TEST_ARR_SIZE {
        let time = asm::full_flush_reload_time(arr_ptr.add(TEST_ARR_SIZE/2));
        misses[std::cmp::min(time as usize, misses.len() - 1)] += 1;
    }
    match hits_thread.join() {
        Err(val) => println!("Error with hit histogram printing thread, {:?}", val),
        Ok(_) => {}
    };
    /*Printing thread*/
    let miss_thread = thread::spawn(move || {
        let miss_copy = misses.clone();
        println!("{}", format!("Misses Histogram").yellow().bold());
        print_histogram_array(&miss_copy[..]);
    });
    let mut hit_max: u64 = 0;
    let mut hit_max_time: usize = 0;
    let mut misses_max: u64 = 0;
    let mut misses_max_time: usize = 0;
    for i in 0..INITIAL_THRESHHOLD {
        if hit_max < hits[i] {
            hit_max = hits[i];
            hit_max_time = i;
        }
        if misses_max == 0 && misses[i] > 3 {
            misses_max = misses[i];
            misses_max_time = i;
        }
    }
    match miss_thread.join() {
        Err(val) => println!("Error with miss histogram printing thread, {:?}", val),
        Ok(_) => {}
    };
    println!(
        "{}",
        format!(
            "Most commonly (within range) occured miss cycles: {}",
            misses_max_time
        ).red()
    );
    println!(
        "{}",
        format!("Most commonly occured hit cycles: {}", hit_max_time).green()
    );
    let mut approx_threshhold_count = std::u32::MAX;
    let mut approx_threadhold: u32 = 0;
    for i in hit_max_time..misses_max_time {
        if approx_threshhold_count as u64 > hits[i] + misses[i] {
            approx_threshhold_count = hits[i] as u32 + misses[i] as u32;
            approx_threadhold = i as u32;
        }
    }
    approx_threadhold
}
