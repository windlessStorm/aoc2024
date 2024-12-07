use std::io::Lines;
use ndarray::{arr2, Array2};


use crate::{read_file, Problem};

pub struct DayFour {}

impl Problem for DayFour {
    fn part_one(&self) -> String {
        let content = read_file("src/inputs/day2.txt");
        let nrows = content.lines().count();
        let ncolumns =  content.lines().next().unwrap().len();
        let mut arr = Array2::<char>::default((nrows, ncolumns));

        println!("{:?}", arr);

        format!("{}", "Part one not yet implemented.")
    }
    fn part_two(&self) -> String {
        format!("{}", "Part one not yet implemented.")
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
}