
use std::{str::*, slice::Chunks};
use std::ops::{Index, IndexMut};


use num_enum::IntoPrimitive;
#[derive(IntoPrimitive)]
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
  sq: [i8; 64],
  piece_set: PieceSet,
  castle_state: [bool; 4],
  en_pessant_sq: i8,
  turn: i8, // 0: white, 1: black
  draw_count: i8 
}
impl Default for Board {
  fn default() -> Self {
    Board {sq:[0; 64], piece_set:PieceSet::default(), castle_state: [true; 4], en_pessant_sq: 64, turn: 0, draw_count: 0}
  }
}




//TODOS: Refactor this section      
const WHITE_DISCOVERED_ATTACK_PIECES:[Piece; 3] = [Piece::WQueen, Piece::WRook, Piece::WBishop];
const BLACK_DISCOVERED_ATTACK_PIECES:[Piece; 3] = [Piece::BQueen, Piece::BRook, Piece::BBishop];
fn does_move_leave_king_in_check(mut b: Board, m: Move)->bool {

  let opposing_pieces = if b.turn == 1 {WHITE_DISCOVERED_ATTACK_PIECES} else {BLACK_DISCOVERED_ATTACK_PIECES};
  let king_pos = if b.turn == 1 {b.piece_set.b_king[0]} else {b.piece_set.w_king[0]};
  
  //the only pieces we need to check are bishops, rooks and queens.
  //diagonal moves either change position by a multiple of 9 or 6
  
  for bishop_pos in b.piece_set[opposing_pieces[2]] {
    let diff = (king_pos - bishop_pos);
    if (diff % 7 == 0) {
      if (diff >= 0) {
        
      }
    }

  }
  for rook_pos in b.piece_set[opposing_pieces[1]] {


  }
  for queen_pos in b.piece_set[opposing_pieces[0]] {


  }






}

fn non_king_does_move_leave_king_in_check(mut b: Board, m: Move, p: i8)->bool{
  b.sq[m.from] = Piece::Empty as i8;
  let tmp = b.sq[m.to];
  b.sq[m.to] = p;



  b.sq[m.to] = tmp;
  b.sq[m.from] = p;
  return true;
}
fn king_does_move_leave_king_in_check(mut b: Board, m: Move, p: i8)->bool {

  return true;
}

fn is_legal_move_king(mut b: Board, m: Move, p: i8)->bool{
  
  return true;
}
fn is_legal_move_queen(mut b: Board, m: Move, p: i8)->bool{
  return true;
}
fn is_legal_move_rook(mut b: Board, m: Move, p: i8)->bool{
  return true
}
fn is_legal_move_bishop(mut b: Board, m: Move, p: i8)->bool{
  return true;
}
fn is_legal_move_knight(mut b: Board, m: Move, p: i8)->bool{
  return true;
}
fn is_legal_move_pawn(mut b: Board, m: Move, p: i8)->bool{
  return true;
}


pub fn is_legal_move(mut b: Board, m: Move)->bool{
  let piece = b.sq[m.from];
  if (piece % 2 != b.turn) || (piece != (Piece::Empty as i8)){
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