use std::fmt::format;
use std::io::Lines;
use ndarray::{arr2, Array, Array2, OwnedRepr};
use ndarray::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;


use crate::{read_file, Problem};

pub struct DayFour {}

#[derive(Debug, EnumIter, Clone, Copy)]
enum Direction {
    Top,
    TopLeft,
    Left,
    LeftBottom,
    Bottom,
    BottomRight,
    Right,
    TopRight
}

#[derive(Debug, PartialEq)]
struct XmasPos {
    x_pos: Point2d,
    m_pos: Point2d,
    a_pos: Point2d,
    s_pos: Point2d,
}

#[derive(Debug, PartialEq)]
struct MasPos {
    m_pos: Point2d,
    a_pos: Point2d,
    s_pos: Point2d,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point2d {
    x: usize,
    y: usize,
}

fn check_dir(row: usize, column: usize, matrix: ArrayBase<OwnedRepr<&Vec<char>>, Dim<[usize; 1]>>, nrows: usize, ncolumns: usize, direction: Direction) -> Option<XmasPos> {
    let cur_point = Point2d {
        x: row,
        y: column
    };

    match direction {

        Direction::Right => {
            if (cur_point.y+3) >= ncolumns {
                return None
            }
            let x_loc = cur_point;
            let m_loc = Point2d { x: x_loc.x, y: x_loc.y + 1};
            let a_loc = Point2d { x: x_loc.x, y: x_loc.y + 2};
            let s_loc = Point2d { x: x_loc.x, y: x_loc.y + 3};
            let word: String = vec![matrix[x_loc.x][x_loc.y], matrix[m_loc.x][m_loc.y], matrix[a_loc.x][a_loc.y], matrix[s_loc.x][s_loc.y]].into_iter().collect();
            if word == "XMAS" {
                return Some(XmasPos {
                    x_pos: x_loc,
                    m_pos: m_loc,
                    a_pos: a_loc,
                    s_pos: s_loc,
                })
            } else {
                // display_matrix(vec![x_loc, m_loc, a_loc, s_loc], matrix.clone(), nrows, ncolumns, ('*','*'));
                return None;
            }
        },

        Direction::Bottom => {
            if (cur_point.x+3) >= nrows {
                return None
            }
            let x_loc = cur_point;
            let m_loc = Point2d { x: x_loc.x+1, y: x_loc.y};
            let a_loc = Point2d { x: x_loc.x+2, y: x_loc.y};
            let s_loc = Point2d { x: x_loc.x+3, y: x_loc.y};

            let word: String = vec![matrix[x_loc.x][x_loc.y], matrix[m_loc.x][m_loc.y], matrix[a_loc.x][a_loc.y], matrix[s_loc.x][s_loc.y]].into_iter().collect();
            if word == "XMAS" {
                return Some(XmasPos {
                    x_pos: x_loc,
                    m_pos: m_loc,
                    a_pos: a_loc,
                    s_pos: s_loc,
                })
            } else {
                // display_matrix(vec![x_loc, m_loc, a_loc, s_loc], matrix.clone(), nrows, ncolumns, ('*','*'));
                return None;
            }
        },

        Direction::Left => {
            if cur_point.y < 3 {
                return None
            }
            let x_loc = cur_point;
            let m_loc = Point2d { x: x_loc.x, y: x_loc.y-1};
            let a_loc = Point2d { x: x_loc.x, y: x_loc.y-2};
            let s_loc = Point2d { x: x_loc.x, y: x_loc.y-3};

            let word: String = vec![matrix[x_loc.x][x_loc.y], matrix[m_loc.x][m_loc.y], matrix[a_loc.x][a_loc.y], matrix[s_loc.x][s_loc.y]].into_iter().collect();
            if word == "XMAS" {
                return Some(XmasPos {
                    x_pos: x_loc,
                    m_pos: m_loc,
                    a_pos: a_loc,
                    s_pos: s_loc,
                })
            } else {
                // display_matrix(vec![x_loc, m_loc, a_loc, s_loc], matrix.clone(), nrows, ncolumns, ('*','*'));
                return None;
            }
        },

        Direction::Top => {
            if cur_point.x < 3 {
                return None
            }
            let x_loc = cur_point;
            let m_loc = Point2d { x: x_loc.x-1, y: x_loc.y};
            let a_loc = Point2d { x: x_loc.x-2, y: x_loc.y};
            let s_loc = Point2d { x: x_loc.x-3, y: x_loc.y};
            let word: String = vec![matrix[x_loc.x][x_loc.y], matrix[m_loc.x][m_loc.y], matrix[a_loc.x][a_loc.y], matrix[s_loc.x][s_loc.y]].into_iter().collect();
            if word == "XMAS" {
                return Some(XmasPos {
                    x_pos: x_loc,
                    m_pos: m_loc,
                    a_pos: a_loc,
                    s_pos: s_loc,
                })
            } else {
                // display_matrix(vec![x_loc, m_loc, a_loc, s_loc], matrix.clone(), nrows, ncolumns, ('*','*'));
                return None;
            }
        },

        Direction::TopRight => {
            if cur_point.x < 3 || (cur_point.y+3) >= ncolumns {
                return None
            }
            let x_loc = cur_point;
            let m_loc = Point2d { x: x_loc.x-1, y: x_loc.y+1};
            let a_loc = Point2d { x: x_loc.x-2, y: x_loc.y+2};
            let s_loc = Point2d { x: x_loc.x-3, y: x_loc.y+3};
            let word: String = vec![matrix[x_loc.x][x_loc.y], matrix[m_loc.x][m_loc.y], matrix[a_loc.x][a_loc.y], matrix[s_loc.x][s_loc.y]].into_iter().collect();
            if word == "XMAS" {
                return Some(XmasPos {
                    x_pos: x_loc,
                    m_pos: m_loc,
                    a_pos: a_loc,
                    s_pos: s_loc,
                })
            } else {
                // display_matrix(vec![x_loc, m_loc, a_loc, s_loc], matrix.clone(), nrows, ncolumns, ('*','*'));
                return None;
            }       
        },

        Direction::BottomRight => {
            if cur_point.x + 3 >= nrows  || (cur_point.y + 3) >= ncolumns {
                return None
            }
            let x_loc = cur_point;
            let m_loc = Point2d { x: x_loc.x+1, y: x_loc.y+1};
            let a_loc = Point2d { x: x_loc.x+2, y: x_loc.y+2};
            let s_loc = Point2d { x: x_loc.x+3, y: x_loc.y+3};
            let word: String = vec![matrix[x_loc.x][x_loc.y], matrix[m_loc.x][m_loc.y], matrix[a_loc.x][a_loc.y], matrix[s_loc.x][s_loc.y]].into_iter().collect();
            if word == "XMAS" {
                return Some(XmasPos {
                    x_pos: x_loc,
                    m_pos: m_loc,
                    a_pos: a_loc,
                    s_pos: s_loc,
                })
            } else {
                // display_matrix(vec![x_loc, m_loc, a_loc, s_loc], matrix.clone(), nrows, ncolumns, ('*','*'));
                return None;
            }             
        },

        Direction::LeftBottom => {
            if cur_point.x +3 >= nrows  || cur_point.y < 3  {
                return None
            }
            let x_loc = cur_point;
            let m_loc = Point2d { x: x_loc.x+1, y: x_loc.y-1};
            let a_loc = Point2d { x: x_loc.x+2, y: x_loc.y-2};
            let s_loc = Point2d { x: x_loc.x+3, y: x_loc.y-3};
            let word: String = vec![matrix[x_loc.x][x_loc.y], matrix[m_loc.x][m_loc.y], matrix[a_loc.x][a_loc.y], matrix[s_loc.x][s_loc.y]].into_iter().collect();
            if word == "XMAS" {
                return Some(XmasPos {
                    x_pos: x_loc,
                    m_pos: m_loc,
                    a_pos: a_loc,
                    s_pos: s_loc,
                })
            } else {
                // display_matrix(vec![x_loc, m_loc, a_loc, s_loc], matrix.clone(), nrows, ncolumns, ('*','*'));
                return None;
            }          
        },

        Direction::TopLeft => {
            if cur_point.x < 3  || cur_point.y < 3  {
                return None
            }
            let x_loc = cur_point;
            let m_loc = Point2d { x: x_loc.x-1, y: x_loc.y-1};
            let a_loc = Point2d { x: x_loc.x-2, y: x_loc.y-2};
            let s_loc = Point2d { x: x_loc.x-3, y: x_loc.y-3};
            let word: String = vec![matrix[x_loc.x][x_loc.y], matrix[m_loc.x][m_loc.y], matrix[a_loc.x][a_loc.y], matrix[s_loc.x][s_loc.y]].into_iter().collect();
            if word == "XMAS" {
                return Some(XmasPos {
                    x_pos: x_loc,
                    m_pos: m_loc,
                    a_pos: a_loc,
                    s_pos: s_loc,
                })
            } else {
                // display_matrix(vec![x_loc, m_loc, a_loc, s_loc], matrix.clone(), nrows, ncolumns, ('*','*'));
                return None;
            }          
        },
    }
}

fn check_dir_part2(row: usize, column: usize, matrix: ArrayBase<OwnedRepr<&Vec<char>>, Dim<[usize; 1]>>, nrows: usize, ncolumns: usize, direction: Direction) -> Option<MasPos> {
    let cur_point = Point2d {
        x: row,
        y: column
    };

    match direction {
        Direction::TopRight => {
            if cur_point.x < 2 || (cur_point.y+2) >= ncolumns {
                return None
            }
            let m_loc = cur_point;
            let a_loc = Point2d { x: m_loc.x-1, y: m_loc.y+1};
            let s_loc = Point2d { x: m_loc.x-2, y: m_loc.y+2};
            let word: String = vec![matrix[m_loc.x][m_loc.y], matrix[a_loc.x][a_loc.y], matrix[s_loc.x][s_loc.y]].into_iter().collect();
            if word == "MAS" {
                return Some(MasPos {
                    m_pos: m_loc,
                    a_pos: a_loc,
                    s_pos: s_loc,
                })
            } else {
                // display_matrix(vec![x_loc, m_loc, a_loc, s_loc], matrix.clone(), nrows, ncolumns, ('*','*'));
                return None;
            }       
        },

        Direction::BottomRight => {
            if cur_point.x + 2 >= nrows  || (cur_point.y + 2) >= ncolumns {
                return None
            }
            let m_loc = cur_point;
            let a_loc = Point2d { x: m_loc.x+1, y: m_loc.y+1};
            let s_loc = Point2d { x: m_loc.x+2, y: m_loc.y+2};
            let word: String = vec![matrix[m_loc.x][m_loc.y], matrix[a_loc.x][a_loc.y], matrix[s_loc.x][s_loc.y]].into_iter().collect();
            if word == "MAS" {
                return Some(MasPos {
                    m_pos: m_loc,
                    a_pos: a_loc,
                    s_pos: s_loc,
                })
            } else {
                // display_matrix(vec![x_loc, m_loc, a_loc, s_loc], matrix.clone(), nrows, ncolumns, ('*','*'));
                return None;
            }             
        },

        Direction::LeftBottom => {
            if cur_point.x + 2 >= nrows  || cur_point.y < 2  {
                return None
            }
            let m_loc = cur_point;
            let a_loc = Point2d { x: m_loc.x+1, y: m_loc.y-1};
            let s_loc = Point2d { x: m_loc.x+2, y: m_loc.y-2};
            let word: String = vec![matrix[m_loc.x][m_loc.y], matrix[a_loc.x][a_loc.y], matrix[s_loc.x][s_loc.y]].into_iter().collect();
            if word == "MAS" {
                return Some(MasPos {
                    m_pos: m_loc,
                    a_pos: a_loc,
                    s_pos: s_loc,
                })
            } else {
                // display_matrix(vec![x_loc, m_loc, a_loc, s_loc], matrix.clone(), nrows, ncolumns, ('*','*'));
                return None;
            }          
        },

        Direction::TopLeft => {
            if cur_point.x < 2  || cur_point.y < 2  {
                return None
            }
            let m_loc = cur_point;
            let a_loc = Point2d { x: m_loc.x-1, y: m_loc.y-1};
            let s_loc = Point2d { x: m_loc.x-2, y: m_loc.y-2};
            let word: String = vec![matrix[m_loc.x][m_loc.y], matrix[a_loc.x][a_loc.y], matrix[s_loc.x][s_loc.y]].into_iter().collect();
            if word == "MAS" {
                return Some(MasPos {
                    m_pos: m_loc,
                    a_pos: a_loc,
                    s_pos: s_loc,
                })
            } else {
                // display_matrix(vec![x_loc, m_loc, a_loc, s_loc], matrix.clone(), nrows, ncolumns, ('*','*'));
                return None;
            }          
        },
        _ => None
    }
}

fn display_matrix(highlight_positions: Vec<Point2d>, matrix: ArrayBase<OwnedRepr<&Vec<char>>, Dim<[usize; 1]>>, nrow: usize, ncolumn: usize, highlight_character: (char, char)) {
    println!("Display called for highliting positions: {:?}", highlight_positions);
    print!("   ");
    for i in 0..ncolumn {
        print!(" {} ", i);
    }
    println!();
    for r  in 0..nrow {
        print!(" {r} ");
        for c in 0..ncolumn {
            if highlight_positions.contains(&Point2d{x: r, y: c}) {
                print!("{}{}{}", highlight_character.0, matrix[r][c], highlight_character.1);
            } else {
                print!(" {} ", matrix[r][c]);
            }
        }
        println!();
    }
}

impl Problem for DayFour {
    fn part_one(&self) -> String {
        let content = read_file("src/inputs/day4.txt");
        let nrows = content.lines().count();
        let ncolumns =  content.lines().next().unwrap().len();
        let content: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
        let arr = Array::from_iter(content.iter());
        let mut total_xmas_count: usize = 0;

        for row in 0..nrows {
            for column in 0..ncolumns {
                for dir in Direction::iter() {
                    // println!("Checking Direction: {:?} for item ({}, {})", dir, row, column);
                    // display_matrix(vec![Point2d{ x: row, y: column}], arr.clone(), nrows, ncolumns);
                    match check_dir(row, column, arr.clone(), nrows, ncolumns, dir) {
                        None =>  {}// println!("No Xmas in dir {:?}", dir),
                        Some(val) => { 
                            // println!("found xmas in dir {:?} {:?}", dir, val);
                            // display_matrix(vec![val.x_pos, val.m_pos, val.a_pos, val.s_pos], arr.clone(), nrows, ncolumns, ('[',']'));
                            total_xmas_count += 1;
                        },
                    }
                }
            }
        }
        format!("Counts: {}", total_xmas_count)
    }
    fn part_two(&self) -> String {
        let content = read_file("src/inputs/day4.txt");
        let nrows = content.lines().count();
        let ncolumns =  content.lines().next().unwrap().len();
        let content: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
        let arr = Array::from_iter(content.iter());
        let mut total_x_mas_count: usize = 0;
        let mut all_mas_locations: Vec<MasPos> = vec![];

        for row in 0..nrows {
            for column in 0..ncolumns {
                for dir in Direction::iter() {
                    // println!("Checking Direction: {:?} for item ({}, {})", dir, row, column);
                    // display_matrix(vec![Point2d{ x: row, y: column}], arr.clone(), nrows, ncolumns);
                    match check_dir_part2(row, column, arr.clone(), nrows, ncolumns, dir) {
                        None =>  {}// println!("No Xmas in dir {:?}", dir),
                        Some(val) => { 
                            // println!("found xmas in dir {:?} {:?}", dir, val);
                            // display_matrix(vec![val.x_pos, val.m_pos, val.a_pos, val.s_pos], arr.clone(), nrows, ncolumns, ('[',']'));
                            all_mas_locations.push(val);
                        },
                    }
                }
            }
        }

        for i in 0..all_mas_locations.len() {
            let cur_mas = &all_mas_locations[i];
            for j in i+1..all_mas_locations.len() {
                let next_mas = &all_mas_locations[j];
                if next_mas.a_pos == cur_mas.a_pos {
                    // display_matrix(vec![next_mas.m_pos, next_mas.a_pos, next_mas.s_pos, cur_mas.m_pos, cur_mas.a_pos, cur_mas.s_pos], arr.clone(), nrows, ncolumns, ('[', ']'));
                    total_x_mas_count += 1;
                }
            }
        }

        format!("Counts: {}", total_x_mas_count)
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
}