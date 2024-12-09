use std::fs;
use days::day_five::DayFive;
use days::day_one::DayOne;
use days::day_two::DayTwo;
use days::day_three::DayThree;
use days::day_four::DayFour;
use structopt::StructOpt;

pub mod days;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opts {
    #[structopt(short = "d", long = "day")]
    day: Option<usize>,

    #[structopt(short = "a", long = "all")]
    run_all: bool,
}

pub trait Problem {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}

fn day_to_problem(day: usize) -> Option<Box<dyn Problem>> {
    match day {
        1 => Some(Box::new(DayOne{})),
        2 => Some(Box::new(DayTwo{})),
        3 => Some(Box::new(DayThree{})),
        4 => Some(Box::new(DayFour{})),
        5 => Some(Box::new(DayFive{})),
        // 6 => Some(Box::new(DayThree{})),
        // 7 => Some(Box::new(DayOne{})),
        // 8 => Some(Box::new(DayTwo{})),
        // 9 => Some(Box::new(DayThree{})),
        // ...
        _ => None
    }
}

pub fn read_file(filename: &str) -> String {
    let value = fs::read_to_string(filename).expect("Should have been able to read!");
    return value
}

fn main() {
    let opts = Opts::from_args();
    if opts.run_all {
        println!("Runall not yet implemented")
    } else {
        if opts.day.is_some() {
            let day = opts.day.unwrap();
            println!("Day {} Part 1: {}", day, day_to_problem(day).unwrap().part_one());
            println!("Day {} Part 2: {}", day, day_to_problem(day).unwrap().part_two());
        }
    }
}
