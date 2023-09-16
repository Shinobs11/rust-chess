
use std::{str::*, slice::Chunks};



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


const default_board:[u8; 64] = [
  Piece::BRook, Piece::BKnight, Piece::BBishop, Piece::BQueen, Piece::BKing, Piece::BBishop, Piece::BKnight, Piece::BRook,
  Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn,
  Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty,
  Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty,
  Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty,
  Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty,
  Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn,
  Piece::WRook, Piece::WKnight, Piece::WBishop, Piece::WQueen, Piece::WKing, Piece::WBishop, Piece::WKnight, Piece::WRook
].map(|p: Piece| p as u8);




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






pub struct Move {
  from: usize,
  to: usize
}

pub struct Board {
  sq: [u8; 64],
  pieces: [u8, 32],
  castle_state: [bool; 4],
  en_pessant_sq: u8,
  turn: u8, // 0: white, 1: black
  draw_count: u8 
}
impl Default for Board {
  fn default() -> Self {
    Board {sq:[0; 64], pieces:[0, 32], castle_state: [true; 4], en_pessant_sq: 64, turn: 0, draw_count: 0}
  }
}

const KING_OFFSETS: (i32, i32, i32, i32, i32, i32, i32, i32) = 
                    (-9, -8, -7, 
                     -1,      1,
                      7,  8,  9);





fn check_for_check(mut b: Board, m: Move, p: u8)->bool {


}

fn non_king_check_for_check(mut b: Board, m: Move, p: u8)->bool{
  b.sq[m.from] = Piece::Empty as u8;



  b.sq[m.from] = p;
  return true;
}
fn king_check_for_check(mut b: Board, m: Move, p: u8)->bool {

  return true;
}

fn is_legal_move_king(mut b: Board, m: Move, p: u8)->bool{
  
  return true;
}
fn is_legal_move_queen(mut b: Board, m: Move, p: u8)->bool{
  return true;
}
fn is_legal_move_rook(mut b: Board, m: Move, p: u8)->bool{
  return true
}
fn is_legal_move_bishop(mut b: Board, m: Move, p: u8)->bool{
  return true;
}
fn is_legal_move_knight(mut b: Board, m: Move, p: u8)->bool{
  return true;
}
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