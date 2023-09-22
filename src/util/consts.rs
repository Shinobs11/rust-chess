use num_enum::{IntoPrimitive, FromPrimitive};
#[derive(IntoPrimitive, FromPrimitive)]
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

#[derive(IntoPrimitive, FromPrimitive)]
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


pub const PIECE_CHAR_MAP:[char; 13] = ['K', 'k', 'Q', 'q', 'R', 'r', 'B', 'b', 'N', 'n', 'P', 'p', '#'];


pub const WHITE_PIECES:[Piece; 6] = [Piece::WKing, Piece::WQueen, Piece::WRook, Piece::WBishop, Piece::WKnight, Piece::WPawn];
pub const BLACK_PIECES:[Piece; 6] = [Piece::BKing, Piece::BQueen, Piece::BRook, Piece::BBishop, Piece::BKnight, Piece::BPawn];


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


