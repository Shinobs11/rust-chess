#![feature(portable_simd)]
#![feature(stdsimd)]


use std::arch::x86_64::{_mm512_and_epi64, _mm512_or_epi64};
use std::{arch::x86_64::_popcnt64, ops::Shl, ops::Shr};
use Which::{First, Second};
use chesslib::cache::*;
use chesslib::chess::bit::*;
use chesslib::chess::check::is_king_in_check;
use chesslib::chess::chess::parse_fens;
use chesslib::chess::consts::*;
use chesslib::chess::types::*;
use chesslib::chess::attack_bitmask::*;
use std::{iter::*, fmt};
use std::simd::*;
use std::fmt::*;
use bitvec::{prelude::*, view::BitView};
pub fn construct_bitmask_from_vec(v: &Vec<u8>){

}

pub type BitBoard = u64;

fn main() {
  let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";








  // // println!("{}", rook_mask << -8);
  
  // println!("rook_mask");
  // print_board(rook_mask);
  // println!("white_mask");
  // print_board(white_mask);
  // println!("black_mask");
  // print_board(black_mask);
  // println!("res");
  // print_board(rook_attack_mask(rook_mask, white_mask, black_mask));
  // 0b00001000
  // 0b11100111

  // let friend:u8 =     0b00001000;
  // let foe:u8 =        0b11100111;
  // println!("ter_cache[8]: {}", TERNARY_CACHE[8]);
  // println!("ter_cache[231]: {}", TERNARY_CACHE[231]);
  // println!("friend: {friend}");
  // println!("foe: {foe}");
  // let rook_mask:u8 =  0b00001000;
  // let rook_idx:u8 = rook_mask.leading_zeros() as u8;
  // println!("rook_idx: {}", rook_idx);
  // let row_mask = (TERNARY_CACHE[friend as usize] 
  //                   + 2*TERNARY_CACHE[foe as usize])
  //                   | ((rook_idx as u16) << 13);

  // println!("{row_mask}");
  // let ter = (TERNARY_CACHE[friend as usize] + 2 * TERNARY_CACHE[foe as usize]) | ((rook_idx as u16) << 13);
  // let res = RAY_CACHE[row_mask as usize];
  // // // let rc = ray_cache();


  // let mut n = 0;
  // for i in RAY_CACHE.iter() {
  //   if *i>0 {
  //     n+=1;
  //   }
  // }
  // println!("n: {}", n);
  // println!("res");
  // print_board(res);


  // let test_bb:u8 = 0xFA;
  // print_board(test_bb as u64);
  // print_board(put_col_mask(test_bb, 5));

    
    // let rook_idx: u8 = 2;
    // let friend_mask:u8 = 0b000000010;
    // let foe_mask:u8 = 0b00000010;
    // //0b0001000000000001
    // let m = get_ternary_bitmask(rook_idx, friend_mask, foe_mask);
    // println!("{:#010b}", RAY_CACHE[m as usize]);

fn convert_bit_slice_to_u64(bs: &BitSlice)->u64{
  let mut res:u64 = 0;
  for (i, x) in bs.to_bitvec().iter().enumerate(){
    res |= (1u64 << (63 - i)) & (((*x) as u64) * u64::MAX);
  }
  return res;
}
  // let friend_mask_arr = bits![
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 1, 0, 0, 0,
  //   0, 1, 0, 1, 0, 0, 1, 0,
  //   1, 0, 1, 0, 1, 1, 0, 1,
  //   1, 1, 1, 1, 1, 0, 1, 1
  // ];
  // let foe_mask_arr = bits![
  //   1, 1, 1, 1, 1, 0, 1, 1,
  //   1, 1, 1, 0, 0, 1, 1, 1,
  //   0, 0, 0, 1, 0, 0, 1, 0,
  //   0, 0, 0, 0, 1, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  // ];
  // let bishop_mask_arr = bits![
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 1, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  // ];
  // let expected_result = bits![
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 1, 0, 0, 0, 0, 0, 0,
  //   0, 0, 1, 0, 0, 0, 1, 0,
  //   0, 0, 0, 1, 0, 1, 0, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  //   0, 0, 0, 0, 0, 1, 0, 0,
  //   0, 0, 0, 0, 0, 0, 1, 0,
  //   0, 0, 0, 0, 0, 0, 0, 0,
  // ];
  // let expected = convert_bit_slice_to_u64(expected_result);
  // println!("expected result: {}", board_to_string(expected));
  // let friend_mask = convert_bit_slice_to_u64(friend_mask_arr);
  // let foe_mask = convert_bit_slice_to_u64(foe_mask_arr);
  // let bishop_mask = convert_bit_slice_to_u64(bishop_mask_arr);

  // let b_attack_mask = bishop_attack_mask(bishop_mask, friend_mask, foe_mask);
  // println!("actual result: {}", board_to_string(b_attack_mask));
  // const GET_COL_MAGIC:u64 = 0x2040810204081u64;
  // let diag_num:u64  = convert_bit_slice_to_u64(diagonals);
  // let mask_num = convert_bit_slice_to_u64(mask);
  // let test_magic_num = convert_bit_slice_to_u64(test_magic);
  // println!("{:x}", test_magic_num);
  // let mut s = String::new();
  // for i in (0..8){
  //   let mut row_str = String::new();
  //   for j in (0..8){
  //     let d = (i*8 + j)/8;
  //     let d_str = d.to_string();

  //     row_str.push_str(d_str.as_str());
  //     row_str.push(' ');
  //   }
  //   row_str.push_str("   ");
  //   for j in (0..8){
  //     let m = (i*8 + j) % 8;
  //     let m_str = m.to_string();

  //     row_str.push_str(m_str.as_str());
  //     row_str.push(' ');
  //   }
  //   row_str.push_str("   ");
  //   for j in (0..8){
  //     let d = 7 - (i*8 + j)/8;
  //     let d_str = d.to_string();

  //     row_str.push_str(d_str.as_str());
  //     row_str.push(' ');
  //   }
  //   row_str.push_str("   ");
  //   for j in (0..8){
  //     let m = 7 - (i*8 + j) % 8;
  //     let m_str = m.to_string();

  //     row_str.push_str(m_str.as_str());
  //     row_str.push(' ');
  //   }
  //   row_str.push('\n');
  //   s.push_str(row_str.as_str());
  // }
  // println!("{}", s);


  // // // let col_num = convert_bit_slice_to_u64(col);
  // let masked_diag_num = diag_num & mask_num;
  // print_mult_steps(masked_diag_num, test_magic_num);
  // println!();
  // print_board((masked_diag_num * test_magic_num) >> 56);
  // let mult_num = convert_bit_slice_to_u64(mult);
  // print_board(GET_COL_MAGIC);
  // print_board(diag_num);
  // println!();
  // print_board(mask_num);
  // println!();
  // print_board((diag_num & mask_num));
  // println!();
  // print_board((diag_num & mask_num) * 0xffedf9fd7cfcffff);

  // for (i, x) in DIAG_MASK_CACHE.iter().enumerate() {
  //   println!("{i}");
  //   print_board(*x);
  //   println!();
  // }





    // print_board(KNIGHT_CACHE[41]);

    // const KNIGHT_OFFSETS:[i32; 8] = [-17, -15, 15, 17, -10, -6, 6, 10];
    // for offset in KNIGHT_OFFSETS.iter() {
    //   println!("offset: {}", offset);
    //   print_board(1 << (63 - (41 + offset)));
    //   println!();
    // }
    


    // let king_pos = bits![
    //     0, 0, 0, 0, 1, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0,
    //     0, 0, 0, 0, 0, 0, 0, 0,
    //   ];
    // let mult = bits![
    //   0, 0, 0, 0, 0, 0, 0, 0,
    //   0, 0, 0, 0, 0, 0, 0, 0,
    //   0, 0, 0, 0, 0, 0, 0, 0,
    //   0, 0, 0, 0, 0, 0, 0, 0,
    //   0, 0, 0, 0, 0, 0, 0, 0,
    //   0, 0, 0, 0, 0, 0, 0, 0,
    //   0, 0, 0, 0, 0, 0, 0, 0,
    //   0, 0, 0, 0, 0, 1, 1, 1,
    // ];
    
    // let king_pos_n = convert_bit_slice_to_u64(king_pos);
    // let mult_n = convert_bit_slice_to_u64(mult);
    // let king_idx = king_pos_n.leading_zeros();
    // println!("{king_idx}");

    // let left = ((king_pos_n & 0x7f7f7f7f7f7f7f7f) << 1);
    // let upper_left = left << 8;
    // let lower_left = left >> 8; 
    // let right = ((king_pos_n & 0xfefefefefefefefe) >> 1);
    // let upper_right = right << 8;
    // let lower_right = right >> 8;
    // let top = king_pos_n << 8;
    // let bottom = king_pos_n >> 8;
    // let res = left | upper_left | lower_left | right | upper_right | lower_right | top | bottom;
    // print_board(res);

    // let l1:u64 = ((1 << (63 - 41)) >> 1) & 0x7f7f7f7f7f7f7f7f;
    // let l2:u64  = ((1 << (63 - 41)) >> 2) & 0x3f3f3f3f3f3f3f3f;
    // let r1: u64 = ((1 << (63 - 41)) << 1) & 0xfefefefefefefefe;
    // let r2: u64 = ((1 << (63 - 41)) << 2) & 0xfcfcfcfcfcfcfcfc;
    // let h1 = l1 | r1;
    // let h2 = l2 | r2;
    // print_board(h1);
    // println!();
    // print_board(h2);
    // println!(); 
    // print_board((h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8));


  let b = Board::board_from_fen("r4rk1/pb3pp1/1p2p2p/2Pq2P1/1b2N2P/4PP2/PpQ5/2KR2NR w - - 0 17".to_string());
  // print_board(b.color_masks[Color::White]);
  // print_board(b.color_masks[Color::Black]);
  println!("{}", b);
  let x = is_king_in_check(Color::White, b);
  println!("{}", x);


}


fn print_mult_steps(lhs: u64, rhs: u64) {
  let mut res:u64 = 0;
  for (i, x) in lhs.view_bits::<Lsb0>().iter().enumerate(){
    let toggle = (*x as u64) * u64::MAX;
    let mult = (toggle & rhs);
    if mult == 0{
      continue;
    }
    let shifted = (mult << i);
    
    let shifted_str = board_to_string(shifted);
    let prev_str = board_to_string(res);
    res += shifted;
    let post_str = board_to_string(res);

    let shifted_vec:Vec<&str> = shifted_str.split('\n').collect();
    let prev_vec:Vec<&str> = prev_str.split('\n').collect();
    
    let post_vec:Vec<&str> = post_str.split('\n').collect();
    println!("{}", i);

    for j in (0..8) {
      println!("{}  {}  {}",post_vec[j], prev_vec[j], shifted_vec[j]);

    }
  }    
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
fn print_board(b:BitBoard){
  let slice = b.view_bits::<Msb0>();
  for x in (0..8){
    for y in (0..8){
      print!("{:3?}", slice[x*8 + y] as u8);
    }
    print!("\n");
  }
}