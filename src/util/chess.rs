
use std::fmt::Display;
use std::{str::*, slice::Chunks};

use std::iter::*;
use std::collections::HashMap;


use num_enum::{TryFromPrimitive, FromPrimitive};

use crate::util::consts::*;
use crate::util::types::*;





pub struct Board {
  pub sq: [u8; 64],
  pub piece_set: PieceSet,
  pub castle_state: [bool; 4],
  pub en_pessant_sq: u8,
  pub turn: u8, // 0: white, 1: black
  pub draw_count: u8 
}
impl Board {
  fn empty_default() -> Self {
    Board { 
      sq:[0;64], 
      piece_set: PieceSet::empty_default(), 
      castle_state: [true; 4], 
      en_pessant_sq: 64, 
      turn: 0, 
      draw_count: 0 
    }
  }
  fn mov(&mut self, m: Move){

  }
  fn move_str(&mut self, s: String){
    let p:Vec<char> = vec!['K', 'Q', 'R', 'B', 'N'];
    let is_last_char_check = s.chars().rev().next().unwrap() == '+';
    let mut m = Move::default();
    if s.contains('0') {
      m.kind = 0;
      m.arg1 = (s.chars().fold(0, |acc, c| acc + (c=='0') as u8) == 3) as u8;
      self.mov(m);
      return;
    }
    let first = s.chars().nth(0).unwrap();
    if first.is_ascii_uppercase() {
      match first {
        'K' => {
          //we can safely assume there's only ever going to be one king
          m.arg1 = self.piece_set[Piece::from_primitive(self.turn)][0]
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

}
impl Default for Board {
  fn default() -> Self {
    Board {sq:DEFAULT_BOARD.map(|x|x as u8), piece_set:PieceSet::default(), castle_state: [true; 4], en_pessant_sq: 64, turn: 0, draw_count: 0}
  }
}

impl Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let mut sq_str = String::new();
      for i in (0..8){
        for j in (0..8){
          sq_str.push(PIECE_CHAR_MAP[self.sq[i*8 + j] as usize]);
        }
        sq_str.push('\n');
      }

      return write!(f, "{}\n{}\n{}\n{}\n{}\n",sq_str, "null", "null", "null", "null");
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







#[inline(always)]
fn non_king_does_move_leave_king_in_check(mut b: Board, m: Move, p: u8)->bool{
  b.sq[m.from] = Piece::Empty as u8;
  let tmp = b.sq[m.to];
  b.sq[m.to] = p;



  b.sq[m.to] = tmp;
  b.sq[m.from] = p;
  return true;
}
#[inline(always)]
fn king_does_move_leave_king_in_check(mut b: Board, m: Move, p: u8)->bool {

  return true;
}
#[inline(always)]
fn is_legal_move_king(mut b: Board, m: Move, p: u8)->bool{
  
  return true;
}
#[inline(always)]
fn is_legal_move_queen(mut b: Board, m: Move, p: u8)->bool{
  return true;
}
#[inline(always)]
fn is_legal_move_rook(mut b: Board, m: Move, p: u8)->bool{
  return true
}
#[inline(always)]
fn is_legal_move_bishop(mut b: Board, m: Move, p: u8)->bool{
  return true;
}
#[inline(always)]
fn is_legal_move_knight(mut b: Board, m: Move, p: u8)->bool{
  return true;
}
#[inline(always)]
fn is_legal_move_pawn(mut b: Board, m: Move, p: u8)->bool{
  return true;
}


pub fn is_legal_move(mut b: Board, m: Move)->bool{
  let piece = b.sq[m.from];
  if (piece % 2 != b.turn) || (piece != (Piece::Empty as u8)){
    return false;
  }

  match piece {
    _ if piece < 2 => return is_legal_move_king(b, m, piece),
    _ if piece < 4 => return is_legal_move_queen(b, m, piece),
    _ if piece < 6 => return is_legal_move_rook(b, m, piece),
    _ if piece < 8 => return is_legal_move_bishop(b, m, piece),
    _ if piece < 10 => return is_legal_move_knight(b, m, piece),
    _ => return is_legal_move_pawn(b, m, piece)
  }
}


pub fn board_from_fen(fen: String)->Board{
  let mut board = Board::empty_default();
  let mut _fen = fen.clone();
  let mut fen_vec: Vec<&str> = fen.split(' ').collect();
  let pos_strs = fen_vec[0].split('/').rev();

  fn set_pos(b: &mut Board, piece: u8, idx: &mut u8){
    b.sq[*idx as usize] = piece;
    b.piece_set[Piece::try_from_primitive(piece).unwrap()].push(*idx);
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