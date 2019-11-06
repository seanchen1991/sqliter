use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

use crate::db::{TABLE_MAX_PAGES};

pub const PAGE_SIZE: usize = 4096;

type Page = Vec<u8>;

#[derive(Debug)]
pub struct Pager {
  pub file: File,
  pub pages: Vec<Option<Page>>,
  pub num_pages: usize,
}



