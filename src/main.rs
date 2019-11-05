use std::io;
use std::io::prelude::*;

static PROMPT: &str = "db > ";

fn main() {
  loop {
    print!("{}", PROMPT);
    io::stdout().flush().expect("failed to flush to stdout");

    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("failed to read from stdin");

    if command.trim() == ".exit" {
      break;
    } else {
      println!("Unrecognized command: {}", command);
    }
  }
}
