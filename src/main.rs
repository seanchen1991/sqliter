use std::io;
use std::process;
use std::io::prelude::*;

static PROMPT: &str = "db > ";

enum MetaCommandResult {
  Success,
  Unrecognized
}

enum StatementType {
  Insert,
  Select
}

struct Statement {
  typ: StatementType
}

#[derive(Debug)]
enum ExecuteErr {
  Reason(String)
}

type ExecuteResult<T> = Result<T, ExecuteErr>;

fn do_meta_command(command: &str) -> MetaCommandResult {
  match command.trim() {
    ".exit" => process::exit(0),
    _ => MetaCommandResult::Unrecognized
  }
}

fn prepare_statement(command: &str) -> Option<Statement> {
  if command.starts_with("insert") {
    Some(Statement { typ: StatementType::Insert })
  } else if command.starts_with("select") {
    Some(Statement { typ: StatementType::Select })
  } else {
    None
  }
}

// fn execute_statement(s: &Statement) -> ExecuteResult<()> {
//   match s.typ {
//     StatementType::Insert => println!("Performing INSERT"),
//     StatementType::Select => println!("Performing SELECT")
//   }

//   Ok(())
// }

fn main() {
  loop {
    print!("{}", PROMPT);
    io::stdout().flush().expect("failed to flush to stdout");

    let mut command = String::new();
    io::stdin().read_line(&mut command).expect("failed to read from stdin");

    if command.starts_with('.') {
      match do_meta_command(&command) {
        MetaCommandResult::Success => continue,
        MetaCommandResult::Unrecognized => {
          print!("Unrecognized command: {}", command);
          continue;
        } 
      }
    }

    if let Some(s) = prepare_statement(&command) {
      match s.typ {
        StatementType::Insert => println!("Performing INSERT"),
        StatementType::Select => println!("Performing SELECT")
      }
    } else {
      print!("Unrecognized keyword at start of {}", command);
    }
  }
}
