use crate::{read_file, Problem};

pub struct DayThree {}


#[derive(PartialEq)]
enum ParserState {
    Empty,
    DoSequence,
    DontSequence,
    StartSequence,
    FirstNumber,
    SecondNumber,
}

trait RunStateMachine {
    fn reset(&mut self);
    fn run_state_machine(&mut self, token: char);
    fn run_state_machine_with_do_sequence(&mut self, token: char);
}

struct StateMachine {
    enabled: bool,
    first_number: u128,
    tens: u128,
    second_number: u128,
    output: u128,
    memory: Vec<char>,
    state: ParserState,
    expect: char,
}

fn initialize_machine() -> StateMachine {
    return StateMachine {
        enabled: true,
        first_number: 0,
        second_number: 0,
        tens: 1,
        output: 0,
        memory: vec![],
        state: ParserState::Empty,
        expect: 'm',
    }
}

impl RunStateMachine for StateMachine {

    fn reset(&mut self) {
        self.first_number = 0;
        self.second_number = 0;
        self.tens = 1;
        self.memory = vec![];
        self.state = ParserState::Empty;
        self.expect = 'm';
    }

    fn run_state_machine( &mut self, token: char) {
        match self.state {
            ParserState::Empty => {
                self.reset();
                if token == 'm' {
                    self.memory.push(token);
                    self.state = ParserState::StartSequence;
                    self.expect = 'u';
                } else {
                    self.memory.clear();
                    self.state = ParserState::Empty;
                    self.expect = 'm';
                }
            },

            ParserState::DoSequence => {}

            ParserState::DontSequence => {}

            ParserState::StartSequence => {
                if token == self.expect {
                    self.memory.push(token);
                     match token {
                        'u' => self.expect = 'l',
                        'l' => self.expect = '(',
                        '(' => {
                            self.tens = 1;
                            self.state = ParserState::FirstNumber;
                            self.first_number = 0;
                        }
                        _ => {
                            self.reset();
                        }
                     }
                } else {
                    self.reset();
                    self.state = ParserState::Empty;
                }
            },
            ParserState::FirstNumber => {
                if token.is_digit(10) {
                    self.tens = 10;
                    let temp: u128 = token.to_digit(10).unwrap() as u128;
                    self.first_number = (self.first_number * self.tens) + temp;
                    self.memory.push(token);
                } else if token == ',' {
                    if self.tens == 1 {
                        self.reset();
                    } else {
                        self.memory.push(token);
                        self.tens = 1;
                        self.state = ParserState::SecondNumber;
                    }
                } else {
                    self.reset();
                }
            },
            ParserState::SecondNumber => {
                if token.is_digit(10) {
                    self.tens = 10;
                    let temp: u128 = token.to_digit(10).unwrap() as u128;
                    self.second_number = (self.second_number * self.tens) + temp;
                    self.memory.push(token);
                } else if token == ')' {
                    if self.tens == 1 {
                        self.reset();
                    } else {
                        self.memory.push(token);
                        if self.enabled {
                            self.output = self.output + (self.first_number as u128 * self.second_number as u128) as u128;
                        } else {
                        }

                        self.reset();
                    }
                } else {
                    self.reset();
                }
            },
        }
    }

    fn run_state_machine_with_do_sequence( &mut self, token: char) {
        match self.state {
            ParserState::Empty => {
                self.reset();
                if token == 'm' {
                    self.memory.push(token);
                    self.state = ParserState::StartSequence;
                    self.expect = 'u';
                } else if token == 'd' {
                    self.memory.push(token);
                    self.state = ParserState::DoSequence;
                    self.expect = 'o';
                } else {
                    self.memory.clear();
                    self.state = ParserState::Empty;
                    self.expect = 'm';
                }
            },

            ParserState::DoSequence => {
                if token == self.expect {
                    self.memory.push(token);
                    match token {
                        'o' => self.expect = '(',
                        '(' => self.expect = ')',
                        ')' => {
                            self.enabled = true;
                            self.reset();
                        },
                        _ => self.reset(),
                    }
                } else if self.expect == '(' && token == 'n' {
                    self.memory.push(token);
                    self.expect = '\'';
                    self.state = ParserState::DontSequence;
                } else {
                    self.reset()
                }
            }

            ParserState::DontSequence => {
                if token == self.expect {
                    self.memory.push(token);
                    match token {
                        '\'' => self.expect = 't',
                        't' => self.expect = '(',
                        '(' => self.expect = ')',
                        ')' => {
                            self.enabled = false;
                            self.reset();
                        },
                        _ => self.reset(),
                    }
                } else {
                    self.reset()
                }
            }

            ParserState::StartSequence => {
                if token == self.expect {
                    self.memory.push(token);
                     match token {
                        'u' => self.expect = 'l',
                        'l' => self.expect = '(',
                        '(' => {
                            self.tens = 1;
                            self.state = ParserState::FirstNumber;
                            self.first_number = 0;
                        }
                        _ => {
                            self.reset();
                        }
                     }
                } else {
                    self.reset();
                    self.state = ParserState::Empty;
                }
            },
            ParserState::FirstNumber => {
                if token.is_digit(10) {
                    self.tens = 10;
                    let temp: u128 = token.to_digit(10).unwrap() as u128;
                    self.first_number = (self.first_number * self.tens) + temp;
                    self.memory.push(token);
                } else if token == ',' {
                    if self.tens == 1 {
                        self.reset();
                    } else {
                        self.memory.push(token);
                        self.tens = 1;
                        self.state = ParserState::SecondNumber;
                    }
                } else {
                    self.reset();
                }
            },
            ParserState::SecondNumber => {
                if token.is_digit(10) {
                    self.tens = 10;
                    let temp: u128 = token.to_digit(10).unwrap() as u128;
                    self.second_number = (self.second_number * self.tens) + temp;
                    self.memory.push(token);
                } else if token == ')' {
                    if self.tens == 1 {
                        self.reset();
                    } else {
                        self.memory.push(token);
                        if self.enabled {
                            self.output = self.output + (self.first_number as u128 * self.second_number as u128) as u128;
                        } else {
                        };
                        self.reset();
                    }
                } else {
                    self.reset();
                }
            },
        }
    }
}

impl Problem for DayThree {
    fn part_one(&self) -> String {
        let content = read_file("src/inputs/day3.txt");    
        let mut machine = initialize_machine();
        for token in content.chars() {
            machine.run_state_machine(token);
        }
        format!("{}", machine.output)
    }
    
    fn part_two(&self) -> String {
        let content = read_file("src/inputs/day3.txt");    
        let mut machine = initialize_machine();
        for token in content.chars() {
            machine.run_state_machine_with_do_sequence(token);
        }
        format!("{}", machine.output)
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
}