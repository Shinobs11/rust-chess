pub type BitBoard = u64;
use Which::{First, Second};
use std::simd::*;
use bitvec::{prelude::*, view::BitView};
use std::{ ops::Shl, ops::Shr};
use std::collections::*;
use crate::chess::consts::*;
use crate::chess::bit::*;
use std::collections::HashMap;
use crate::cache::*;



fn print_board(b:BitBoard){
  let slice = b.view_bits::<Lsb0>();

  for x in (0..8){
    for y in (0..8){
      print!("{:3?}", slice[x*8 + y] as u8);
    }
    print!("\n");
  }
}
pub fn get_bit_idx(n:u64)->usize{
  return n.view_bits::<Msb0>().first_one().unwrap();
}

pub fn test(lhs:u64, sh:u32)->u64{
  let a = lhs << sh;
  return a;
}

pub fn range_max(r: u64)->u32{
  let piece_idx = r.leading_zeros();
  let range_max = piece_idx/8;
  return range_max;
}












// So there's a choice to be made about en pessant here
// A. I can represent the square the pawn will move to
// B. I can represent the piece being targeted.
// I think I'll choose A., but I'll need to keep both possibilities in mind when writing future code
pub fn pawn_attack_mask(piece_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64, color: Color, ep_square: u8)->u64{
  let piece_idx = piece_mask.leading_zeros();
  let mut res:u64 = 0;

  
  if color == Color::White {
    let starting_row: u8 = 7;
    res |= (((piece_idx / 8) as u8 == starting_row) as u64) * (piece_mask << (16)) & !friendly_pos_mask & !opponent_pos_mask;
    res |= (piece_mask << 8) & !friendly_pos_mask & !opponent_pos_mask;
    res |= (piece_mask << 7) & (opponent_pos_mask | (1 << (63 - ep_square)));
    res |= (piece_mask << 9) & (opponent_pos_mask | (1 << (63 - ep_square)));
  }
  else {
    let starting_row: u8 = 1;
    res |= (((piece_idx / 8) as u8 == starting_row) as u64) * (piece_mask >> 16) & !friendly_pos_mask & !opponent_pos_mask;
    res |= (piece_mask >> 8) & !friendly_pos_mask & !opponent_pos_mask;
    res |= (piece_mask >> 7) & (opponent_pos_mask | (1 << (63 - ep_square)));
    res |= (piece_mask >> 9) & (opponent_pos_mask | (1 << (63 - ep_square)));   
  }


  return res;
}

pub fn batch_pawn_attack_mask(v: &Vec<(u64, u64, u64, Color, u8)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask, color, ep_square) in v {
    res.push(pawn_attack_mask(*piece_mask, *friend_mask, *foe_mask, *color, *ep_square));
  }
  return res;
}




/*
  At first glance, knight attack masks are trivial to calculate, but
  become more difficult once factoring in the (literal) edge-cases.
  

*/

pub fn knight_attack_mask(piece_mask: u64, friendly_pos_mask: u64)->u64{
  return KNIGHT_CACHE[piece_mask.leading_zeros() as usize] & !friendly_pos_mask;
}

pub fn batch_knight_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(knight_attack_mask(*piece_mask, *friend_mask));
  }
  return res;
}



pub fn bishop_attack_mask(piece_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64{
  let piece_idx = piece_mask.leading_zeros();
  let pos_diag_mask = DIAG_MASK_CACHE[(2 * piece_idx) as usize];
  let neg_diag_mask = DIAG_MASK_CACHE[((2 * piece_idx) + 1) as usize];

  let f_pos_diag_mask = (((friendly_pos_mask & pos_diag_mask) * GET_DIAG_MASK_MAGIC) >> 56) as u8;
  let f_neg_diag_mask = (((friendly_pos_mask & neg_diag_mask) * GET_DIAG_MASK_MAGIC) >> 56) as u8;
  let o_pos_diag_mask = (((opponent_pos_mask & pos_diag_mask) * GET_DIAG_MASK_MAGIC) >> 56) as u8;
  let o_neg_diag_mask = (((opponent_pos_mask & neg_diag_mask) * GET_DIAG_MASK_MAGIC) >> 56) as u8;

  let pos_ray = RAY_CACHE[get_ternary_bitmask((piece_idx % 8) as u8, f_pos_diag_mask , o_pos_diag_mask) as usize];
  let neg_ray = RAY_CACHE[get_ternary_bitmask((piece_idx % 8) as u8, f_neg_diag_mask, o_neg_diag_mask) as usize];
  
  let expanded_pos_ray = (pos_ray as u64) * GET_DIAG_MASK_MAGIC;
  let expanded_neg_ray = (neg_ray as u64) * GET_DIAG_MASK_MAGIC;

  return (pos_diag_mask & expanded_pos_ray) | (neg_diag_mask & expanded_neg_ray);
}
pub fn batch_bishop_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(bishop_attack_mask(*piece_mask, *friend_mask, *foe_mask));
  }
  return res;
}

pub fn rook_attack_mask(piece_mask:u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64{
  let mut res = 0;
  let piece_idx:u8 = piece_mask.leading_zeros() as u8;
  let piece_row_idx: u8 = (piece_idx / 8) as u8;
  let piece_col_idx: u8 = (piece_idx % 8) as u8;

  let row_mask = (TERNARY_CACHE[get_row_mask(friendly_pos_mask, piece_row_idx) as usize] 
                    + 2*TERNARY_CACHE[get_row_mask(opponent_pos_mask, piece_row_idx) as usize])
                    | ((piece_col_idx as u16) << 13); //watch out for the piece_col_idx               
  let col_mask = (TERNARY_CACHE[get_col_mask(friendly_pos_mask, piece_col_idx) as usize] 
                    + 2*TERNARY_CACHE[get_col_mask(opponent_pos_mask, piece_col_idx) as usize])
                    | ((piece_row_idx as u16) << 13);// and piece_row_idx

  //bit of a gotcha, need to provide piece_col_idx to index row_attack_mask and vice versa
  let row_attack_mask = RAY_CACHE[row_mask as usize];
  let col_attack_mask = RAY_CACHE[col_mask as usize];
  res = (put_row_mask(row_attack_mask, piece_row_idx) | put_col_mask(col_attack_mask, piece_col_idx)) & !piece_mask;
  return res;
}











pub fn batch_rook_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(rook_attack_mask(*piece_mask, *friend_mask, *foe_mask));
  }
  return res;
}

// pub fn batch_simd_branchless_rook(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
//   let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
//   for (piece_mask, friend_mask, foe_mask) in v {
//     res.push(simd_branchless_rook(*piece_mask, *friend_mask, *foe_mask));
//   }
//   return res;
// }








pub fn queen_attack_mask(piece_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64 {
  let mut res:u64;
  let piece_idx: u8 = piece_mask.leading_zeros() as u8;
  let piece_row_idx: u8 = (piece_idx / 8) as u8;
  let piece_col_idx: u8 = (piece_idx % 8) as u8;

  let row_mask: u16 = TERNARY_CACHE[get_row_mask(friendly_pos_mask, piece_row_idx) as usize] 
                    + 2*TERNARY_CACHE[get_row_mask(opponent_pos_mask, piece_row_idx) as usize]
                    | ((piece_row_idx as u16) << 13);
  let col_mask: u16 = TERNARY_CACHE[get_col_mask(friendly_pos_mask, piece_col_idx) as usize] 
                    + 2*TERNARY_CACHE[get_col_mask(opponent_pos_mask, piece_col_idx) as usize]
                    | ((piece_col_idx as u16) << 13);

  //bit of a gotcha, need to provide piece_col_idx to index row_attack_mask and vice versa
  let row_attack_mask: u8 = RAY_CACHE[row_mask as usize];
  let col_attack_mask: u8 = RAY_CACHE[col_mask as usize];

  res = (put_row_mask(row_attack_mask, piece_row_idx) | put_col_mask(col_attack_mask, piece_col_idx)) & !piece_mask;
  
  let pos_diag_mask: u64 = DIAG_MASK_CACHE[(2 * piece_idx) as usize];
  let neg_diag_mask: u64 = DIAG_MASK_CACHE[((2 * piece_idx) + 1) as usize];

  let f_pos_diag_mask: u8 = (((friendly_pos_mask & pos_diag_mask) * GET_DIAG_MASK_MAGIC) >> 56) as u8;
  let f_neg_diag_mask: u8 = (((friendly_pos_mask & neg_diag_mask) * GET_DIAG_MASK_MAGIC) >> 56) as u8;
  let o_pos_diag_mask: u8 = (((opponent_pos_mask & pos_diag_mask) * GET_DIAG_MASK_MAGIC) >> 56) as u8;
  let o_neg_diag_mask: u8 = (((opponent_pos_mask & neg_diag_mask) * GET_DIAG_MASK_MAGIC) >> 56) as u8;

  let pos_ray: u8 = RAY_CACHE[get_ternary_bitmask((piece_idx % 8) as u8, f_pos_diag_mask , o_pos_diag_mask) as usize];
  let neg_ray: u8 = RAY_CACHE[get_ternary_bitmask((piece_idx % 8) as u8, f_neg_diag_mask, o_neg_diag_mask) as usize];
  
  let expanded_pos_ray: u64 = (pos_ray as u64) * GET_DIAG_MASK_MAGIC;
  let expanded_neg_ray: u64 = (neg_ray as u64) * GET_DIAG_MASK_MAGIC;

  res |= (pos_diag_mask & expanded_pos_ray) | (neg_diag_mask & expanded_neg_ray);

  return res;
}



pub fn batch_queen_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(queen_attack_mask(*piece_mask, *friend_mask, *foe_mask));
  }
  return res;
}

// again another choice to be made with the king
// A. I ignore the possibility of check and handle it later
// B. I handle check in the mask
// as I simply don't have enough information, being passed into the function, I'm obv going to choose A.
pub fn king_attack_mask(piece_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64 {
  let mut res: u64 = 0;
  //There is an opportunity for caching to be beneficial here, but I'm not sure if it'd be worth while
  //after all, there's like 32 ops in total here. 
  res |= (piece_mask << 9) & (!friendly_pos_mask | opponent_pos_mask);
  res |= (piece_mask << 8) & (!friendly_pos_mask | opponent_pos_mask);
  res |= (piece_mask << 7) & (!friendly_pos_mask | opponent_pos_mask);
  res |= (piece_mask << 1) & (!friendly_pos_mask | opponent_pos_mask);
  res |= (piece_mask >> 1) & (!friendly_pos_mask | opponent_pos_mask);
  res |= (piece_mask >> 7) & (!friendly_pos_mask | opponent_pos_mask);
  res |= (piece_mask >> 8) & (!friendly_pos_mask | opponent_pos_mask);
  res |= (piece_mask >> 9) & (!friendly_pos_mask | opponent_pos_mask);
  
  return res;
} 
pub fn batch_king_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(king_attack_mask(*piece_mask, *friend_mask, *foe_mask));
  }
  return res;
}

pub fn batch_all_attack_mask(v: &Vec<(u64, u64, u64, GenericPiece)>)-> Vec<u64> {
  let mut res: Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for x in v {
    let piece_mask = x.0;
    let friend_mask = x.1;
    let foe_mask = x.2;
    let piece_type = x.3;

    let r = match piece_type {
      GenericPiece::King => 0,
      GenericPiece::Queen => queen_attack_mask(piece_mask, friend_mask, foe_mask),
      GenericPiece::Rook => rook_attack_mask(piece_mask, friend_mask, foe_mask),
      GenericPiece::Bishop => bishop_attack_mask(piece_mask, friend_mask, foe_mask),
      GenericPiece::Knight => knight_attack_mask(piece_mask, friend_mask),
      GenericPiece::Pawn => 0,
      GenericPiece::Empty => 0
    };
    res.push(r);
  }
  return res;

}


/*
For the branchless bishop, an optimization to consider that could also be applied to the branchless rook
is to calculate range_max with a bitwise AND, potentially saving us a couple of ops in latency.


// */
// pub fn bishop_attack_mask(piece_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64{
//   let piece_idx = piece_mask.leading_zeros();
//   let mut result:u64 = 0;
//   // delta = -7
//   let mut prev:u64 = piece_mask;
//   let mut attack_toggle:u64 = 0;
  
//   let mut range_x = 7 - (piece_idx % 8);
//   let mut range_y = piece_idx / 8;
//   let mut range_max:u32 = range_x.min(range_y);

//   let mut has_been_blocked:u64;
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask << (7 * i)) & !friendly_pos_mask & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }

//   //delta = 7
  
//   prev = piece_mask;
//   attack_toggle = 0;
//   range_x = 7 - (piece_idx % 8);
//   range_y = 7 - (piece_idx / 8);
//   range_max = range_x.min(range_y);
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask >> (7 * i)) & !friendly_pos_mask & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }

//   //delta = -9
//   prev = piece_mask;
//   attack_toggle = 0;
//   range_x = (piece_idx % 8);
//   range_y = (piece_idx / 8);
//   range_max = range_x.min(range_y);
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask << (9 * i)) & !friendly_pos_mask  & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }

//   //delta = 9
//   prev = piece_mask;
//   attack_toggle = 0;
//   range_x = (piece_idx % 8);
//   range_y = 7 - (piece_idx / 8);
//   range_max = range_x.min(range_y);
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask >> (9 * i)) & !friendly_pos_mask  & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }
  
//   return result;
// }


/*
Well shit, i guess it's not branchless now. branch prediction is wild
*/
// pub fn rook_attack_mask(piece_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64{
//   let piece_idx = piece_mask.leading_zeros();
//   let mut result:u64 = 0;

//   // delta = -8
//   let mut prev:u64 = piece_mask;
//   let mut attack_toggle:u64 = 0;
//   let mut range_max:u32 = piece_idx/8;
//   let mut has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask << (i*8)) & !friendly_pos_mask & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }

//   //delta = 8
  
//   prev = piece_mask;
//   attack_toggle = 0;
//   range_max = (63-piece_idx)/8;
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask >> (i*8)) & !friendly_pos_mask & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }

//   //delta = -1
//   prev = piece_mask;
//   attack_toggle = 0;
//   range_max = (piece_idx % 8);
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask << (i as i32)) & !friendly_pos_mask  & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }

//   //delta = 1
//   prev = piece_mask;
//   attack_toggle = 0;
//   range_max = 7 - (piece_idx % 8);
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask >> (i as i32)) & !friendly_pos_mask  & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }
  
//   return result;
// }
// pub fn simd_branchless_rook(piece_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64{

//   let piece_idx:u64 = piece_mask.leading_zeros() as u64;
//   let mut result:u64x4 = u64x4::from_array([0;4]);
//   let simd_friendly_pos_mask = u64x4::from_array([friendly_pos_mask; 4]);
//   let simd_opponent_pos_mask = u64x4::from_array([opponent_pos_mask; 4]);

//   let negative_offsets:u64x2 = u64x2::from_array([8, 1]);
//   let positive_offsets:u64x2 = u64x2::from_array([8, 1]);

//   let mut prev:u64x4 = u64x4::from_array([piece_mask;4]);
//   let mut attack_toggle:u64x4 = u64x4::from_array([0;4]);
//   // let mut range_max:u64x4 = u64x4::from_array([piece_idx/8, (63-piece_idx)/8, piece_idx % 8, 7 - piece_idx % 8]);
//   let lb:u64x4 = u64x4::from([0, 0, piece_idx & !7, piece_idx & !7]);
//   let ub:u64x4 = u64x4::from([63, 63, piece_idx | 7, piece_idx | 7]);
//   // let simd_piece_mask = u64x4::from_array([piece_mask; 4]);
//   for i in (1..8){
//     let simd_i:u64x4 = u64x4::from([i;4]);
//     //for tomorrow: Masks seem to have 0 as false and -1 as true
//     let has_been_blocked:u64x4 = prev.simd_ne(u64x4::from_array([0;4])).to_int().abs().cast() * u64x4::from_array([u64::MAX; 4]);
//     let left_shift = u64x2::from_array([piece_mask;2]) << (simd_swizzle!(simd_i, [0, 2]) * negative_offsets);
    
//     let right_shift = u64x2::from_array([piece_mask;2]) >> (simd_swizzle!(simd_i, [1, 3]) * positive_offsets);
//     let shift:Simd<u64, 4> = simd_swizzle!(left_shift, right_shift, [First(0), Second(0), First(1), Second(1)]);
//     let offsets:i64x4 = simd_i.cast() * i64x4::from_array([-8, 8, -1, 1]);
//     let checked_shift:Simd<u64, 4> = (offsets.simd_ge(lb.cast()) & offsets.simd_le(ub.cast())).to_int().abs().cast() * u64x4::from_array([u64::MAX; 4]) & shift;
    
//     prev = has_been_blocked & checked_shift & !simd_friendly_pos_mask & !attack_toggle;
//     attack_toggle = ((prev & simd_opponent_pos_mask).simd_gt(u64x4::from_array([0; 4])).to_int().abs().cast() * u64x4::from_array([u64::MAX;4]));
//     result |= prev;
//   }
//   return result.reduce_and();
// }

// pub fn queen_attack_mask(piece_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64 {

//   let piece_idx = piece_mask.leading_zeros();
//   let mut result:u64 = 0;
//   // delta = -7
//   let mut prev:u64 = piece_mask;
//   let mut attack_toggle:u64 = 0;
  
//   let mut range_x = 7 - (piece_idx % 8);
//   let mut range_y = piece_idx / 8;
//   let mut range_max:u32 = range_x.min(range_y);

//   let mut has_been_blocked:u64;
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask << (7 * i)) & !friendly_pos_mask & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }

//   //delta = 7
  
//   prev = piece_mask;
//   attack_toggle = 0;
//   range_x = 7 - (piece_idx % 8);
//   range_y = 7 - (piece_idx / 8);
//   range_max = range_x.min(range_y);
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask >> (7 * i)) & !friendly_pos_mask & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }

//   //delta = -9
//   prev = piece_mask;
//   attack_toggle = 0;
//   range_x = (piece_idx % 8);
//   range_y = (piece_idx / 8);
//   range_max = range_x.min(range_y);
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask << (9 * i)) & !friendly_pos_mask  & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }

//   //delta = 9
//   prev = piece_mask;
//   attack_toggle = 0;
//   range_x = (piece_idx % 8);
//   range_y = 7 - (piece_idx / 8);
//   range_max = range_x.min(range_y);
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask >> (9 * i)) & !friendly_pos_mask  & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }



//   // delta = -8
//   prev = piece_mask;
//   attack_toggle = 0;
//   range_max = piece_idx/8;
//   has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask << (i*8)) & !friendly_pos_mask & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }

//   //delta = 8
  
//   prev = piece_mask;
//   attack_toggle = 0;
//   range_max = (63-piece_idx)/8;
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask >> (i*8)) & !friendly_pos_mask & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }   
    
//   //delta = -1
//   prev = piece_mask;
//   attack_toggle = 0;
//   range_max = (piece_idx % 8);
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask << (i as i32)) & !friendly_pos_mask  & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }

//   //delta = 1
//   prev = piece_mask;
//   attack_toggle = 0;
//   range_max = 7 - (piece_idx % 8);
//   for i in (1..=range_max){
//     has_been_blocked = !(prev == 0) as u64 * u64::MAX;
//     prev = has_been_blocked & (piece_mask >> (i as i32)) & !friendly_pos_mask  & !attack_toggle;
//     if prev == 0 {
//       break;
//     }
//     attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
//     result |= prev;
//   }
  
//   return result;
// }