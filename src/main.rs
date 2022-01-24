use std::iter;

#[derive(Clone, Debug, Copy)]
pub enum Commands {
    Forward,
    Back,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart { index: usize },
    LoopEnd { index: usize },
}

pub struct Interpreter {
    memory: Vec<u8>,
    size: usize,
    commands: Vec<Option<Commands>>,
    input: String,
    pointer: usize,
}

impl Interpreter {
    pub fn new(size: usize) -> Interpreter {
        Interpreter {
            memory: iter::repeat(0 as u8).take(size).collect(),
            size: size,
            commands: Vec::new(),
            input: String::new(),
            pointer: 0,
        }
    }

    fn parse_to_commands(&mut self, commands_string: &str) -> () {
        self.commands = commands_string
            .chars()
            .enumerate()
            .map(|(idx, ch)| -> Option<Commands> {
                match ch {
                    '+' => Some(Commands::Increment),
                    '-' => Some(Commands::Decrement),
                    '>' => Some(Commands::Forward),
                    '<' => Some(Commands::Back),
                    '.' => Some(Commands::Output),
                    ',' => Some(Commands::Input),
                    '[' => Some(Commands::LoopStart { index: idx }),
                    ']' => Some(Commands::LoopEnd { index: idx }),
                    _ => None,
                }
            })
            .collect();
    }

    fn get_input(&mut self, input: &str) -> () {
        self.input = input.to_string()
    }

    pub fn run(&mut self, commands_string: &str, input: &str) {
        self.parse_to_commands(commands_string);
        self.get_input(input);
        let mut command_pointer: usize = 0;
        while command_pointer < self.commands.len() {
            let command = self.commands[command_pointer];
            match command {
                Some(Commands::Forward) => self.forward(),
                Some(Commands::Back) => self.back(),
                Some(Commands::Increment) => self.increment(),
                Some(Commands::Decrement) => self.decrement(),
                Some(Commands::Input) => (),
                Some(Commands::Output) => (),
                Some(Commands::LoopStart { index }) => (),
                Some(Commands::LoopEnd { index }) => (),
                _ => (),
            }
            println!("{:?}", self.memory);
            command_pointer += 1;
        }
    }

    fn forward(&mut self) {
        self.pointer = if self.pointer == self.size - 1 {
            0
        } else {
            self.pointer + 1
        }
    }

    fn back(&mut self) {
        self.pointer = if self.pointer == 0 {
            self.size - 1
        } else {
            self.pointer - 1
        }
    }

    fn increment(&mut self) {
        let value = self.memory[self.pointer];
        self.memory[self.pointer] = value.wrapping_add(1)
    }

    fn decrement(&mut self) {
        let value = self.memory[self.pointer];
        self.memory[self.pointer] = value.wrapping_sub(1)
    }
}

fn main() {
    let mut interpeter = Interpreter::new(10);
    interpeter.run("+++++>>++++>>++<-", "");
}
