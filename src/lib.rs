#![feature(int_roundings)]
#[macro_use]
mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub fn get_days() -> Vec<fn()> {
    vec![day1::run_day, day2::run_day, day3::run_day, day4::run_day, day5::run_day]
}
