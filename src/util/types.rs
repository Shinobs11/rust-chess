use crate::util::consts::*;
use std::ops::{Index, IndexMut};
pub struct PieceSet {
  pub w_king:Vec<u8>,
  pub b_king:Vec<u8>,
  pub w_queen:Vec<u8>,
  pub b_queen:Vec<u8>,
  pub w_rook:Vec<u8>,
  pub b_rook:Vec<u8>,
  pub w_bishop:Vec<u8>,
  pub b_bishop:Vec<u8>,
  pub w_knight:Vec<u8>,
  pub b_knight:Vec<u8>,
  pub w_pawn:Vec<u8>,
  pub b_pawn:Vec<u8>,
}
impl PieceSet {
  pub fn empty_default() -> Self {
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
  //0 = normal move, 1 = castle, 2 = en pessant
  pub kind: u8, 
  //if kind == 0, index of selected piece
  //if kind == 1, short castle = 0, long castle = 1
  //if kind == 2, index of selected pawn
  pub arg1: u8, 
  //if kind == 0, index of target square
  //if kind == 1, empty
  //if kind == 2 empty
  pub arg2: u8

}
impl Default for Move {
  fn default() -> Self {
      Move { kind: 0, arg1: 0, arg2: 0 }
  }
}