# Rust brainfuck interpreter

## v1.0.0

Naive implementation. Messy code style and abstraction, everything in one place. But it works.

## To Do
- Implement Tape struct (has increment, decrement, write, read methods and pointer state)
- Loop should be recursive type including all other values of type and loop itself
- Enums of operation and instructions
- Operations is just tokens, instructions is actions
- Implement interpreter struct
  - Methods
    - parse - tokenize input to vector of operations (functional)
    - to_instructions - parse tokens and return a vector of instructions (functional)
    - run - parse input, translate to instuctions, and execute them
    - execute - instuction reducer, mutates state
  - State
    - Tape 
    - Input