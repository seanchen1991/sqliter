use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

use crate::db::{TABLE_MAX_PAGES};

pub const PAGE_SIZE: usize = 4096;

type Page = Vec<u8>;

static FILE_PATH: &str = "/Users/seanchen/Projects/sqliter/database.db";

#[derive(Clone, Debug)]
pub struct Pager {
  pub file: File,
  pub pages: Vec<Option<Page>>,
  pub num_pages: usize,
}

impl Pager {
  pub fn new() -> Self {
    let file = OpenOptions::new()
      .read(true)
      .create(true)
      .write(true)
      .open(FILE_PATH)
      .expect("Failed to open database file");
    let pages = vec![None; TABLE_MAX_PAGES];
    let num_pages = (file.metadata().unwrap().len() / PAGE_SIZE as u64) as usize;

    Pager {
      file,
      pages,
      num_pages,
    }
  }

  pub fn close(&mut self) {
    for i in 0..self.num_pages {
      self.flush(i);
    }
  }

  fn flush(&mut self, page_index: usize) {
    let offset = page_index * PAGE_SIZE;

    if let Some(mut page) = self.pages[page_index] {
      self.file.seek(SeekFrom::Start(offset as u64)).unwrap();
      self.file.write_all(page.as_mut_slice()).unwrap();
    }
  }

  pub fn page_to_read(&mut self, page_index: usize) -> &Page {
    if page_index > TABLE_MAX_PAGES {
      panic!("Reached EOF");
    }

    if self.pages[page_index].is_none() {
      self.load(page_index);
    }

    self.pages[page_index].as_ref().unwrap()
  }

  pub fn page_to_write(&mut self, page_index: usize) -> &mut Page {
    if page_index > TABLE_MAX_PAGES {
      panic!("Reached EOF");
    }

    if self.pages[page_index].is_none() {
      let page = vec![0; PAGE_SIZE];
      self.pages[page_index] = Some(page);
      self.num_pages += 1;
    }

    self.pages[page_index].as_mut().unwrap()
  }

  fn load(&mut self, page_index: usize) {
    let offset = page_index * PAGE_SIZE;
    let mut buf = vec![0; PAGE_SIZE];
    self.file.seek(SeekFrom::Start(offset as u64)).unwrap();
    self.file.read(buf.as_mut_slice()).unwrap();
    self.pages[page_index] = Some(buf);
  }
}


