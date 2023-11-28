#[macro_export]
macro_rules! test_2016 {
    ($day:literal, $($params:expr),+) => {$crate::test_year_day!(2016, $day, $($params),+)};
}

mod day1;

macro_rules! run {
    ($day:ident) => {$crate::run_year!(2016, $day)};
}

pub fn all() {
    run!(day1);
}

pub fn run() {
    run!(day1);
}