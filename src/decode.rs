extern crate colored;
use colored::*;

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
