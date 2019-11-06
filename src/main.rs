use std::fmt;
use std::mem;
use std::process;
use std::io::prelude::*;
use std::io::{stdin, stdout};
use byteorder::{ByteOrder, BigEndian};

static HELP_TEXT: &str = "
SQLiter: A SQLite clone written in Rust.

Available meta commands: 
    .exit : Exit the shell
    .help : Display this help message

Available statements: 
    insert: Add a record to the database 
    select: Display records from the database 
";

const PROMPT: &str = "db > ";
const ID_SIZE: usize = mem::size_of::<u32>();
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 256;

const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
const ROW_SIZE: usize = mem::size_of::<Row>();

const PAGE_SIZE: usize = 4096;
const TABLE_MAX_PAGES: usize = 32;

const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

enum MetaCommandResult {
  Success,
  Unrecognized
}

enum StatementType {
  Insert,
  Select
}

struct Statement {
  typ: StatementType,
  row: Option<Row>
}

#[derive(Clone, Debug)]
struct Row {
  id: u32,
  username: String,
  email: String
}

impl fmt::Display for Row {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Row {}, Username: {}, Email: {}", self.id, self.username, self.email)
  }
}

impl Row {
  fn serialize(&self) -> Vec<u8> {
    let mut buf = vec![0; ROW_SIZE];
    BigEndian::write_u32(&mut buf, self.id);
    Row::write_string(&mut buf, ID_SIZE, USERNAME_SIZE, &self.username);
    Row::write_string(&mut buf, EMAIL_OFFSET, EMAIL_SIZE, &self.email);

    buf
  }

  fn deserialize(buf: Vec<u8>) -> Row {
    let id = BigEndian::read_u32(&buf);
    let username = Row::read_string(&buf, ID_SIZE, USERNAME_SIZE);
    let email = Row::read_string(&buf, EMAIL_OFFSET, EMAIL_SIZE);

    Row { id, username, email }
  }

  fn write_string(buf: &mut Vec<u8>, pos: usize, max_len: usize, s: &str) {
    let bytes = s.as_bytes().to_owned();
    let mut offset = 0;

    for b in bytes {
      buf[pos + offset] = b;
      offset += 1;
    }

    while offset < max_len {
      buf[pos + offset] = 0;
      offset += 1;
    }
  }

  fn read_string(buf: &[u8], pos: usize, max_len: usize) -> String {
    let mut end = pos;

    while end < max_len + pos && buf[end] != 0 {
      end += 1;
    }

    let bytes = buf[pos..end].to_vec();

    String::from_utf8(bytes).unwrap()
  }
}

#[derive(Debug)]
enum PrepareErr {
  SyntaxError(String),
  UnrecognizedError(String)
}

type PrepareResult<T> = Result<T, PrepareErr>;

#[derive(Debug)]
enum ExecuteErr {
  TableFull
}

type ExecuteResult<T> = Result<T, ExecuteErr>;

fn do_meta_command(command: &str) -> MetaCommandResult {
  match command.trim() {
    ".exit" => process::exit(0),
    ".help" => {
      println!("{}", HELP_TEXT);
      MetaCommandResult::Success
    },
    _ => {
      println!("Unrecognized meta command");
      MetaCommandResult::Unrecognized
    }
  }
}

fn prepare_statement(command: &str) -> PrepareResult<Statement> {
  if command.starts_with("insert") {
    Ok(Statement { typ: StatementType::Insert, row: None })
  } else if command.starts_with("select") {
    Ok(Statement { typ: StatementType::Select, row: None })
  } else {
    Err(PrepareErr::UnrecognizedError("Unrecognized prepare statement".to_string()))
  }
}

fn execute_statement(s: &Statement) -> ExecuteResult<()> {
  match s.typ {
    StatementType::Insert => println!("Performing INSERT"),
    StatementType::Select => println!("Performing SELECT")
  }

  Ok(())
}

fn print_prompt() {
  print!("{}", PROMPT);
  stdout().flush().expect("failed to flush to stdout");
}

fn read_input() -> String {
  let mut buffer = String::new();
  stdin().read_line(&mut buffer).expect("failed to read from stdin");

  buffer
}

fn main() {
  loop {
    print_prompt();
    let mut buffer = read_input();
    buffer = buffer.trim().to_string();

    if buffer.starts_with('.') {
      match do_meta_command(&buffer) {
        MetaCommandResult::Success => continue,
        MetaCommandResult::Unrecognized => continue
      }
    }

    if let Ok(s) = prepare_statement(&buffer) {
      match s.typ {
        StatementType::Insert => println!("Performing INSERT"),
        StatementType::Select => println!("Performing SELECT")
      }
    } else {
      println!("Unrecognized keyword at start of {}", buffer);
    }
  }
}
