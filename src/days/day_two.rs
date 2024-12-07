use crate::{read_file, Problem};

pub struct DayTwo {}

fn is_report_safe_with_problem_dampening(levels: Vec<i32>) -> bool {
    let initial_safety = is_report_safe(levels.clone());
    if initial_safety {
        return true
    } else {
        for i in 0..levels.len() {
            let mut temp_level = levels.clone();
            temp_level.remove(i);
            if is_report_safe(temp_level) {
                return true;
            }
        }
    }
    return false
}

fn is_report_safe(levels: Vec<i32>) -> bool {
    let mut slope: i32 = 0;
    for i in 0..(levels.len()-1) {
        let diff: i32 = levels[i+1] - levels[i];
        let abs_diff = levels[i+1].abs_diff(levels[i]);
        if abs_diff == 0 || abs_diff > 3 {
            return false
        }
        let cur_slope = diff / (abs_diff as i32);
        if slope + cur_slope == 0 {
            return false
        } else { slope = cur_slope }
    }
    return true;
}

impl Problem for DayTwo {

    fn part_one(&self) -> String {
        let content = read_file("src/inputs/day2.txt");
        let list_of_reports = content.split("\r\n").collect::<Vec<&str>>();
        let mut safe_report_count = 0;
        for report in list_of_reports {
            let levels: Vec<i32> = report.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
            if is_report_safe(levels) {
                safe_report_count += 1;
            }
        }
        format!("{}", safe_report_count)
    }

    fn part_two(&self) -> String {
        let content = read_file("src/inputs/day2.txt");
        let list_of_reports = content.split("\r\n").collect::<Vec<&str>>();
        let mut safe_report_count = 0;
        for report in list_of_reports {
            let levels: Vec<i32> = report.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
            if is_report_safe_with_problem_dampening(levels) {
                safe_report_count += 1;
            }
        }
        format!("{}", safe_report_count)
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
}