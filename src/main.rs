use std::env;
use std::fs;
use std::char;
use std::collections::HashMap;
use std::io::Read;
use std::io;

struct Memory {
  ptr: usize,
  tape: [u8; 30000],
  prog_ptr: usize
}

// add a lexer and parser but find a way to make it work like this too
fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
       println!("Filename missing!");
       return
    }

    let filename = args[1].clone();

    let program = fs::read_to_string(filename)
        .expect("Could not read file");

    let program_chars = program.as_bytes();

    let mut bf: Memory = Memory {
      ptr: 15000,
      tape: [0; 30000],
      prog_ptr: 0
    };

    let mut stack = Vec::new();
    let mut jump_table: HashMap<usize, usize> = HashMap::new();

    for (i, op) in program.chars().enumerate() {
        if op == '[' {
           stack.push(i);
        } else if op == ']' {
          jump_table.insert(i, stack.pop().unwrap());
          jump_table.insert(*jump_table.get(&i).unwrap(), i);
        }
    }

    while bf.prog_ptr < program.len() {

      let op = program_chars[bf.prog_ptr] as char;

      match op { 
        '<' => {
               bf.ptr = bf.ptr.wrapping_sub(1);
        } 
        '>' => {
               bf.ptr = bf.ptr.wrapping_add(1);
        }
        '+' => { 
            bf.tape[bf.ptr] = bf.tape[bf.ptr].wrapping_add(1);
        }
        '-' => { 
            bf.tape[bf.ptr] = bf.tape[bf.ptr].wrapping_sub(1);
        }
        '.' => { print!("{}", bf.tape[bf.ptr] as char); }
        ',' => {
                let input: u8 = std::io::stdin()
                .bytes() 
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as u8).unwrap_or(0); // ctrl+d for EOF
                bf.tape[bf.ptr] = input;
        }
        '[' => {
               if bf.tape[bf.ptr] == 0 {
                  bf.prog_ptr = *jump_table.get(&bf.prog_ptr).unwrap();
               }
        }
        ']' => {

            if bf.tape[bf.ptr] != 0 {
               bf.prog_ptr = *jump_table.get(&bf.prog_ptr).unwrap();
            }
        }
        _ => {}
      }

      bf.prog_ptr += 1;
    }

}
