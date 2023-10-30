use std::cmp::Ordering;
use chesslib::chess::attack_bitmask::*;
use bitvec::{prelude::*, view::BitView};
fn convert_bit_slice_to_u64(bs: &BitSlice)->u64{
  let mut res:u64 = 0;
  for (i, x) in bs.iter().enumerate(){
    res |= (1 << (63 - i)) & (((*x) as u64) * u64::MAX);
  }
  return res;
}
fn board_to_string(b: BitBoard)->String{
  let slice = b.view_bits::<Msb0>();
  let mut s = String::new();
  for x in (0..8){
    for y in (0..8){
      s.push_str(format!("{:3?}", slice[x*8 + y] as u8).as_str());
    }
    s.push('\n');
  }
  return s;
}
#[test]
fn test_bishop_attack_mask(){
  let friend_mask_arr = bits![
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 1, 0, 0, 0,
    0, 1, 0, 1, 0, 0, 1, 0,
    1, 0, 1, 0, 1, 1, 0, 1,
    1, 1, 1, 1, 1, 0, 1, 1
  ];
  let foe_mask_arr = bits![
    1, 1, 1, 1, 1, 0, 1, 1,
    1, 1, 1, 0, 0, 1, 1, 1,
    0, 0, 0, 1, 0, 0, 1, 0,
    0, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
  ];
  let bishop_mask_arr = bits![
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
  ];
  let expected_result = bits![
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 0, 0, 0, 0, 0, 0,
    0, 0, 1, 0, 0, 0, 1, 0,
    0, 0, 0, 1, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 0, 0,
    0, 0, 0, 0, 0, 0, 1, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
  ];
  let expected = convert_bit_slice_to_u64(expected_result);
  println!("expected result: {}", board_to_string(expected));
  let friend_mask = convert_bit_slice_to_u64(friend_mask_arr);
  let foe_mask = convert_bit_slice_to_u64(foe_mask_arr);
  let bishop_mask = convert_bit_slice_to_u64(bishop_mask_arr);

  let b_attack_mask = bishop_attack_mask(bishop_mask, friend_mask, foe_mask);
  println!("actual result: {}", board_to_string(b_attack_mask));

  assert_eq!(expected, b_attack_mask);
  


}
