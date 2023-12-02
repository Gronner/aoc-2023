#[macro_use]
mod utils;
mod day1;
mod day2;

pub fn get_days() -> Vec<fn()> {
    vec![
        day1::run_day,
        day2::run_day,
    ]
}
