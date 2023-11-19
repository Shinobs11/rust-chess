

use std::{str::*, slice::Chunks};

use std::iter::*;
use std::collections::HashMap;


use num_enum::{TryFromPrimitive, FromPrimitive};

use crate::chess::consts::*;
use crate::chess::types::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn retrieve_fens(path: String)->Vec<String>{
  let p = Path::new(&path);
  let display = path.to_string();
  let mut file = match File::open(p) {
    Err(why) => panic!("couldn't open {}: {}", display, why),
    Ok(file) => file
  };

  let mut s = String::new();
  match file.read_to_string(&mut s) {
    Err(why) => panic!("couldn't read {}: {}", display, why),
    Ok(_) => {},
  }

  let mut v = Vec::<String>::new();
  for x in s.split('\n').into_iter(){
    v.push(x.clone().to_string());
  };
  return v;
}


pub fn parse_fens(strs: Vec<String>)->Vec<Board>{
  let mut boards = Vec::<Board>::with_capacity(strs.len());
  for s in strs {
    boards.push(Board::board_from_fen(s));
  }
  return boards;
}












fn alg_square(s: &str)->u8{
  let x = s.chars().nth(0).unwrap();
  let y = s.chars().nth(0).unwrap();
  let x_int = (x as u8 - 'a' as u8);
  let y_int = (y as u8 - '1' as u8);
  return 8*y_int + x_int;
}