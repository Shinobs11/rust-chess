

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



fn mov(b: &mut Board, m: Move){

}
fn move_str(b: &mut Board, s: String){
  let p:Vec<char> = vec!['K', 'Q', 'R', 'B', 'N'];
  let is_last_char_check = s.chars().rev().next().unwrap() == '+';
  let is_takes = s.contains('x');
  let mut m = Move::default();
  if s.contains('0') {
    m.kind = 0;
    m.arg1 = (s.chars().fold(0, |acc, c| acc + (c=='0') as u8) == 3) as u8;
    mov(b, m);
    return;
  }
  let first = s.chars().nth(0).unwrap();
  if first.is_ascii_uppercase() {
    match first {
      'K' => {
        //we can safely assume there's only ever going to be one king
        m.arg1 = *b.piece_set[Piece::from_primitive(b.turn)].iter().nth(0).unwrap();
      }
      'Q' => {
        
      }
      'R' => {

      }
      'B' => {

      }
      'N' => {

      }
      _ => {

      }
    }
  }





}






// #[inline(always)]
// fn non_king_does_move_leave_king_in_check(mut b: Board, m: Move, p: u8)->bool{
//   b.sq[m.from] = Piece::Empty as u8;
//   let tmp = b.sq[m.to];
//   b.sq[m.to] = p;



//   b.sq[m.to] = tmp;
//   b.sq[m.from] = p;
//   return true;
// }
// #[inline(always)]
// fn king_does_move_leave_king_in_check(mut b: Board, m: Move, p: u8)->bool {

//   return true;
// }
// #[inline(always)]
// fn is_legal_move_king(mut b: Board, m: Move, p: u8)->bool{
  
//   return true;
// }
// #[inline(always)]
// fn is_legal_move_queen(mut b: Board, m: Move, p: u8)->bool{
//   return true;
// }
// #[inline(always)]
// fn is_legal_move_rook(mut b: Board, m: Move, p: u8)->bool{
//   return true
// }
// #[inline(always)]
// fn is_legal_move_bishop(mut b: Board, m: Move, p: u8)->bool{
//   return true;
// }
// #[inline(always)]
// fn is_legal_move_knight(mut b: Board, m: Move, p: u8)->bool{
//   return true;
// }
// #[inline(always)]
// fn is_legal_move_pawn(mut b: Board, m: Move, p: u8)->bool{
//   return true;
// }


// pub fn is_legal_move(mut b: Board, m: Move)->bool{
//   let piece = b.sq[m.from];
//   if (piece % 2 != b.turn) || (piece != (Piece::Empty as u8)){
//     return false;
//   }

//   match piece {
//     _ if piece < 2 => return is_legal_move_king(b, m, piece),
//     _ if piece < 4 => return is_legal_move_queen(b, m, piece),
//     _ if piece < 6 => return is_legal_move_rook(b, m, piece),
//     _ if piece < 8 => return is_legal_move_bishop(b, m, piece),
//     _ if piece < 10 => return is_legal_move_knight(b, m, piece),
//     _ => return is_legal_move_pawn(b, m, piece)
//   }
// }



fn alg_square(s: &str)->u8{
  let x = s.chars().nth(0).unwrap();
  let y = s.chars().nth(0).unwrap();
  let x_int = (x as u8 - 'a' as u8);
  let y_int = (y as u8 - '1' as u8);
  return 8*y_int + x_int;
}