use num_enum::{IntoPrimitive, FromPrimitive};
use std::ops::*;
#[derive(IntoPrimitive, FromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Piece {
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
  #[num_enum(default)]
  Empty = 12
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum GenericPiece {
  King = 0,
  Queen = 1,
  Rook = 2,
  Bishop = 3,
  Knight = 4,
  Pawn = 5,
  #[num_enum(default)]
  Empty = 6
}
#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Color {
  #[default]
  White = 0,
  Black = 1
}
impl Not for Color {
  type Output = Color;
  fn not(self) -> Self::Output {
    match self {
      Color::White => Color::Black,
      Color::Black => Color::White
    }
  }
}

pub const PIECE_CHAR_MAP: [char; 13] = ['K', 'Q', 'R', 'B', 'N', 'P', 'k', 'q', 'r', 'b', 'n', 'p', '#'];
pub const WHITE_PIECES:[Piece; 6] = [Piece::WKing, Piece::WQueen, Piece::WRook, Piece::WBishop, Piece::WKnight, Piece::WPawn];
pub const BLACK_PIECES:[Piece; 6] = [Piece::BKing, Piece::BQueen, Piece::BRook, Piece::BBishop, Piece::BKnight, Piece::BPawn];
pub const PIECES_WITH_EMPTY: [Piece; 13] = [Piece::WKing, Piece::WQueen, Piece::WRook, Piece::WBishop, Piece::WKnight, Piece::WPawn,
                                            Piece::BKing, Piece::BQueen, Piece::BRook, Piece::BBishop, Piece::BKnight, Piece::BPawn,
                                            Piece::Empty];
pub const PIECES: [Piece; 12] = [ Piece::WKing, Piece::WQueen, Piece::WRook, Piece::WBishop, Piece::WKnight, Piece::WPawn,
                                  Piece::BKing, Piece::BQueen, Piece::BRook, Piece::BBishop, Piece::BKnight, Piece::BPawn];

pub const DEFAULT_BOARD:[Piece; 64] = [
  Piece::BRook, Piece::BKnight, Piece::BBishop, Piece::BQueen, Piece::BKing, Piece::BBishop, Piece::BKnight, Piece::BRook,
  Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn, Piece::BPawn,
  Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty,
  Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty,
  Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty,
  Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty, Piece::Empty,
  Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn, Piece::WPawn,
  Piece::WRook, Piece::WKnight, Piece::WBishop, Piece::WQueen, Piece::WKing, Piece::WBishop, Piece::WKnight, Piece::WRook
];




pub const default_pieces:[u8;32] = [
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

pub const W_KING_DEFAULT_MASK: u64 = (1 << (63 - 60));
pub const W_QUEEN_DEFAULT_MASK: u64 = (1 << (63 - 59));
pub const W_ROOK_DEFAULT_MASK: u64 = (1 << (63 - 63)) | (1 << (63 - 56));
pub const W_BISHOP_DEFAULT_MASK: u64 = (1 << (63 - 61)) | (1 << (63 - 58));
pub const W_KNIGHT_DEFAULT_MASK: u64 = (1 << (63 - 62)) | (1 << (63 - 57));
pub const W_PAWN_DEFAULT_MASK: u64 = (1 << (63 - 47)) | (1 << (63 - 48)) | (1 << (63 - 49)) | (1 << (63 - 50)) | (1 << (63 - 51)) | (1 << (63 - 52)) | (1 << (63 - 53)) | (1 << (63 - 54));

pub const B_KING_DEFAULT_MASK: u64 = (1 << (63 - 4));
pub const B_QUEEN_DEFAULT_MASK: u64 = (1 << (63 - 3));
pub const B_ROOK_DEFAULT_MASK: u64 = (1 << (63 - 0)) | (1 << (63 - 7));
pub const B_BISHOP_DEFAULT_MASK: u64 = (1 << (63 - 2)) | (1 << (63 - 5));
pub const B_KNIGHT_DEFAULT_MASK: u64 = (1 << (63 - 1)) | (1 << (63 - 6));
pub const B_PAWN_DEFAULT_MASK: u64 = (1 << (63 - 8)) | (1 << (63 - 9)) | (1 << (63 - 10)) | (1 << (63 - 11)) | (1 << (63 - 12)) | (1 << (63 - 13)) | (1 << (63 - 14)) | (1 << (63 - 15));

