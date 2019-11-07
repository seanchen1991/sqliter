use std::mem;
use std::fmt;
use byteorder::{ByteOrder, BigEndian};

use crate::pager::{Pager, PAGE_SIZE};

const ID_SIZE: usize = mem::size_of::<u32>();
const USERNAME_SIZE: usize = 32;
const EMAIL_SIZE: usize = 256;
const ID_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;

const ROW_SIZE: usize = mem::size_of::<Row>();
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
pub const TABLE_MAX_PAGES: usize = 100;
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Row {
  pub id: u32,
  pub username: String,
  pub email: String,
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
    Row::write_string(&mut buf, USERNAME_OFFSET, USERNAME_SIZE, &self.username);
    Row::write_string(&mut buf, EMAIL_OFFSET, EMAIL_SIZE, &self.email);

    buf
  }

  fn deserialize(buf: Vec<u8>) -> Row {
    Row {
      id: BigEndian::read_u32(&buf),
      username: Row::read_string(&buf, USERNAME_OFFSET, USERNAME_SIZE),
      email: Row::read_string(&buf, EMAIL_OFFSET, EMAIL_SIZE)
    }
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

pub struct Table {
  pager: Pager,
  pub num_rows: usize,
}

impl fmt::Display for Table {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    for i in 0..self.num_rows {
      write!(f, "{}", self.read_row(i))?;
    }

    write!(f, "----------------------------")
  }
}

impl Table {
  pub fn new() -> Self {
    let pager = Pager::new();
    
    Table { pager, num_rows: pager.num_pages }
  }

  pub fn close(&mut self) {
    self.pager.close();
  }

  pub fn insert_row(&mut self, row: Row) {
    let bytes = row.serialize();

    let page_index = self.num_rows / ROWS_PER_PAGE;
    let page = self.pager.page_to_write(page_index);
    let row_index = (self.num_rows % ROWS_PER_PAGE) * ROW_SIZE;

    for (i, byte) in bytes.iter().enumerate() {
      page[row_index + i] = *byte;
    }

    self.num_rows += 1;
  }

  pub fn read_row(&mut self, num_rows: usize) -> Row {
    let page_index = num_rows / ROWS_PER_PAGE;
    let page = self.pager.page_to_read(page_index);
    let row_index = (num_rows % ROWS_PER_PAGE) * ROW_SIZE;
    let bytes = page[row_index..row_index + ROW_SIZE].to_vec();

    Row::deserialize(bytes)
  }
}