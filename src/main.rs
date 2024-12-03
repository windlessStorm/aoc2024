use std::{fs};

fn read_file(filename: &str) -> String {
    let value = fs::read_to_string(filename).expect("Should have been able to read!");
    return value
}

fn split_left_right_list(list: Vec<&str>) -> (Vec<u32>, Vec<u32>) {
    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for value in list {
        let items: Vec<u32> = value.split("   ").map(|x| x.parse::<u32>().unwrap()).collect();
        left_list.push(items[0]);
        right_list.push(items[1]);
    }
    left_list.sort();
    right_list.sort();
    return (left_list, right_list)
}

fn solve_part1() {
    let content = read_file("src/inputs/day1part1.txt");
    let list_of_values = content.split("\r\n").collect::<Vec<&str>>();
    let (right_list,  left_list) = split_left_right_list(list_of_values);
    println!("Right List: {:?}", right_list);

    println!("Left List: {:?}", left_list);

    let mut sum: u32 = 0;
    for i in 0..right_list.len() {
        let diff = left_list[i].abs_diff(right_list[i]);
        sum += diff;
    }

    println!("{}", sum)
}

fn solve_part2() {
    let content = read_file("src/inputs/day1part1.txt");
    let list_of_values = content.split("\r\n").collect::<Vec<&str>>();
    let (right_list,  left_list) = split_left_right_list(list_of_values);

    let mut similarity_score: u32 = 0;
    for i in left_list {
        let count: u32 = right_list.iter().filter(|&x| *x==i).count() as u32;
        similarity_score += count*i
    }

    println!("{}", similarity_score)
}

fn main() {
    solve_part1();
    solve_part2();
}
