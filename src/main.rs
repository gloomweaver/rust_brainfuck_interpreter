use std::iter;

#[derive(Clone, Debug, Copy)]
pub enum Commands {
    Forward,
    Back,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

pub struct Interpreter {
    memory: Vec<u8>,
    size: usize,
    commands: Vec<Option<Commands>>,
    input: Vec<u8>,
    output: Vec<u8>,
    pointer: usize,
    command_pointer: usize,
}

impl Interpreter {
    pub fn new(size: usize) -> Interpreter {
        Interpreter {
            memory: iter::repeat(0 as u8).take(size).collect(),
            size: size,
            commands: Vec::new(),
            input: Vec::new(),
            output: Vec::new(),
            pointer: 0,
            command_pointer: 0,
        }
    }

    fn parse_to_commands(&mut self, commands_string: &str) -> () {
        self.commands = commands_string
            .chars()
            .map(|ch| -> Option<Commands> {
                match ch {
                    '+' => Some(Commands::Increment),
                    '-' => Some(Commands::Decrement),
                    '>' => Some(Commands::Forward),
                    '<' => Some(Commands::Back),
                    '.' => Some(Commands::Output),
                    ',' => Some(Commands::Input),
                    '[' => Some(Commands::LoopStart),
                    ']' => Some(Commands::LoopEnd),
                    _ => None,
                }
            })
            .collect();
    }

    fn get_input(&mut self, input: Vec<u8>) -> () {
        self.input = input;
        self.input.reverse();
    }

    pub fn get_output(self) -> Vec<u8> {
        self.output
    }

    pub fn run(&mut self, commands_string: &str, input: Vec<u8>) {
        self.parse_to_commands(commands_string);
        self.get_input(input);
        while self.command_pointer < self.commands.len() {
            let command = self.commands[self.command_pointer];
            match command {
                Some(Commands::Forward) => self.forward(),
                Some(Commands::Back) => self.back(),
                Some(Commands::Increment) => self.increment(),
                Some(Commands::Decrement) => self.decrement(),
                Some(Commands::Input) => self.input(),
                Some(Commands::Output) => self.output(),
                Some(Commands::LoopStart) => self.loop_start(),
                Some(Commands::LoopEnd) => self.loop_end(),
                _ => (),
            }
            self.command_pointer += 1;
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

    fn input(&mut self) {
        let input_char = self.input.pop();
        if input_char.is_some() {
            let value = input_char.unwrap();
            if value > 0 {
                self.memory[self.pointer] = input_char.unwrap();
            }
        }
    }

    fn output(&mut self) {
        let output_char = self.memory[self.pointer];
        self.output.push(output_char)
    }

    fn skip_back(&mut self) {
        let mut count: u8 = 1;
        let mut op;
        while count > 0 {
            self.command_pointer -= 1;
            op = self.commands[self.command_pointer];
            match op {
                Some(Commands::LoopStart) => count -= 1,
                Some(Commands::LoopEnd) => count += 1,
                _ => (),
            }
        }
    }

    fn skip_forward(&mut self) {
        let mut count: u8 = 1;
        let mut op;
        while count > 0 {
            self.command_pointer += 1;
            op = self.commands[self.command_pointer];
            match op {
                Some(Commands::LoopStart) => count += 1,
                Some(Commands::LoopEnd) => count -= 1,
                _ => (),
            }
        }
    }

    fn loop_start(&mut self) {
        if self.memory[self.pointer] == 0 {
            self.skip_forward()
        }
    }

    fn loop_end(&mut self) {
        if self.memory[self.pointer] > 0 {
            self.skip_back()
        }
    }
}

fn main() {
    let mut interpreter = Interpreter::new(10);
    interpreter.run(",>,>,.<.<.", "HEllo".to_string().as_bytes().to_vec());
    println!("{:?}", interpreter.get_output());
}
