

use std::{str::*, slice::Chunks};

use std::iter::*;
use std::collections::HashMap;


use num_enum::{TryFromPrimitive, FromPrimitive};

use crate::util::consts::*;
use crate::util::types::*;







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
        m.arg1 = b.piece_set[Piece::from_primitive(b.turn)][0];
      }
      'Q' => {
        
      }
      'R' => {

      }
      'B' => {

      }
      'N' => {

      }
    }
  }





}


//TODOS: Refactor this section      
const WHITE_DISCOVERED_ATTACK_PIECES:[u8; 3] = [Piece::WQueen as u8, Piece::WRook as u8, Piece::WBishop as u8];
const BLACK_DISCOVERED_ATTACK_PIECES:[u8; 3] = [Piece::BQueen as u8, Piece::BRook as u8, Piece::BBishop as u8];
#[inline(always)]
fn does_move_leave_king_in_check(mut b: Board, m: Move)->bool {

  let opposing_pieces = if b.turn == 1 {WHITE_DISCOVERED_ATTACK_PIECES} else {BLACK_DISCOVERED_ATTACK_PIECES};
  let king_pos:i8 = if b.turn == 1 {b.piece_set.b_king[0] as i8} else {b.piece_set.w_king[0] as i8};
  
  //the only pieces we need to check are bishops, rooks and queens.
  //diagonal moves either change position by a multiple of 9 or 6
  
  
  for bishop_pos in b.piece_set[Piece::try_from(opposing_pieces[2]).unwrap()].iter() {
    let diff:i8 = (king_pos as i8 - *bishop_pos as i8);
    let mod_9 = diff % 9;
    if (diff % 7 == 0){
      let div_7 = diff/7;
      for i in (1..(div_7.signum()*div_7)){
        let checked_position = (div_7.signum()*7*i + king_pos) as usize;
        if(b.sq[checked_position]==Piece::Empty as u8){
          return true;
        }
      }
    }
    if (diff % 9 == 0){
      let div_9 = diff/9;
      for i in (1..(div_9.signum()*div_9)){
        let checked_position = (div_9.signum()*9*i + king_pos) as usize;
        if(b.sq[checked_position]==Piece::Empty as u8){
          return true;
        }
      }
    }
    
  }
  // for rook_pos in b.piece_set[opposing_pieces[1]] {


  // }
  // for queen_pos in b.piece_set[opposing_pieces[0]] {

  return false;
}




fn find_queen_attack(b:&mut Board, target:u8)->i8{
  for queen in b.piece_set[Piece::from_primitive(2 + b.turn)].iter() {
    let diff:i8 = target as i8 - *queen as i8;
    if diff % 8 == 0 {
      let div_8 = 
    }

  }


  return -1;

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


pub fn board_from_fen(fen: String)->Board{
  let mut board = Board::empty_default();
  let mut _fen = fen.clone();
  let mut fen_vec: Vec<&str> = fen.split(' ').collect();
  let pos_strs = fen_vec[0].split('/').rev();

  fn set_pos(b: &mut Board, piece: u8, idx: &mut u8){
    b.sq[*idx as usize] = piece;
    b.piece_set[Piece::from_primitive(piece)].push(*idx);
    *idx += 1;
  }

  fn set_empty(b: &mut Board, count: char, idx: &mut u8) {
    let n:u8 = ((count as u8) - 48) as u8;
    for c in (0..n){
      b.sq[(*idx + c) as usize] = Piece::Empty.into();
    } 
    *idx += n;
  }
  let mut idx = 0;
  for pos_slice in pos_strs {
    for c in pos_slice.chars() {
      match c {
        'k' => set_pos(&mut board, Piece::BKing.into(), &mut idx),
        'q' => set_pos(&mut board, Piece::BQueen.into(), &mut idx),
        'r' => set_pos(&mut board, Piece::BRook.into(), &mut idx),
        'b' => set_pos(&mut board, Piece::BBishop.into(), &mut idx),
        'n' => set_pos(&mut board, Piece::BKnight.into(), &mut idx),
        'p' => set_pos(&mut board, Piece::BPawn.into(), &mut idx),
        'K' => set_pos(&mut board, Piece::WKing.into(), &mut idx),
        'Q' => set_pos(&mut board, Piece::WQueen.into(), &mut idx),
        'R' => set_pos(&mut board, Piece::WRook.into(), &mut idx),
        'B' => set_pos(&mut board, Piece::WBishop.into(), &mut idx),
        'N' => set_pos(&mut board, Piece::WKnight.into(), &mut idx),
        'P' => set_pos(&mut board, Piece::WPawn.into(), &mut idx),
        '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8' => set_empty(&mut board, c,&mut idx),
        _ => {}
      }
    }
  }
  return board;
}

fn alg_square(s: &str)->u8{
  let x = s.chars().nth(0).unwrap();
  let y = s.chars().nth(0).unwrap();
  let x_int = (x as u8 - 'a' as u8);
  let y_int = (y as u8 - '1' as u8);
  return 8*y_int + x_int;
}