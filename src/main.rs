mod core;
use core::CachedFunction;

#[allow(dead_code)]
fn minimum(nums: (i32, i32)) -> i32 {
    if nums.0 < nums.1 {
        nums.0
    } else {
        nums.1
    }
}

#[allow(dead_code)]
fn add_strings(strs: (&str, &str)) -> String {
    strs.0.to_string() + strs.1
}

#[allow(dead_code)]
fn reps(args: (&str, &i32)) -> String {
    args.0.repeat(*args.1 as usize)
}

#[allow(dead_code)]
fn maxx(nums: Vec<u32>) -> u32 {
    *nums.iter().max().unwrap_or(&0)
}

fn main() {
    let mut cashed_minimum = CachedFunction::new(maxx);
    println!("{}", cashed_minimum.call(vec![1, 2, 3, 7]));
    println!("{}", cashed_minimum.call(vec![1, 0, 3, 0]));
    println!("{}", cashed_minimum.call(vec![8, 0, 0, 98]));
    println!("{}", cashed_minimum.call(vec![1, 0, 3, 0]));
}
