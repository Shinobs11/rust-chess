


// use std::{str::*, slice::Chunks};

// use super::bit::setBit;


// pub type BitMask = u64;

// pub struct FullBitBoard {
//   squares: [u8; 64],
//   w_mask: u64,
//   b_mask: u64,
//   states: u32
// }
// impl Default for FullBitBoard{
//   fn default() -> Self {
//       FullBitBoard { squares: [0; 64], w_mask: 0, b_mask: 0, states: 0 }
//   }
// }





// pub struct PositionCache {
//   pub king: [BitMask; 64],
//   pub queen: [BitMask; 64],
//   pub rook: [BitMask; 64],
//   pub bishop: [BitMask; 64],
//   pub knight: [BitMask; 64]
// }
// impl Default for PositionCache{
//   fn default() -> Self {

//       let mut king: [BitMask; 64] = [0; 64];
//       let mut queen: [BitMask; 64] = [0; 64];
//       let mut rook: [BitMask; 64] = [0; 64];
//       let mut bishop: [BitMask; 64] = [0; 64];
//       let mut knight: [BitMask; 64] = [0; 64];

//       let king_offsets = (
//         (1, 1), (1, -1), (-1, 1), (-1, 1), (1, 0), (0, 1), (-1, 0), (0, -1)
//       );

//       let rook_offsets = (..(-7..=7).map(|x|(0, x)), ..(-7..=7).map(|x|(x, 0)));
//       let bishop_offsets = (..(-7..=7).map(|x|(x, x)), ..(-7..=7).map(|x|(x, -x)));
//       let queen_offsets = (..rook_offsets, ..bishop_offsets);

//       for (i, mask) in king.iter_mut().enumerate(){
//         for offset in ()
//         setBit(mask, i as u8);

//       }

      



//       return PositionCache { king: [0; 64], queen: [0; 64], rook: [0; 64], bishop: [0; 64], knight: [0; 64] }
//   }
// }



// // pub fn FENToBitBoard(fen: &String){
// //   let mut bb = FullBitBoard{..Default::default()};
// //   let s: Vec<&str> = fen.split(' ').collect();
// //   let pos = s[0].split('/').rev().enumerate();
// //   let turn = s[1];
// //   let castling = s[2];
// //   let en_pessant = s[3];
// //   let hm_clock = s[4];
// //   let fm_clock = s[5];


// // } 