use std::{collections::{HashMap, HashSet}, io::{self, Write}, thread, time};
use colored::*;

use crate::{pause, read_file, Problem};

pub struct DaySix {}

#[derive(Debug, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn direction_to_char(dir: Direction) -> char {
    match dir {
        Direction::UP => '^',
        Direction::DOWN => 'v',
        Direction::LEFT => '<',
        Direction::RIGHT => '>'
    }
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<char>>,
    nrows: usize,
    ncolumns: usize,
}

#[derive(Debug, Clone)]
struct GameState {
    map: Map,
    cur_pos: (usize, usize),
    cur_facing: Direction,
    path_taken: Vec<(usize, usize)>,
    obstacles: Vec<(usize, usize)>,
    is_over: bool,
}

impl GameState {
    fn from(content: Vec<Vec<char>>) -> Self {
        let nrows = content.len();
        let ncolumns = content[0].len();
        let mut map = content;
        let directions: HashMap<char, Direction> = HashMap::from([
            ('^', Direction::UP),
            ('<', Direction::LEFT),
            ('v', Direction::DOWN),
            ('>', Direction::RIGHT),
        ]);
        let mut cur_pos: (usize, usize) = (0,0);
        let mut cur_facing: Direction = Direction::RIGHT;
        let mut obstacles: Vec<(usize, usize)> = vec![];
        for row in 0..nrows {
            for column in 0..ncolumns {
                let character = map[row][column];
                if directions.keys().collect::<Vec<&char>>().contains(&&character) {
                    cur_pos = (row, column);
                    cur_facing = directions[&map[row][column]];
                    map[row][column] = '.';
                }
                if map[row][column] == '#' {
                    obstacles.push((row, column));
                }
            }
        }
        return GameState {
            map: Map {
                grid: map.clone(),
                nrows,
                ncolumns
            },
            cur_pos,
            cur_facing,
            obstacles,
            path_taken: vec![cur_pos],
            is_over: false,
    
        }
    }

    fn turn_right(&mut self) {
        self.cur_facing = match self.cur_facing {
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN
        }
    }

    fn render(&mut self) {
        let mut stdout: io::StdoutLock<'_> = io::stdout().lock();
        let mut print_buffer = String::new();
        for row in 0..self.map.nrows {
            for column in 0..self.map.ncolumns {
                if (row, column) == self.cur_pos {
                    if self.is_over {
                        print_buffer.push_str(&format!("{}", direction_to_char(self.cur_facing).to_string().yellow()));
                    } else{
                        print_buffer.push_str(&format!("{}", direction_to_char(self.cur_facing).to_string().green()));
                    }
                } else if self.path_taken.contains(&(row, column)){
                   print_buffer.push_str(&format!("{}", self.map.grid[row][column].to_string().green()));
                } else if self.obstacles.contains(&(row, column)){
                    print_buffer.push_str(&format!("{}", self.map.grid[row][column].to_string().red()));
                } else {
                    print_buffer.push_str(&format!("{}", self.map.grid[row][column].to_string().blue()));
                }
            }
            print_buffer.push_str("\r\n");
        }
        clearscreen::clear().unwrap();
        stdout.write_all(print_buffer.as_bytes()).expect("Can't write to stdout!");
    }

    fn run(&mut self) {
        self.run_step();
        let millis = time::Duration::from_millis(1);
        thread::sleep(millis);
        self.render();
    }

    fn run_step(&mut self) {

        let (dx, dy) = match self.cur_facing {
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
            Direction::RIGHT => (0, 1),
        };

        let new_x = self.cur_pos.0 as isize + dx;
        let new_y = self.cur_pos.1 as isize + dy;

        if new_x > self.map.nrows.try_into().unwrap() || new_x < 0 || new_y > self.map.ncolumns.try_into().unwrap() || new_y < 0 {
            self.is_over = true;
            return
        } else if self.obstacles.contains(&(new_x as usize, new_y as usize)) {
            self.turn_right();
        } else {
            self.cur_pos = (new_x as usize, new_y as usize);
        }
    }
}

impl Problem for DaySix {
    fn part_one(&self) -> String {
        let content = read_file("src/inputs/day6.txt");
        let content: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

        let mut game = GameState::from(content);
        
        while !game.is_over {
            game.run();
        }

        let set: HashSet<&(usize, usize)> = HashSet::from_iter(game.path_taken.iter().clone());

        format!("{:?}",set.len())
    }
    fn part_two(&self) -> String {
        format!("{}", "Part one not yet implemented.")
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
}