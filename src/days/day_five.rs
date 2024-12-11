use std::collections::{HashMap, HashSet};

use crate::{read_file, Problem};

pub struct DayFive {}

fn is_sequence_in_correct_order(seq: Vec<usize>, mut page_follow_order_map: HashMap<usize, HashSet<usize>>, mut page_preceding_order_map: HashMap<usize, HashSet<usize>>) -> Option<usize> {
    for i in 0..(seq.len() - 1) {
        let page_one = seq[i];
        let page_two = seq[i+1];

        if page_follow_order_map.entry(page_one).or_default().contains(&page_two) {
            if !page_preceding_order_map.entry(page_one).or_default().contains(&page_two) {} 
            else {
                return None
            }
        } else {
            return None
        }
    }
    return Some(seq[(seq.len()-1)/2])
}

fn fix_incorrectly_ordered_updates(mut seq: Vec<usize>, mut page_follow_order_map: HashMap<usize, HashSet<usize>>, mut page_preceding_order_map: HashMap<usize, HashSet<usize>>) -> usize {
    for _ in 0..(seq.len() - 1) {
        for i in 0..(seq.len() - 1) {
            let mut nothing_to_fix = false;
            while !nothing_to_fix {
                let page_one = seq[i];
                let page_two = seq[i+1];

                if page_follow_order_map.entry(page_one).or_default().contains(&page_two) {
                    if !page_preceding_order_map.entry(page_one).or_default().contains(&page_two) {
                        nothing_to_fix = true;                    
                    } else {
                        nothing_to_fix = false;
                        seq[i] = page_two;
                        seq[i+1] = page_one;
                    }
                } else {
                    nothing_to_fix = false;
                    let _old_seq = seq.clone();
                    seq[i] = page_two;
                    seq[i+1] = page_one;
                }
            }
        }
    }
    return seq[(seq.len()-1)/2]
}

impl Problem for DayFive {
    fn part_one(&self) -> String {
        let content = read_file("src/inputs/day5.txt");
        let mut split = content.split("\r\n\r\n");
        let ordering_rules: Vec<Vec<usize>> = split.next().unwrap().split("\r\n").map(|pair| pair.split("|").map(|x| x.parse::<usize>().unwrap()).collect()).collect();
        let print_sequence: Vec<Vec<usize>> = split.next().unwrap().split("\r\n").map(|line| line.split(",").map(|x| x.parse::<usize>().unwrap()).collect()).collect();

        let mut page_follow_order_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut page_preceding_order_map: HashMap<usize, HashSet<usize>> = HashMap::new();

        for rule in ordering_rules {
            let cur_set = page_follow_order_map.entry(rule[0]).or_insert(HashSet::from([]));
            cur_set.insert(rule[1]);
            let cur_set = page_preceding_order_map.entry(rule[1]).or_insert(HashSet::from([]));
            cur_set.insert(rule[0]);
        }

        let mut sum: usize = 0;

        for sequence in print_sequence {
            match is_sequence_in_correct_order(sequence.clone(), page_follow_order_map.clone(), page_preceding_order_map.clone()) {
                Some(val) => { 
                    sum += val;
                }
                None => { }
            }
        }

        format!("{}", sum)
    }
    fn part_two(&self) -> String {
        let content = read_file("src/inputs/day5.txt");
        let mut split = content.split("\r\n\r\n");
        let ordering_rules: Vec<Vec<usize>> = split.next().unwrap().split("\r\n").map(|pair| pair.split("|").map(|x| x.parse::<usize>().unwrap()).collect()).collect();
        let print_sequence: Vec<Vec<usize>> = split.next().unwrap().split("\r\n").map(|line| line.split(",").map(|x| x.parse::<usize>().unwrap()).collect()).collect();

        let mut page_follow_order_map: HashMap<usize, HashSet<usize>> = HashMap::new();
        let mut page_preceding_order_map: HashMap<usize, HashSet<usize>> = HashMap::new();

        for rule in ordering_rules {
            let cur_set = page_follow_order_map.entry(rule[0]).or_insert(HashSet::from([]));
            cur_set.insert(rule[1]);
            let cur_set = page_preceding_order_map.entry(rule[1]).or_insert(HashSet::from([]));
            cur_set.insert(rule[0]);
        }

        let mut sum: usize = 0;

        for sequence in print_sequence {
            match is_sequence_in_correct_order(sequence.clone(), page_follow_order_map.clone(), page_preceding_order_map.clone()) {
                Some(_val) => {}
                None => { 
                    sum += fix_incorrectly_ordered_updates(sequence.clone(), page_follow_order_map.clone(), page_preceding_order_map.clone());
                }
            }
        }

        format!("{}", sum)
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
}