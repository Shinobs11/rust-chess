//(king, queen, knight, bishop, rook, pawn)x2 + empty:13, leaving 3 other states
//for the sake of simd during move calculations we could do opposing side pieces + one state representing any of your sides pieces + one state for empty square
//this would be 8 states and perfectly fit in avx-512 registers
#[derive(Default)]
pub struct FullBitBoard {
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
    pub empty: u64,
    pub state: u64
}
// state is represented in 8 bit slices separated by -
// castling-en_pessant-hm_clock-fm_clock-reserved-reserved-reserved-reserved

pub struct AttackBitBoard {
    pub opp_king: u64,
    pub opp_queen: u64,
    pub opp_rook: u64,
    pub opp_bishop: u64,
    pub opp_knight: u64,
    pub opp_pawn: u64,
    pub f_pieces: u64,
    pub empty: u64,
}

//because chess has such a limited number of positions (64), I can generate an array for each piece as a map for possible places to move.
//it'd be 64x64x7 bits or about 28kb

use crate::util::bit::*;
pub fn FENToBitBoard(fen: String) -> FullBitBoard {
    let mut bb = FullBitBoard{..Default::default()};
    let s: Vec<&str> = fen.split(' ').collect();
    let pos = s[0].split('/').enumerate();
    let turn = s[1];
    let castling = s[2];
    let en_pessant = s[3];
    let hm_clock = s[4];
    let fm_clock = s[5];


    fn setEmpty(target: &mut u64, c: char, start: u8){
      let n_sq:u8 = (c as u8) - 48;
      for i in 0..n_sq {
        setBit(target, (n_sq + start) as u8);
      }

    }
    

    for (i, pos_slice) in pos {
      for (j, c) in pos_slice.chars().enumerate() {
        match c {
          'k' => setBit(&mut bb.b_king, (8*i + j) as u8),
          'q' => setBit(&mut bb.b_queen, (8*i + j) as u8),
          'r' => setBit(&mut bb.b_rook, (8*i + j) as u8),
          'b' => setBit(&mut bb.b_bishop, (8*i + j) as u8),
          'n' => setBit(&mut bb.b_knight, (8*i + j) as u8),
          'p' => setBit(&mut bb.b_pawn, (8*i + j) as u8),
          'K' => setBit(&mut bb.w_king, (8*i + j) as u8),
          'Q' => setBit(&mut bb.w_queen, (8*i + j) as u8),
          'R' => setBit(&mut bb.w_rook, (8*i + j) as u8),
          'B' => setBit(&mut bb.w_bishop, (8*i + j) as u8),
          'N' => setBit(&mut bb.w_knight, (8*i + j) as u8),
          'P' => setBit(&mut bb.w_pawn, (8*i + j) as u8),
          '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8' => setEmpty(&mut bb.empty, c, (8*i + j) as u8),
          _ => {}
        }
      }
    } 

    

    
    
    return FullBitBoard {..Default::default()};
}

pub fn printBitBoard(bb:u64){
  
}