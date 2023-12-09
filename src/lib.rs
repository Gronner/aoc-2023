#![feature(int_roundings)]
#![feature(isqrt)]
#![feature(iter_map_windows)]
#[macro_use]
mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn get_days() -> Vec<fn()> {
    vec![
        day1::run_day,
        day2::run_day,
        day3::run_day,
        day4::run_day,
        day5::run_day,
        day6::run_day,
        day7::run_day,
        day8::run_day,
        day9::run_day,
    ]
}
