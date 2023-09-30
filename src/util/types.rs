use num_enum::FromPrimitive;
use std::collections::HashSet;
use std::hash::Hash;
use crate::util::consts::*;
use std::fmt::Display;
use std::ops::{Index, IndexMut};
#[derive(Debug, Clone)]
pub struct PieceSet {
  pub w_king:HashSet<u8>,
  pub b_king:HashSet<u8>,
  pub w_queen:HashSet<u8>,
  pub b_queen:HashSet<u8>,
  pub w_rook:HashSet<u8>,
  pub b_rook:HashSet<u8>,
  pub w_bishop:HashSet<u8>,
  pub b_bishop:HashSet<u8>,
  pub w_knight:HashSet<u8>,
  pub b_knight:HashSet<u8>,
  pub w_pawn:HashSet<u8>,
  pub b_pawn:HashSet<u8>,
}
impl PieceSet {
  pub fn empty_default() -> Self {
    return PieceSet {
      w_king: HashSet::new(), 
      b_king: HashSet::new(), 
      w_queen: HashSet::new(), 
      b_queen: HashSet::new(), 
      w_rook: HashSet::new(), 
      b_rook: HashSet::new(), 
      w_bishop: HashSet::new(), 
      b_bishop: HashSet::new(), 
      w_knight: HashSet::new(), 
      b_knight: HashSet::new(), 
      w_pawn: HashSet::new(), 
      b_pawn: HashSet::new() 
    }
  }
}
impl Default for PieceSet{
  fn default() -> Self {
      return PieceSet {
        w_king: HashSet::from([60]),
        b_king: HashSet::from([4]),
        w_queen: HashSet::from([59]),
        b_queen: HashSet::from([3]),
        w_rook: HashSet::from([56, 63]),
        b_rook: HashSet::from([0, 7]),
        w_bishop: HashSet::from([58, 61]),
        b_bishop: HashSet::from([2, 5]),
        w_knight: HashSet::from([57, 62]),
        b_knight: HashSet::from([1, 6]),
        w_pawn: HashSet::from([48, 49, 50, 51, 52, 53, 54, 55]),
        b_pawn: HashSet::from([8, 9, 10, 11, 12, 13, 14, 15]),
      }
  }
}
impl Index<Piece> for PieceSet {
  type Output = HashSet<u8>;
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

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CastlingRights {
  pub w_long: bool,
  pub w_short: bool,
  pub b_long: bool,
  pub b_short: bool
}
impl Default for CastlingRights {
  fn default() -> Self {
      CastlingRights { w_long: true, w_short: true, b_long: true, b_short: true }
  }
}

#[derive(Clone, Debug)]
pub struct Board {
  pub sq: [u8; 64],
  pub piece_set: PieceSet,
  pub castle_state: CastlingRights,
  pub en_pessant_sq: u8,
  pub turn: u8, // 0: white, 1: black
  pub draw_count: u8 
}
impl Board {
  pub fn empty_default() -> Self {
    Board { 
      sq:[0;64], 
      piece_set: PieceSet::empty_default(), 
      castle_state: CastlingRights::default(), 
      en_pessant_sq: 255, 
      turn: 0, 
      draw_count: 0 
    }
  }
  pub fn get_piece_mask(&self, side:u8)->u64{
    let mut mask:u64 = 0;
    match side{
      0 => {
        for p in WHITE_PIECES {
          for p_idx in self.piece_set[p].iter() {
            mask |= (1u64 << *p_idx);
          }
        }
      }
      1 => {
        for p in BLACK_PIECES {
          for p_idx in self.piece_set[p].iter() {
            mask |= (1u64 << *p_idx);
          }
        }
      }
      _ => {}
    }
    return mask;
  }
  
  
}
impl Default for Board {
  fn default() -> Self {
    Board {sq:DEFAULT_BOARD.map(|x|x as u8), piece_set:PieceSet::default(), castle_state: CastlingRights::default(), en_pessant_sq: 255, turn: 0, draw_count: 0}
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
impl Board {
  pub fn board_from_fen(fen: String)->Board{
    let mut _fen = fen.trim().clone();
    if _fen.is_empty() {
      panic!("Invalid FEN: Empty string received.");
    }
    let mut board = Board::empty_default();
    let mut fen_vec: Vec<&str> = fen.split(' ').collect();

    if fen_vec.len() != 6 {
      panic!("Invalid FEN: FEN should have 6 sections, this string has {}", fen_vec.len());
    }
    let pos_strs = fen_vec[0].split('/');
  
    fn set_pos(b: &mut Board, piece: u8, idx: &mut u8){
      b.sq[*idx as usize] = piece;
      b.piece_set[Piece::from_primitive(piece)].insert(*idx);
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

    let turn_char = fen_vec[1].chars().nth(0).unwrap();
    if turn_char == 'w' {
      board.turn = 0;
    }
    else if turn_char == 'b' {
      board.turn = 1;
    }
    else{
      panic!("Invalid FEN: Invalid turn character provided {}", turn_char);
    }
    
    let castling_str = fen_vec[2];
    board.castle_state = CastlingRights{
      w_long: castling_str.contains('Q'),
      w_short: castling_str.contains('K'),
      b_long: castling_str.contains('q'),
      b_short: castling_str.contains('k')
    };


    let en_pessant_str = fen_vec[3];
    if en_pessant_str.contains('-'){
      board.en_pessant_sq = 255;
    }
    else {
      board.en_pessant_sq = match u8::from_str_radix(fen_vec[3], 10) {
        Ok(x) => x,
        Err(err)=> panic!("Invalid FEN: Invalid en pessant square provided {}", err)
      };
    }





    return board;
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