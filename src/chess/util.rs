use bitvec::{prelude::*, view::BitView};
use crate::chess::types::*;
use std::path::Path;
use std::ascii::*;
use std::io::prelude::*;
use std::fs::File;
pub fn print_bb(b:u64){
  let slice = b.view_bits::<Msb0>();
  for x in (0..8){
    for y in (0..8){
      print!("{:3?}", slice[x*8 + y] as u8);
    }
    print!("\n");
  }
}

pub fn retrieve_fens(path: String, out:&mut Vec<String>){
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

  
  for x in s.split('\n').into_iter(){
    out.push(x.to_string());
  };
}


pub fn parse_fens(strs: &Vec<String>)->Vec<Board>{
  let mut boards = Vec::<Board>::with_capacity(strs.len());
  for s in strs {
    boards.push(Board::board_from_fen(s.to_string()));
  }
  return boards;
}



pub fn san_square_to_index(s: &[Char]) -> u8{
  let first = s[0] as u8 - 'a' as u8;
  let second = s[1] as u8 - '1' as u8;
  return (7 - first) * 8 + second;
}

