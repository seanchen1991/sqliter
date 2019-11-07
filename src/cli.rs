use text_io;
use std::fmt;
use std::error::Error;
use std::io::{self, Write};

use crate::db::{Row, Table, TABLE_MAX_ROWS};

static HELP_TEXT: &str = "
This is an SQLite clone written in Rust.
Available meta commands:
    .exit : Quit the interactive shell
    .help : Display this help message
Available statements:
    insert : Add a record in the database
    select : Display records from the database
";

enum StatementType {
  Insert,
  Select,
}

struct Statement {
  kind: StatementType,
  row: Option<Row>,
}

#[derive(Debug)]
enum PrepareError {
  StringLength,
  Syntax,
  Unrecognized,
}

impl fmt::Display for PrepareError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      PrepareError::StringLength => write!(f, "Error: String is too long"),
      PrepareError::Syntax => write!(f, "Error: Could not parse statement"),
      PrepareError::Unrecognized => write!(f, "Error: Unrecognized keyword at start of statement"),
    }
  }
}

impl Error for PrepareError {
  fn description(&self) -> &str {
    match *self {
      PrepareError::StringLength => "Error: String is too long",
      PrepareError::Syntax => "Error: Could not parse statement",
      PrepareError::Unrecognized => "Error: Unrecognized keyword at start of statement",
    }
  }
}

#[derive(Debug)]
pub enum ExecuteError {
  TableFull,
  UnrecognizedMetaCommand,
}

impl Error for ExecuteError {
  fn description(&self) -> &str {
    match *self {
      ExecuteError::TableFull => "Error: The table is full",
      ExecuteError::UnrecognizedMetaCommand => "Error: Unrecognized meta command",
    }
  }
}

impl fmt::Display for ExecuteError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      ExecuteError::TableFull => write!(f, "Error: The table is full"),
      ExecuteError::UnrecognizedMetaCommand => write!(f, "Error: Unrecognized meta command"),
    }
  }
}

fn print_prompt() {
  print!("db > ");
  io::stdout().flush().expect("Failed to flush to stdout");
}

fn read_input() -> String {
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer).expect("Failed to read stdin");

  buffer
}

fn do_meta_command(buffer: &str) -> Result<Option<i32>, ExecuteError> {
  match buffer.trim() {
    ".exit" => {
      Ok(Some(0))
    }
    ".help" => {
      println!("{}", HELP_TEXT);
      Ok(None)
    }
    _ => Err(ExecuteError::UnrecognizedMetaCommand),
  }
}

fn prepare_statement(input_buffer: String) -> Result<Statement, PrepareError> {
  let statement: &str = input_buffer.trim().as_ref();
  match statement {
    _ if statement.starts_with("insert") => {
      let id: u32;
      let username: String;
      let email: String;
      let scan_result = parse_insert(statement);
      match scan_result {
        Ok((_id, _username, _email)) => {
          id = _id;
          username = _username;
          email = _email;
        }
        Err(_err) => return Err(PrepareError::Syntax),
      };

      if username.len() > 32 || email.len() > 256 {
        return Err(PrepareError::StringLength);
      }

      let row: Row = Row {
        id,
        username,
        email,
      };
      Ok(Statement {
        kind: StatementType::Insert,
        row: Some(row),
      })
  }
  _ if statement.starts_with("select") => {
      Ok(Statement {
        kind: StatementType::Select,
        row: Default::default(),
      })
    }
    _ => Err(PrepareError::Unrecognized),
  }
}

fn parse_insert(statement: &str) -> Result<(u32, String, String), text_io::Error> {
  let id: u32;
  let username: String;
  let email: String;
  try_scan!(statement.bytes() => "insert {} {} {}", id, username, email);
  
  Ok((id, username, email))
}

fn execute_statement(statement: Statement, table: &mut Table) -> Result<(), ExecuteError> {
  match statement.kind {
    StatementType::Insert => {
      if table.num_rows >= TABLE_MAX_ROWS {
        return Err(ExecuteError::TableFull);
      }
      let row_to_insert = statement.row.unwrap();
      table.insert_row(row_to_insert);
      Ok(())
    }
    StatementType::Select => {
      println!("{}", table);
      Ok(())
    }
  }
}

pub fn run(table: &mut Table) -> i32 {
  loop {
    print_prompt();
    let mut buffer = read_input();
    buffer = buffer.trim().to_string();

    if buffer.chars().next() == Some('.') {
      match do_meta_command(&buffer) {
        Ok(Some(exit_code)) => return exit_code,
        Ok(None) => continue,
        Err(e) => {
            println!("{}.", e.description());
            continue;
        }
      }
    }

    let statement = prepare_statement(buffer);

    match statement {
      Ok(statement) => {
        match execute_statement(statement, table) {
          Ok(()) => println!("Executed."),
          Err(e) => println!("{}.", e.description()),
        }
      }
      Err(e) => println!("{}.", e.description()),
    }
  }
}