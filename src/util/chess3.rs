// //(king, queen, knight, bishop, rook, pawn)x2 + empty:13, leaving 3 other states
// //for the sake of simd during move calculations we could do opposing side pieces + one state representing any of your sides pieces + one state for empty square
// //this would be 8 states and perfectly fit in avx-512 registers





// use std::{str::*, slice::Chunks};

// pub type BitBoard = u64;

// pub struct Move {
//   pos_i: u8,
//   pos_f: u8
// }


// #[derive(Default)]
// pub struct FullBitBoard {
//     pub w_king: BitBoard,
//     pub w_queen: BitBoard,
//     pub w_rook: BitBoard,
//     pub w_bishop: BitBoard,
//     pub w_knight: BitBoard,
//     pub w_pawn: BitBoard,
//     pub b_king: BitBoard,
//     pub b_queen: BitBoard,
//     pub b_rook: BitBoard,
//     pub b_bishop: BitBoard,
//     pub b_knight: BitBoard,
//     pub b_pawn: BitBoard,
//     pub empty: BitBoard,
//     pub state: BitBoard
// }
// // state is represented in 8 bit slices separated by -
// // castling-en_pessant-hm_clock-fm_clock-reserved-reserved-reserved-reserved



// pub struct PositionCache {
//   pub king: [BitBoard; 64],
//   pub queen: [BitBoard; 64],
//   pub rook: [BitBoard; 64],
//   pub bishop: [BitBoard; 64],
//   pub knight: [BitBoard; 64]
// }
// impl Default for PositionCache{
//   fn default() -> Self {
//       return PositionCache { king: [0; 64], queen: [0; 64], rook: [0; 64], bishop: [0; 64], knight: [0; 64] }
//   }
// }
// //because chess has such a limited number of positions (64), I can generate an array for each piece as a map for possible places to move.
// //it'd be 64x64x7 bits or about 28kb
// //maybe we do all of theme except pawn.

// use core::fmt;
// use crate::util::bit::*;

// const KNIGHT_MOVE_RULE:[BitBoard; 1] = [0];

// pub fn GeneratePositionCache(){




// }
// pub fn is_move_legal(){

// }


// pub fn createBitBoardFromArray(arr: &[[u8; 8]; 8]) -> BitBoard{
//   let mut bb:BitBoard = 0;
//   for i in (0..8) {
//     for j in (0..8){
//       setBit(&mut bb, 8*i + j);
//     }
//   }
//   return bb;
// }

// pub fn FENToBitBoard(fen: String) -> FullBitBoard {
//     let mut bb = FullBitBoard{..Default::default()};
//     let s: Vec<&str> = fen.split(' ').collect();
//     let pos = s[0].split('/').rev().enumerate();
//     let turn = s[1];
//     let castling = s[2];
//     let en_pessant = s[3];
//     let hm_clock = s[4];
//     let fm_clock = s[5];


//     fn setEmpty(target: &mut BitBoard, c: char, start: u8){
//       let n_sq:u8 = (c as u8) - 48;
//       for i in 0..n_sq {
//         setBit(target, (n_sq + start) as u8);
//       }

//     }
    

//     for (i, pos_slice) in pos {
//       for (j, c) in pos_slice.chars().enumerate() {
//         match c {
//           'k' => setBit(&mut bb.b_king, (8*i + j) as u8),
//           'q' => setBit(&mut bb.b_queen, (8*i + j) as u8),
//           'r' => setBit(&mut bb.b_rook, (8*i + j) as u8),
//           'b' => setBit(&mut bb.b_bishop, (8*i + j) as u8),
//           'n' => setBit(&mut bb.b_knight, (8*i + j) as u8),
//           'p' => setBit(&mut bb.b_pawn, (8*i + j) as u8),
//           'K' => setBit(&mut bb.w_king, (8*i + j) as u8),
//           'Q' => setBit(&mut bb.w_queen, (8*i + j) as u8),
//           'R' => setBit(&mut bb.w_rook, (8*i + j) as u8),
//           'B' => setBit(&mut bb.w_bishop, (8*i + j) as u8),
//           'N' => setBit(&mut bb.w_knight, (8*i + j) as u8),
//           'P' => setBit(&mut bb.w_pawn, (8*i + j) as u8),
//           '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8' => setEmpty(&mut bb.empty, c, (8*i + j) as u8),
//           _ => {}
//         }
//       }
//     } 

    

    
    
//     return bb;
// }

// //top-left corner is index 63, bottom right is index 0;
// pub fn printBitBoard(bb:BitBoard, reverse_orientation: bool)->String{
//   let s:Vec<char> = format!("{:064b}", bb).chars().collect();
//   let mut res: String = String::new();
  
//   if reverse_orientation{
//     for i in (0..8).rev(){
//       for j in (0..8).rev(){
//         res.push(s[8*i + j]);
//         res.push(' ');
//       }
//       res.push('\n');
//       }
//       ;
//   }
//   else {
//     for i in 0..8{
//       for j in 0..8{
//         res.push(s[8*i + j]);
//         res.push(' ');
//       }
//       res.push('\n');
//       }
//       ;
//   }

//   return res;
// }

