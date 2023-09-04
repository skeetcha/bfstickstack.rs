use std::io::{self, Read};
use std::io::Write;
use std::str;

fn interpret(input: &String, stack: &mut Vec<i64>, tape: &mut [i64; 30000], tape_loc: &mut usize, stack_mode: &mut bool) {
    let mut prog_loc = 0;
    let mut tape_locs: Vec<usize> = Vec::new();
    let mut stack_locs: Vec<usize> = Vec::new();

    while prog_loc < input.len() {
        match input.chars().nth(prog_loc).unwrap() {
            '>' => {
                if *stack_mode {
                    stack.push(0);
                } else {
                    if *tape_loc < 30000 {
                        *tape_loc += 1;
                    }
                }
            },
            '<' => {
                if *stack_mode {
                    if stack.len() == 0 {
                        println!("Empty stack");
                    } else {
                        let _ = stack.pop().unwrap();
                    }
                } else {
                    if *tape_loc > 0 {
                        *tape_loc -= 1;
                    }
                }
            },
            '+' => {
                if *stack_mode {
                    if stack.len() == 0 {
                        println!("Empty stack");
                    } else {
                        *stack.last_mut().unwrap() += 1;
                    }
                } else {
                    tape[*tape_loc] += 1;
                }
            },
            '-' => {
                if *stack_mode {
                    if stack.len() == 0 {
                        println!("Empty stack");
                    } else {
                        *stack.last_mut().unwrap() -= 1;
                    }
                } else {
                    tape[*tape_loc] -= 1;
                }
            },
            '.' => {
                if *stack_mode && stack.len() == 0 {
                    println!("Empty stack");
                } else {
                    let bytes: [u8; 8];

                    if *stack_mode {
                        bytes = stack.last().unwrap().to_le_bytes();
                    } else {
                        bytes = tape[*tape_loc].to_le_bytes();
                    }

                    print!("{}", str::from_utf8(&bytes).unwrap());
                }
            },
            ',' => {
                let mut buf: [u8; 1] = [b'\0'];
                io::stdin().read_exact(&mut buf).expect("failed to read character");

                if *stack_mode {
                    stack.push(buf[0] as i64);
                } else {
                    tape[*tape_loc] = buf[0] as i64;
                }
            },
            '[' => {
                if tape[*tape_loc] == 0 {
                    let mut matches = 1;

                    while matches != 0 {
                        prog_loc += 1;

                        if prog_loc == input.len() {
                            break;
                        } else if input.chars().nth(prog_loc).unwrap() == '[' {
                            matches += 1;
                        } else if input.chars().nth(prog_loc).unwrap() == ']' {
                            matches -= 1;
                        }
                    }
                } else {
                    tape_locs.push(prog_loc);
                }
            },
            ']' => {
                prog_loc = tape_locs.pop().unwrap() - 1;
            },
            '{' => {
                if *stack.last().unwrap() == 0 {
                    let mut matches = 1;

                    while matches != 0 {
                        prog_loc += 1;

                        if prog_loc == input.len() {
                            break;
                        } else if input.chars().nth(prog_loc).unwrap() == '[' {
                            matches += 1;
                        } else if input.chars().nth(prog_loc).unwrap() == ']' {
                            matches -= 1;
                        }
                    }
                } else {
                    stack_locs.push(prog_loc);
                }
            },
            '}' => {
                prog_loc = stack_locs.pop().unwrap() - 1;
            },
            '/' => {
                *stack_mode = !*stack_mode;
            },
            _ => ()
        }

        prog_loc += 1;
    }
}

fn main() {
    let mut stack: Vec<i64> = Vec::new();
    let mut tape: [i64; 30000] = [0; 30000];
    let mut tape_loc: usize = 0;
    let mut stack_mode = false;
    println!("Rust BFStickStack Interpreter v0.1\nWritten by Cass Unterholzner");
    let mut quit = false;

    while !quit {
        print!(">>> ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to readline");
        input = input.strip_suffix('\n').unwrap().to_string();

        if input == "quit" {
            quit = true;
        } else {
            interpret(&input, &mut stack, &mut tape, &mut tape_loc, &mut stack_mode);
        }
    }
}
