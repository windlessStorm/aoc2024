use core::fmt;
use std::{collections::HashSet, fmt::Debug, io::{self, Write}};
use colored::*;

use crate::{pause, read_file, Problem};

pub struct DaySix {}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Direction {
    fn to_string(&self) -> String {
        let dir = match self {
            Direction::UP => "^",
            Direction::DOWN => "v",
            Direction::LEFT => "<",
            Direction::RIGHT => ">",
        };
        dir.to_string()
    }

    fn from(character: &str) -> Option<Direction> {
        match character {
            "^" => Some(Direction::UP),
            "v" => Some(Direction::DOWN),
            ">" => Some(Direction::RIGHT),
            "<" => Some(Direction::LEFT),
            _ => None
        }
    }
}

#[derive(Clone, PartialEq)]
enum Cell {
    Guard(Pos, Direction),
    Empty(Pos),
    Obstacle(Pos),
}

impl Cell {
    fn to_string(&self) -> String {
        match *self {
            Cell::Empty(_) => ".".to_string(),
            Cell::Guard(_pos, direction) => direction.to_string(),
            Cell::Obstacle(_) => "#".to_string(),
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Cell::Empty(_) => write!(f, "."),
            Cell::Guard(_pos, direction) => write!(f, "{:?}", direction),
            Cell::Obstacle(_) => write!(f, "#")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq, Copy)]
struct Pos {
    x: usize,
    y: usize
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<Cell>>,
    nrows: usize,
    ncolumns: usize,
    cur_pos: Pos,
    cur_facing: Direction,
    path_taken: Vec<(Pos, Direction)>,
    obstacles: Vec<Pos>,
}

#[derive(Debug, Clone)]
struct Game {
    map: Map,
    state: GameState
}

#[derive(Debug, Clone, PartialEq)]
enum GameState {
    INIT,
    RUNNING,
    END,
    LOOP
}

impl Game {
    fn from(content: Vec<Vec<char>>) -> Self {
        let nrows = content.len();
        let ncolumns = content[0].len();
        let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::Empty(Pos{x: 0, y: 0}); ncolumns]; nrows];;
        let mut cur_pos: Pos = Pos {x: 0, y: 0};
        let mut cur_facing: Direction = Direction::UP;
        let mut obstacles: Vec<Pos> = vec![];
        for row in 0..nrows {
            for column in 0..ncolumns {
                let pos = Pos{ x: row, y: column };
                let character = content[row][column];
                match Direction::from(&character.to_string()) {
                    Some(direction) => {
                        cur_pos = pos;
                        cur_facing = direction;
                        grid[row][column] = Cell::Guard(cur_pos, direction);
                    },
                    None => {
                        if character == '#' {
                            obstacles.push(pos);
                            grid[row][column] = Cell::Obstacle(pos);
                        } else if character == '.' {
                            grid[row][column] = Cell::Empty(pos);
                        }
                    }
                }
            }
        }
        return Game {
            map: Map {
                grid: grid.clone(),
                nrows,
                ncolumns,
                cur_pos,
                cur_facing,
                path_taken: vec![(cur_pos, cur_facing)],
                obstacles,
            },
            state: GameState::INIT
        }
    }

    fn turn_right(&mut self) {
        self.map.cur_facing = match self.map.cur_facing {
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN
        };
        self.map.grid[self.map.cur_pos.x][self.map.cur_pos.y] = Cell::Guard(self.map.cur_pos, self.map.cur_facing)
    }

    fn _render(&mut self) {
        let mut stdout: io::StdoutLock<'_> = io::stdout().lock();
        let mut print_buffer = String::new();
        print_buffer.push_str(&format!("   "));
        for i in 0..self.map.ncolumns {
            print_buffer.push_str(&format!(" {} ", i));
        }
        print_buffer.push_str(&format!("\n"));
        for row in 0..self.map.nrows {
            print_buffer.push_str(&format!(" {} ", row));
            for column in 0..self.map.ncolumns {
                let pos = Pos { x: row, y: column };
                if pos ==  self.map.cur_pos {
                    if self.state == GameState::END {
                        print_buffer.push_str(&format!(" {} ", self.map.cur_facing.to_string().yellow()));
                    } else{
                        print_buffer.push_str(&format!(" {} ", self.map.cur_facing.to_string().white()));
                    }
                } else if self.map.path_taken.contains(&(pos, Direction::UP)) 
                || self.map.path_taken.contains(&(pos, Direction::DOWN)) 
                || self.map.path_taken.contains(&(pos, Direction::LEFT))
                || self.map.path_taken.contains(&(pos, Direction::RIGHT)) {
                   print_buffer.push_str(&format!(" {} ", self.map.grid[row][column].to_string().green()));
                } else if self.map.obstacles.contains(&pos){
                    if self.map.obstacles.last() == Some(&pos) {
                        print_buffer.push_str(&format!("[{}]", "#".red()));
                    }
                    else {
                        print_buffer.push_str(&format!(" {} ", self.map.grid[row][column].to_string().red()));
                    }
                } else {
                    print_buffer.push_str(&format!(" {} ", self.map.grid[row][column].to_string().blue()));
                }
            }
            print_buffer.push_str("\r\n");
        }
        // clearscreen::clear().unwrap();
        print_buffer.push_str("\r\n");
        print_buffer.push_str("\r\n");
        stdout.write_all(print_buffer.as_bytes()).expect("Can't write to stdout!");
    }

    fn run(&mut self) {
        self.run_step();
        // let millis = time::Duration::from_millis(200);
        // thread::sleep(millis);
        // if self.path_taken.len()%2 == 0 {
        //     self.render();
        // }
    }

    fn run_step(&mut self) {
        // println!("Running Step -> facing: {:?}, pos: {:?}", self.map.cur_facing, self.map.cur_pos);
        // pause();
        let (dx, dy) = match self.map.cur_facing {
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
            Direction::RIGHT => (0, 1),
        };

        let new_x = self.map.cur_pos.x.wrapping_add_signed(dx);
        let new_y = self.map.cur_pos.y.wrapping_add_signed(dy);
        let new_pos = Pos {x: new_x, y: new_y};
        if new_pos.x >= self.map.nrows || new_pos.y >= self.map.ncolumns {
            self.state = GameState::END;
        } else if self.map.obstacles.contains(&new_pos) {
            self.turn_right();
        } else {
            self.map.cur_pos = new_pos;
            if self.map.path_taken.contains(&(self.map.cur_pos, self.map.cur_facing)) {
                self.state = GameState::LOOP;
            } else {
                self.map.path_taken.push((self.map.cur_pos, self.map.cur_facing));
            }
        }
    }   
}

impl Problem for DaySix {
    fn part_one(&self) -> String {
        let content = read_file("src/inputs/test.txt");
        let content: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

        let mut game = Game::from(content);
        loop {
            match game.state {
                GameState::END => break,
                _ =>  game.run(),
            }
        }

        let set: HashSet<Pos> = HashSet::from_iter(game.map.path_taken.iter().map(|point| point.0));
        format!("{:?}",set.len())
    }
    fn part_two(&self) -> String {
        let content = read_file("src/inputs/day6.txt");
        let content: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

        println!("Running initial path simulation...");
        let mut game = Game::from(content.clone());
        loop {
            match game.state {
                GameState::END => break,
                _ => game.run(),
            }
        }
        
        let positions_to_test: Vec<Pos> = game.map.path_taken[1..]
            .iter()
            .map(|point| point.0)
            .collect::<HashSet<Pos>>() 
            .into_iter()
            .collect();
        
        println!("Testing {} unique positions...", positions_to_test.len());
        let mut valid_loop_positions = HashSet::new();
        let mut tested = 0;
        let total_positions = positions_to_test.len();

        for pos in positions_to_test {
            tested += 1;
            if tested % 100 == 0 {
                println!("Progress: {}/{} positions tested. Found {} loops.", 
                    tested, 
                    total_positions,
                    valid_loop_positions.len()
                );
            }

            let mut new_game = Game::from(content.clone());
            new_game.map.obstacles.push(pos);
            
            loop {
                match new_game.state {
                    GameState::END => break,
                    GameState::LOOP => {
                        valid_loop_positions.insert(pos);
                        break;
                    },
                    _ => new_game.run(),
                }
            }
        }

        println!("Finished! Found {} valid loop positions.", valid_loop_positions.len());
        format!("{}", valid_loop_positions.len())
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
}