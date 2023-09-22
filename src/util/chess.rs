
use std::fmt::Display;
use std::{str::*, slice::Chunks};
use std::ops::{Index, IndexMut};
use std::iter::*;
use std::collections::HashMap;
use num_enum::{IntoPrimitive, TryFromPrimitive};
#[derive(IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
enum Piece {
  WKing = 0,
  BKing = 1,
  WQueen = 2,
  BQueen = 3,
  WRook = 4,
  BRook = 5,
  WBishop = 6,
  BBishop = 7,
  WKnight = 8,
  BKnight = 9,
  WPawn = 10,
  BPawn = 11,
  Empty = 12
}


const PIECE_CHAR_MAP:[char; 13] = ['K', 'k', 'Q', 'q', 'R', 'r', 'B', 'b', 'N', 'n', 'P', 'p', '#'];


const WHITE_PIECES:[Piece; 6] = [Piece::WKing, Piece::WQueen, Piece::WRook, Piece::WBishop, Piece::WKnight, Piece::WPawn];
const BLACK_PIECES:[Piece; 6] = [Piece::BKing, Piece::BQueen, Piece::BRook, Piece::BBishop, Piece::BKnight, Piece::BPawn];

const DEFAULT_BOARD:[Piece; 64] = [
  Piece::BRook, Piece::BKnight, Piece::BBishop, Piece::BQueen, Piece::BKing, Piece::BBishop, Piece::BKnight, Piece::BRook,
  Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn,
  Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty,
  Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty,
  Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty,
  Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty,
  Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn,
  Piece::WRook, Piece::WKnight, Piece::WBishop, Piece::WQueen, Piece::WKing, Piece::WBishop, Piece::WKnight, Piece::WRook
];





const default_pieces:[u8;32] = [
  60, //white king
  59, //white queen
  63, 56, //white rooks
  61, 58, //white bishops
  62, 57, //white knights
  47, 48, 49, 50, 51, 52, 53, 54, //white pawns
  4, //black king
  3, //black queen
  0, 7, //black rooks
  2, 5, //black bishops
  1, 6, //black knights
  8, 9, 10, 11, 12, 13, 14, 15 //black pawns
];



pub struct PieceSet {
  w_king:Vec<u8>,
  b_king:Vec<u8>,
  w_queen:Vec<u8>,
  b_queen:Vec<u8>,
  w_rook:Vec<u8>,
  b_rook:Vec<u8>,
  w_bishop:Vec<u8>,
  b_bishop:Vec<u8>,
  w_knight:Vec<u8>,
  b_knight:Vec<u8>,
  w_pawn:Vec<u8>,
  b_pawn:Vec<u8>,
}
impl PieceSet {
  fn empty_default() -> Self {
    return PieceSet {
      w_king: vec![], 
      b_king: vec![], 
      w_queen: vec![], 
      b_queen: vec![], 
      w_rook: vec![], 
      b_rook: vec![], 
      w_bishop: vec![], 
      b_bishop: vec![], 
      w_knight: vec![], 
      b_knight: vec![], 
      w_pawn: vec![], 
      b_pawn: vec![] 
    }
  }
}
impl Default for PieceSet{
  fn default() -> Self {
      return PieceSet {
        w_king: vec![60],
        b_king: vec![4],
        w_queen: vec![59],
        b_queen: vec![3],
        w_rook: vec![63, 56],
        b_rook: vec![0, 7],
        w_bishop: vec![61, 58],
        b_bishop: vec![2, 5],
        w_knight: vec![62, 57],
        b_knight: vec![1, 6],
        w_pawn: vec![47, 48, 49, 50, 51, 52, 53, 54,],
        b_pawn: vec![8, 9, 10, 11, 12, 13, 14, 15],
      }
  }
}
impl Index<Piece> for PieceSet {
  type Output = Vec<u8>;
  fn index(&self, index: Piece) -> &Self::Output {
      match index {
          Piece::WKing => &self.w_king,
          Piece::WQueen => &self.w_queen,
          Piece::WRook => &self.w_rook,
          Piece::WBishop => &self.w_bishop,
          Piece::WKnight => &self. w_knight,
          Piece::WPawn => &self.w_pawn,
          Piece::BKing => &self.b_king,
          Piece::BQueen => &self.b_queen,
          Piece::BRook => &self.b_rook,
          Piece::BBishop => &self.b_bishop,
          Piece::BKnight => &self.b_knight,
          Piece::BPawn => &self.b_pawn,
          Piece::Empty => unimplemented!()
      }
  }
}
impl IndexMut<Piece> for PieceSet {
  fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
      match index {
          Piece::WKing => &mut self.w_king,
          Piece::WQueen => &mut self.w_queen,
          Piece::WRook => &mut self.w_rook,
          Piece::WBishop => &mut self.w_bishop,
          Piece::WKnight => &mut self. w_knight,
          Piece::WPawn => &mut self.w_pawn,
          Piece::BKing => &mut self.b_king,
          Piece::BQueen => &mut self.b_queen,
          Piece::BRook => &mut self.b_rook,
          Piece::BBishop => &mut self.b_bishop,
          Piece::BKnight => &mut self.b_knight,
          Piece::BPawn => &mut self.b_pawn,
          Piece::Empty => unimplemented!()
      }
  }
}



pub struct Move {
  from: usize,
  to: usize
}

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
  fn move_str(&mut self, s: String){
    let p:Vec<char> = vec!['K', 'Q', 'R', 'B', 'N'];
    let is_last_char_check = s.chars().rev().next().unwrap() == '+';

    if s.contains('0') {
      let count = s.chars().fold(0, |acc, c| acc + (c=='0') as u8);
      if count == 3 {
        if self.turn == 0 {
          let king_sq = alg_square("e1");
          let rook_sq = alg_square("a1");
        
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