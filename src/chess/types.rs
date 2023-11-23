use crate::chess::chess::san_square_to_index;
use crate::chess::consts::*;
use num_enum::FromPrimitive;
use std::collections::HashSet;
use std::fmt::Display;
use std::hash::Hash;
use std::ascii::*;
use std::ops::{Index, IndexMut};
use bitvec::{prelude::*, view::BitView};

#[derive(Debug, Clone)]
pub struct PieceSet {
    pub w_king: HashSet<u8>,
    pub b_king: HashSet<u8>,
    pub w_queen: HashSet<u8>,
    pub b_queen: HashSet<u8>,
    pub w_rook: HashSet<u8>,
    pub b_rook: HashSet<u8>,
    pub w_bishop: HashSet<u8>,
    pub b_bishop: HashSet<u8>,
    pub w_knight: HashSet<u8>,
    pub b_knight: HashSet<u8>,
    pub w_pawn: HashSet<u8>,
    pub b_pawn: HashSet<u8>,
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
            b_pawn: HashSet::new(),
        };
    }
}
impl Default for PieceSet {
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
        };
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
            Piece::WKnight => &self.w_knight,
            Piece::WPawn => &self.w_pawn,
            Piece::BKing => &self.b_king,
            Piece::BQueen => &self.b_queen,
            Piece::BRook => &self.b_rook,
            Piece::BBishop => &self.b_bishop,
            Piece::BKnight => &self.b_knight,
            Piece::BPawn => &self.b_pawn,
            Piece::Empty => unimplemented!(),
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
            Piece::WKnight => &mut self.w_knight,
            Piece::WPawn => &mut self.w_pawn,
            Piece::BKing => &mut self.b_king,
            Piece::BQueen => &mut self.b_queen,
            Piece::BRook => &mut self.b_rook,
            Piece::BBishop => &mut self.b_bishop,
            Piece::BKnight => &mut self.b_knight,
            Piece::BPawn => &mut self.b_pawn,
            Piece::Empty => unimplemented!(),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct CastlingRights {
    pub w_long: bool,
    pub w_short: bool,
    pub b_long: bool,
    pub b_short: bool,
}
impl Default for CastlingRights {
    fn default() -> Self {
        CastlingRights {
            w_long: true,
            w_short: true,
            b_long: true,
            b_short: true,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BitBoardSet {
    pub w_king: u64,
    pub w_queen: u64,
    pub w_rook: u64,
    pub w_bishop: u64,
    pub w_knight: u64,
    pub w_pawn: u64,
    pub b_king: u64,
    pub b_queen: u64,
    pub b_rook: u64,
    pub b_bishop: u64,
    pub b_knight: u64,
    pub b_pawn: u64,
}
impl BitBoardSet {
    pub fn empty_default() -> Self {
        BitBoardSet {
            w_king: 0,
            w_queen: 0,
            w_rook: 0,
            w_bishop: 0,
            w_knight: 0,
            w_pawn: 0,
            b_king: 0,
            b_queen: 0,
            b_rook: 0,
            b_bishop: 0,
            b_knight: 0,
            b_pawn: 0,
        }
    }
}
impl Default for BitBoardSet {
    fn default() -> Self {
        BitBoardSet {
            w_king: W_KING_DEFAULT_MASK,
            w_queen: W_QUEEN_DEFAULT_MASK,
            w_rook: W_ROOK_DEFAULT_MASK,
            w_bishop: W_BISHOP_DEFAULT_MASK,
            w_knight: W_KNIGHT_DEFAULT_MASK,
            w_pawn: W_PAWN_DEFAULT_MASK,
            b_king: B_KING_DEFAULT_MASK,
            b_queen: B_QUEEN_DEFAULT_MASK,
            b_rook: B_ROOK_DEFAULT_MASK,
            b_bishop: B_BISHOP_DEFAULT_MASK,
            b_knight: B_KNIGHT_DEFAULT_MASK,
            b_pawn: B_PAWN_DEFAULT_MASK,
        }
    }
}
impl Index<Piece> for BitBoardSet {
  type Output = u64;
  fn index(&self, index: Piece) -> &Self::Output {
      match index {
          Piece::WKing => &self.w_king,
          Piece::WQueen => &self.w_queen,
          Piece::WRook => &self.w_rook,
          Piece::WBishop => &self.w_bishop,
          Piece::WKnight => &self.w_knight,
          Piece::WPawn => &self.w_pawn,
          Piece::BKing => &self.b_king,
          Piece::BQueen => &self.b_queen,
          Piece::BRook => &self.b_rook,
          Piece::BBishop => &self.b_bishop,
          Piece::BKnight => &self.b_knight,
          Piece::BPawn => &self.b_pawn,
          Piece::Empty => unimplemented!(),
      }
  }
}
impl IndexMut<Piece> for BitBoardSet {
  fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
      match index {
          Piece::WKing => &mut self.w_king,
          Piece::WQueen => &mut self.w_queen,
          Piece::WRook => &mut self.w_rook,
          Piece::WBishop => &mut self.w_bishop,
          Piece::WKnight => &mut self.w_knight,
          Piece::WPawn => &mut self.w_pawn,
          Piece::BKing => &mut self.b_king,
          Piece::BQueen => &mut self.b_queen,
          Piece::BRook => &mut self.b_rook,
          Piece::BBishop => &mut self.b_bishop,
          Piece::BKnight => &mut self.b_knight,
          Piece::BPawn => &mut self.b_pawn,
          Piece::Empty => unimplemented!(),
      }
  }
}
impl Index<u8> for BitBoardSet {
  type Output = u64;
  fn index(&self, index: u8) -> &Self::Output {
      match index {
          0 => &self.w_king,
          2 => &self.w_queen,
          4 => &self.w_rook,
          6 => &self.w_bishop,
          8 => &self.w_knight,
          10 => &self.w_pawn,
          1 => &self.b_king,
          3 => &self.b_queen,
          5 => &self.b_rook,
          7 => &self.b_bishop,
          9 => &self.b_knight,
          11 => &self.b_pawn,
          12_u8..=u8::MAX => unimplemented!(),
      }
  }
}
impl IndexMut<u8> for BitBoardSet {
  fn index_mut(&mut self, index: u8) -> &mut Self::Output {
      match index {
          0 => &mut self.w_king,
          2 => &mut self.w_queen,
          4 => &mut self.w_rook,
          6 => &mut self.w_bishop,
          8 => &mut self.w_knight,
          10 => &mut self.w_pawn,
          1 => &mut self.b_king,
          3 => &mut self.b_queen,
          5 => &mut self.b_rook,
          7 => &mut self.b_bishop,
          9 => &mut self.b_knight,
          11 => &mut self.b_pawn,
          12_u8..=u8::MAX => unimplemented!(),
      }
  }
}





#[derive(Clone, Debug)]
pub struct ColorMasks {
  pub white: u64,
  pub black: u64
}
impl Index<Color> for ColorMasks {
  type Output = u64;
  fn index(&self, index: Color) -> &Self::Output {
      match index {
        Color::White => &self.white,
        Color::Black => &self.black
      }
  }
}
impl IndexMut<Color> for ColorMasks {
  fn index_mut(&mut self, index: Color) -> &mut Self::Output {
      match index {
          Color::White => &mut self.white,
          Color::Black => &mut self.black
      }
  }
}
impl Index<bool> for ColorMasks {
  type Output = u64;
  fn index(&self, index: bool) -> &Self::Output {
      match index {
        false => &self.white,
        true => &self.black
      }
  }
}
impl IndexMut<bool> for ColorMasks {
  fn index_mut(&mut self, index: bool) -> &mut Self::Output {
      match index {
        false => &mut self.white,
        true => &mut self.black
      }
  }
}
impl Default for ColorMasks {
    fn default() -> Self {
        ColorMasks { white: 0, black: 0 }
    }
}
impl ColorMasks {
  fn empty_default() -> Self {
    ColorMasks { white: 0, black: 0 }
  }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub bbs: BitBoardSet,
    pub color_masks: ColorMasks,
    pub piece_set: PieceSet,
    pub castle_state: CastlingRights,
    pub en_pessant_sq: u8,
    pub turn: u8, // 0: white, 1: black
    pub draw_count: u8,
}
impl Board {
    pub fn empty_default() -> Self {
        Board {
            bbs: BitBoardSet::empty_default(),
            color_masks: ColorMasks::empty_default(),
            piece_set: PieceSet::empty_default(),
            castle_state: CastlingRights::default(),
            en_pessant_sq: 255,
            turn: 0,
            draw_count: 0,
        }
    }
    pub fn get_piece_mask(&self, side: u8) -> u64 {
        let mut mask: u64 = 0;
        match side {
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
        Board {
            bbs: BitBoardSet::default(),
            color_masks: ColorMasks::default(),
            piece_set: PieceSet::default(),
            castle_state: CastlingRights::default(),
            en_pessant_sq: 255,
            turn: 0,
            draw_count: 0,
        }
    }
}

impl Display for Board {
  //TODOS: THIS IS GODAWFUL, REFACTOR WHEN BRAIN EXISTS
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut sq_str: [u8; 64] = ['#' as u8; 64];
        
        let w_king_bits_iter = self.bbs.w_king.view_bits::<Msb0>().iter().enumerate();
        let w_queen_bits_iter = self.bbs.w_queen.view_bits::<Msb0>().iter().enumerate();
        let w_rook_bits_iter = self.bbs.w_rook.view_bits::<Msb0>().iter().enumerate();
        let w_bishop_bits_iter = self.bbs.w_bishop.view_bits::<Msb0>().iter().enumerate();
        let w_knight_bits_iter = self.bbs.w_knight.view_bits::<Msb0>().iter().enumerate();
        let w_pawn_bits_iter = self.bbs.w_pawn.view_bits::<Msb0>().iter().enumerate();

        let b_king_bits_iter = self.bbs.b_king.view_bits::<Msb0>().iter().enumerate();
        let b_queen_bits_iter = self.bbs.b_queen.view_bits::<Msb0>().iter().enumerate();
        let b_rook_bits_iter = self.bbs.b_rook.view_bits::<Msb0>().iter().enumerate();
        let b_bishop_bits_iter = self.bbs.b_bishop.view_bits::<Msb0>().iter().enumerate();
        let b_knight_bits_iter = self.bbs.b_knight.view_bits::<Msb0>().iter().enumerate();
        let b_pawn_bits_iter = self.bbs.b_pawn.view_bits::<Msb0>().iter().enumerate();

        let mut iters = [w_king_bits_iter, w_queen_bits_iter, w_rook_bits_iter, w_bishop_bits_iter, w_knight_bits_iter, w_pawn_bits_iter,
                                                        b_king_bits_iter, b_queen_bits_iter, b_rook_bits_iter, b_bishop_bits_iter, b_knight_bits_iter, b_pawn_bits_iter
        ].to_vec();
        iters.reverse();

        for i in (0..iters.len()) {
          let it = iters.pop().unwrap();
          let symbol = PIECE_CHAR_MAP[i];
          for (j, bit) in it {
            if *bit == true {
              sq_str[j] = symbol as u8;
            }
          }
        }

        
        let mut bitboard_str = String::new();
        for i in (0..8) {
          for j in (0..8) {
            bitboard_str.push(sq_str[i*8 + j] as char);
            bitboard_str.push(' ');

          }
          bitboard_str.push('\n');
        }


        return write!(
            f,
            "{}\n{}\n{}\n{}\n{}\n",
            bitboard_str, "null", "null", "null", "null"
        );
    }
}
impl Board {
    pub fn board_from_fen(fen: String) -> Board {
        let mut _fen = fen.trim().clone();
        if _fen.is_empty() {
            panic!("Invalid FEN: Empty string received.");
        }
        let mut board = Board::empty_default();
        let mut fen_vec: Vec<&str> = fen.split(' ').collect();

        if fen_vec.len() != 6 {
            panic!(
                "Invalid FEN: FEN should have 6 sections, this string has {}",
                fen_vec.len()
            );
        }
        let pos_strs = fen_vec[0].split('/');

        fn set_pos(b: &mut Board, piece: Piece, idx: &mut u8) {
            b.bbs[piece] |= (1 << (63 - *idx));
            // b.piece_set[Piece::from_primitive(piece)].insert(*idx);
            *idx += 1;
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
                     _ => {
                      idx += c as u8 - '0' as u8
                    }
                }
            }
        }

        for piece in WHITE_PIECES {
          board.color_masks[Color::White] |= board.bbs[piece];
        }

        for piece in BLACK_PIECES {
          board.color_masks[Color::Black] |= board.bbs[piece];
        }
        
        

        let turn_char = fen_vec[1].chars().nth(0).unwrap();
        if turn_char == 'w' {
            board.turn = 0;
        } else if turn_char == 'b' {
            board.turn = 1;
        } else {
            panic!("Invalid FEN: Invalid turn character provided {}", turn_char);
        }

        let castling_str = fen_vec[2];
        board.castle_state = CastlingRights {
            w_long: castling_str.contains('Q'),
            w_short: castling_str.contains('K'),
            b_long: castling_str.contains('q'),
            b_short: castling_str.contains('k'),
        };

        let en_pessant_str = fen_vec[3];
        if en_pessant_str.contains('-') {
            board.en_pessant_sq = 255;
        } else {
            board.en_pessant_sq = san_square_to_index(fen_vec[3].as_ascii().unwrap());
        }

        return board;
    }
}


//TODOs: this might be more efficient to use a single u16 than a struct with 3 u8s

pub struct Move {
    //0 = normal move, 1 = castle, 2 = en pessant
    kind: u8,
    //if kind == 0, index of selected piece
    //if kind == 1, short castle = 0, long castle = 1
    //if kind == 2, index of selected pawn
    arg1: u8,
    //if kind == 0, index of target square
    //if kind == 1, empty
    //if kind == 2 empty
    arg2: u8,
}
impl Move {
  #[inline(always)]
  pub fn new(kind: u8, arg1: u8, arg2: u8)->Self {Self {kind, arg1, arg2}}
  #[inline(always)]
  pub fn kind(&mut self) -> u8 {self.kind}
  #[inline(always)]
  pub fn arg1(&mut self) -> u8 {self.arg1}
  #[inline(always)]
  pub fn arg2(&mut self) -> u8 {self.arg2}
}
impl Default for Move {
    fn default() -> Self {
        Move {
            kind: 0,
            arg1: 0,
            arg2: 0,
        }
    }
}
