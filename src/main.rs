#[macro_use]
extern crate text_io;
extern crate byteorder;

use std::process;

mod db;
mod cli;
mod pager;

use db::Table;

fn main() {
  let mut table: Table = Table::new();
  let ec = cli::run(&mut table);
  table.close();

  process::exit(ec);
}