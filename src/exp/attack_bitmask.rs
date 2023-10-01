#![feature(portable_simd)]
#![feature(stdsimd)]
#![feature(const_for)]
#![feature(const_trait_impl)]
#![feature(const_trait_impl)]
#[deny(long_running_const_eval)]
pub type BitBoard = u64;
use Which::{First, Second};
use bitvec::ptr::null;
use std::simd::*;
use bitvec::{prelude::*, view::BitView};
use std::{ ops::Shl, ops::Shr};
use std::collections::*;
use crate::consts::*;
use crate::bit::*;

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



















/*
This is just for starters, there's a million and one ways I can cut this down in size from 500K+ u8s
*/
pub const fn rook_cache()->[u8; 8*u16::MAX as usize]{
  let mut cache:[u8; 8*u16::MAX as usize] = [0; 8*u16::MAX as usize];
  let mut r_idx:usize = 0;
  while r_idx < 8 {
    let rook_mask = (1u8 << (7 - r_idx));
    let mut comb = 0;


    while comb < u16::MAX{
      let friends_mask = ((comb & 0x00FF)  as u8); //least significant 8 bits
      let foes_mask = (comb >> 8) as u8; //most significant 8 bits
      let mut prev = rook_mask;
      let mut attack_toggle:u8 = 0; 
      let mut res:u8 = 0;
      let mut i = 1;
      //positive dir
      let mut range_max = 7 - r_idx; //2
      let mut has_been_blocked: u8;
      while i <= range_max {
        has_been_blocked = !(prev == 0) as u8 * u8::MAX;
        prev = has_been_blocked & (rook_mask >> i) & !friends_mask & !attack_toggle;
        if prev == 0 {
          break;
        }
        attack_toggle = ((prev & foes_mask) > 0) as u8 * u8::MAX;
        res |= prev;
        i += 1;
      }
      range_max = r_idx;
      i = 1;
      prev = rook_mask;
      attack_toggle = 0;
      while i <= range_max {
        has_been_blocked = !(prev == 0) as u8 * u8::MAX;
        prev = has_been_blocked & (rook_mask << i) & !friends_mask & !attack_toggle;
        if prev == 0 {
          break;
        }
        attack_toggle = ((prev & foes_mask) > 0) as u8 * u8::MAX;
        res |= prev;
        i += 1;
      }

      cache[(r_idx * u16::MAX as usize) + comb as usize] = res;
      comb += 1;
    }
    r_idx += 1;
  }
  return cache;
}

pub const ROOK_CACHE:[u8; 8*u16::MAX as usize] = rook_cache();


pub fn rook_attack_mask(piece_mask:u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64{
  let piece_idx:u8 = piece_mask.leading_zeros() as u8;
  let piece_row_idx = piece_idx / 8;
  let piece_col_idx = piece_idx % 8;
  // println!("piece_row_idx: {}", piece_row_idx);
  // println!("piece_col_idx: {}", piece_col_idx);
  let row_mask = ((get_row_mask(opponent_pos_mask, piece_row_idx) as u16) << 8) | get_row_mask(friendly_pos_mask, piece_row_idx) as u16;
  let col_mask = ((get_col_mask(opponent_pos_mask, piece_col_idx) as u16) << 8) | get_col_mask(friendly_pos_mask, piece_col_idx) as u16;
  // println!("row_mask: {:#018b}", row_mask);
  // println!("col_mask: {:#018b}", col_mask);
  //bit of a gotcha, need to provide piece_col_idx to index row_attack_mask and vice versa
  let row_attack_mask = ROOK_CACHE[((piece_col_idx as usize * u16::MAX as usize) + row_mask as usize)];
  let col_attack_mask = ROOK_CACHE[((piece_row_idx as usize * u16::MAX as usize) + col_mask as usize)];
  
  // println!("row_attack_mask: {:#010b}", row_attack_mask);
  // println!("col_attack_mask: {:#010b}", col_attack_mask);
  return (put_row_mask(row_attack_mask, piece_row_idx) | put_col_mask(col_attack_mask, piece_col_idx)) & !piece_mask;
}

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


pub fn simd_branchless_rook(piece_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64{

  let piece_idx:u64 = piece_mask.leading_zeros() as u64;
  let mut result:u64x4 = u64x4::from_array([0;4]);
  let simd_friendly_pos_mask = u64x4::from_array([friendly_pos_mask; 4]);
  let simd_opponent_pos_mask = u64x4::from_array([opponent_pos_mask; 4]);

  let negative_offsets:u64x2 = u64x2::from_array([8, 1]);
  let positive_offsets:u64x2 = u64x2::from_array([8, 1]);

  let mut prev:u64x4 = u64x4::from_array([piece_mask;4]);
  let mut attack_toggle:u64x4 = u64x4::from_array([0;4]);
  // let mut range_max:u64x4 = u64x4::from_array([piece_idx/8, (63-piece_idx)/8, piece_idx % 8, 7 - piece_idx % 8]);
  let lb:u64x4 = u64x4::from([0, 0, piece_idx & !7, piece_idx & !7]);
  let ub:u64x4 = u64x4::from([63, 63, piece_idx | 7, piece_idx | 7]);
  // let simd_piece_mask = u64x4::from_array([piece_mask; 4]);
  for i in (1..8){
    let simd_i:u64x4 = u64x4::from([i;4]);
    //for tomorrow: Masks seem to have 0 as false and -1 as true
    let has_been_blocked:u64x4 = prev.simd_ne(u64x4::from_array([0;4])).to_int().abs().cast() * u64x4::from_array([u64::MAX; 4]);
    let left_shift = u64x2::from_array([piece_mask;2]) << (simd_swizzle!(simd_i, [0, 2]) * negative_offsets);
    
    let right_shift = u64x2::from_array([piece_mask;2]) >> (simd_swizzle!(simd_i, [1, 3]) * positive_offsets);
    let shift:Simd<u64, 4> = simd_swizzle!(left_shift, right_shift, [First(0), Second(0), First(1), Second(1)]);
    let offsets:i64x4 = simd_i.cast() * i64x4::from_array([-8, 8, -1, 1]);
    let checked_shift:Simd<u64, 4> = (offsets.simd_ge(lb.cast()) & offsets.simd_le(ub.cast())).to_int().abs().cast() * u64x4::from_array([u64::MAX; 4]) & shift;
    
    prev = has_been_blocked & checked_shift & !simd_friendly_pos_mask & !attack_toggle;
    attack_toggle = ((prev & simd_opponent_pos_mask).simd_gt(u64x4::from_array([0; 4])).to_int().abs().cast() * u64x4::from_array([u64::MAX;4]));
    result |= prev;
  }
  return result.reduce_and();
}





pub fn batch_rook_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(rook_attack_mask(*piece_mask, *friend_mask, *foe_mask));
  }
  return res;
}

pub fn batch_simd_branchless_rook(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(simd_branchless_rook(*piece_mask, *friend_mask, *foe_mask));
  }
  return res;
}





/*
For the branchless bishop, an optimization to consider that could also be applied to the branchless rook
is to calculate range_max with a bitwise AND, potentially saving us a couple of ops in latency.


*/
pub fn bishop_attack_mask(piece_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64{
  let piece_idx = piece_mask.leading_zeros();
  let mut result:u64 = 0;
  // delta = -7
  let mut prev:u64 = piece_mask;
  let mut attack_toggle:u64 = 0;
  
  let mut range_x = 7 - (piece_idx % 8);
  let mut range_y = piece_idx / 8;
  let mut range_max:u32 = range_x.min(range_y);

  let mut has_been_blocked:u64;
  for i in (1..=range_max){
    has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (piece_mask << (7 * i)) & !friendly_pos_mask & !attack_toggle;
    if prev == 0 {
      break;
    }
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = 7
  
  prev = piece_mask;
  attack_toggle = 0;
  range_x = 7 - (piece_idx % 8);
  range_y = 7 - (piece_idx / 8);
  range_max = range_x.min(range_y);
  for i in (1..=range_max){
    has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (piece_mask >> (7 * i)) & !friendly_pos_mask & !attack_toggle;
    if prev == 0 {
      break;
    }
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = -9
  prev = piece_mask;
  attack_toggle = 0;
  range_x = (piece_idx % 8);
  range_y = (piece_idx / 8);
  range_max = range_x.min(range_y);
  for i in (1..=range_max){
    has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (piece_mask << (9 * i)) & !friendly_pos_mask  & !attack_toggle;
    if prev == 0 {
      break;
    }
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = 9
  prev = piece_mask;
  attack_toggle = 0;
  range_x = (piece_idx % 8);
  range_y = 7 - (piece_idx / 8);
  range_max = range_x.min(range_y);
  for i in (1..=range_max){
    has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (piece_mask >> (9 * i)) & !friendly_pos_mask  & !attack_toggle;
    if prev == 0 {
      break;
    }
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }
  
  return result;
}

pub fn batch_bishop_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(bishop_attack_mask(*piece_mask, *friend_mask, *foe_mask));
  }
  return res;
}



pub fn queen_attack_mask(piece_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64 {

  let piece_idx = piece_mask.leading_zeros();
  let mut result:u64 = 0;
  // delta = -7
  let mut prev:u64 = piece_mask;
  let mut attack_toggle:u64 = 0;
  
  let mut range_x = 7 - (piece_idx % 8);
  let mut range_y = piece_idx / 8;
  let mut range_max:u32 = range_x.min(range_y);

  let mut has_been_blocked:u64;
  for i in (1..=range_max){
    has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (piece_mask << (7 * i)) & !friendly_pos_mask & !attack_toggle;
    if prev == 0 {
      break;
    }
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = 7
  
  prev = piece_mask;
  attack_toggle = 0;
  range_x = 7 - (piece_idx % 8);
  range_y = 7 - (piece_idx / 8);
  range_max = range_x.min(range_y);
  for i in (1..=range_max){
    has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (piece_mask >> (7 * i)) & !friendly_pos_mask & !attack_toggle;
    if prev == 0 {
      break;
    }
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = -9
  prev = piece_mask;
  attack_toggle = 0;
  range_x = (piece_idx % 8);
  range_y = (piece_idx / 8);
  range_max = range_x.min(range_y);
  for i in (1..=range_max){
    has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (piece_mask << (9 * i)) & !friendly_pos_mask  & !attack_toggle;
    if prev == 0 {
      break;
    }
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = 9
  prev = piece_mask;
  attack_toggle = 0;
  range_x = (piece_idx % 8);
  range_y = 7 - (piece_idx / 8);
  range_max = range_x.min(range_y);
  for i in (1..=range_max){
    has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (piece_mask >> (9 * i)) & !friendly_pos_mask  & !attack_toggle;
    if prev == 0 {
      break;
    }
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }



  // delta = -8
  prev = piece_mask;
  attack_toggle = 0;
  range_max = piece_idx/8;
  has_been_blocked = !(prev == 0) as u64 * u64::MAX;
  for i in (1..=range_max){
    has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (piece_mask << (i*8)) & !friendly_pos_mask & !attack_toggle;
    if prev == 0 {
      break;
    }
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = 8
  
  prev = piece_mask;
  attack_toggle = 0;
  range_max = (63-piece_idx)/8;
  for i in (1..=range_max){
    has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (piece_mask >> (i*8)) & !friendly_pos_mask & !attack_toggle;
    if prev == 0 {
      break;
    }
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }   
    
  //delta = -1
  prev = piece_mask;
  attack_toggle = 0;
  range_max = (piece_idx % 8);
  for i in (1..=range_max){
    has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (piece_mask << (i as i32)) & !friendly_pos_mask  & !attack_toggle;
    if prev == 0 {
      break;
    }
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = 1
  prev = piece_mask;
  attack_toggle = 0;
  range_max = 7 - (piece_idx % 8);
  for i in (1..=range_max){
    has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (piece_mask >> (i as i32)) & !friendly_pos_mask  & !attack_toggle;
    if prev == 0 {
      break;
    }
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }
  
  return result;
}

pub fn batch_queen_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(queen_attack_mask(*piece_mask, *friend_mask, *foe_mask));
  }
  return res;
}


/*
  There's two factors that let the attack mask for pawns, knights and kings be calculated far faster than bishops, rooks or queens.
  1. Pawns, knights and kings don't "slide" across the board, they jump to their positions.
  2. Because of the small search space, we can reasonably pre-calculate all of their possible attacks.
*/

pub const fn knight_cache()->[u64; 64]{
  const KNIGHT_OFFSETS_1:[i32; 4] = [-17, -15, 15, 17];
  const KNIGHT_OFFSETS_2:[i32; 4] = [-10, -6, 6, 10];
  let mut out:[u64; 64] = [0; 64];
  let mut idx = 0;
  while idx < 64 {
    let mut res = 0;
    let mut target_pos:i32 = 0;
    let mut d:i32 = i32::MIN;
    let mut d_idx = 0;
    while d_idx < 4 {
      d = KNIGHT_OFFSETS_1[d_idx];
      target_pos = idx + d;
      if (target_pos >= 0 && target_pos <=63) && (target_pos/8 - idx/8 ) == (2 * d.signum()){
        res |= if (d.is_positive()) {left_shift((1u64 << (63 - idx)) as u64, d as u8)} else {right_shift((1u64 << (63 - idx)) as u64, d.abs() as u8)};
      }
      d_idx += 1;
    }
    d_idx = 0;
    while d_idx < 4 {
      d = KNIGHT_OFFSETS_2[d_idx];
      target_pos = idx + d;
      if (target_pos >= 0 && target_pos <=63) && (target_pos/8 - idx/8) == (d.signum()){
        res |= if (d.is_positive()) {left_shift((1u64 << (63 - idx)) as u64, d as u8)} else {right_shift((1u64 << (63 - idx)) as u64, d.abs() as u8)};
      }
      d_idx += 1;
    }
    out[idx as usize] = res;
    idx+=1;
  }
  return out;
}
pub const KNIGHT_CACHE:[u64; 64] = knight_cache();

pub fn knight_attack_mask(piece_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64{
  return KNIGHT_CACHE[piece_mask.leading_zeros() as usize] & !friendly_pos_mask;
}

pub fn batch_knight_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(knight_attack_mask(*piece_mask, *friend_mask, *foe_mask));
  }
  return res;
}