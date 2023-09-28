pub type BitBoard = u64;

use core::simd;
use Which::{First, Second};
use std::simd::*;
use bitvec::{prelude::*, view::BitView};
fn get_bit_idx(n:u64)->usize{
  return n.view_bits::<Msb0>().first_one().unwrap();
}

pub fn test(lhs:u64, sh:u32)->u64{
  let a = lhs << sh;
  return a;
}

pub fn branchless_rook(rook_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64{
  let rook_idx = rook_mask.leading_zeros();
  let mut result:u64 = 0;

  //potential optimization, I might be able to avoid using an 8 element array here, since I only need to consider the immediately previous result.
  // delta = -8
  let mut prev:u64 = rook_mask;
  let mut attack_toggle:u64 = 0;
  let mut range_max:u32 = rook_idx/8;
  for i in (1..=range_max){
    let has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (rook_mask << (i*8)) & !friendly_pos_mask & !attack_toggle;
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = 8
  
  prev = rook_mask;
  attack_toggle = 0;
  range_max = (63-rook_idx)/8;
  for i in (1..=range_max){
    let has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (rook_mask >> (i*8)) & !friendly_pos_mask & !attack_toggle;
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = -1
  prev = rook_mask;
  attack_toggle = 0;
  range_max = (rook_idx % 8);
  for i in (1..=range_max){
    let has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (rook_mask << (i as i32)) & !friendly_pos_mask  & !attack_toggle;
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }

  //delta = 1
  prev = rook_mask;
  attack_toggle = 0;
  range_max = 7 - (rook_idx % 8);
  for i in (1..=range_max){
    let has_been_blocked = !(prev == 0) as u64 * u64::MAX;
    prev = has_been_blocked & (rook_mask >> (i as i32)) & !friendly_pos_mask  & !attack_toggle;
    attack_toggle = ((prev & opponent_pos_mask) > 0) as u64 * u64::MAX;
    result |= prev;
  }
  
  return result;
}




fn batch_branchless_rook(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (rook_mask, friend_mask, foe_mask) in v {
    res.push(branchless_rook(*rook_mask, *friend_mask, *foe_mask));
  }
  return res;
}

fn simd_branchless_rook(rook_mask: u64, friendly_pos_mask: u64, opponent_pos_mask: u64)->u64{

  let rook_idx:u64 = rook_mask.leading_zeros() as u64;
  let mut result:u64x4 = u64x4::from_array([0;4]);
  let simd_friendly_pos_mask = u64x4::from_array([friendly_pos_mask; 4]);
  let simd_opponent_pos_mask = u64x4::from_array([opponent_pos_mask; 4]);

  let negative_offsets:u64x2 = u64x2::from_array([8, 1]);
  let positive_offsets:u64x2 = u64x2::from_array([8, 1]);

  let mut prev:u64x4 = u64x4::from_array([rook_mask;4]);
  let mut attack_toggle:u64x4 = u64x4::from_array([0;4]);
  // let mut range_max:u64x4 = u64x4::from_array([rook_idx/8, (63-rook_idx)/8, rook_idx % 8, 7 - rook_idx % 8]);
  let lb:u64x4 = u64x4::from([0, 0, rook_idx & !7, rook_idx & !7]);
  let ub:u64x4 = u64x4::from([63, 63, rook_idx | 7, rook_idx | 7]);
  let simd_rook_mask = u64x4::from_array([rook_mask; 4]);
  for i in (1..8){
    let simd_i:u64x4 = u64x4::from([i;4]);
    //for tomorrow: Masks seem to have 0 as false and -1 as true
    let has_been_blocked:u64x4 = prev.simd_ne(u64x4::from_array([0;4])).to_int().abs().cast() * u64x4::from_array([u64::MAX; 4]);
    let left_shift = u64x2::from_array([rook_mask;2]) << (simd_swizzle!(simd_i, [0, 2]) * negative_offsets);
    
    let right_shift = u64x2::from_array([rook_mask;2]) << (simd_swizzle!(simd_i, [1, 3]) * negative_offsets);
    let shift:Simd<u64, 4> = simd_swizzle!(left_shift, right_shift, [First(0), Second(0), First(1), Second(1)]);
    let offsets:i64x4 = simd_i.cast() * i64x4::from_array([-8, 8, -1, 1]);
    let checked_shift = (offsets.simd_ge(lb.cast()) | offsets.simd_le(ub.cast())).to_int().abs().cast();
    
    prev = has_been_blocked & shift & !simd_friendly_pos_mask & !attack_toggle;
    attack_toggle = ((prev & simd_opponent_pos_mask).simd_gt(u64x4::from_array([0; 4])).to_int().abs().cast() * u64x4::from_array([u64::MAX;4]));
    result |= prev;
  }




  
  
  return result.reduce_and();
}

