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


fn convert_bit_slice_to_u64(bs: &BitSlice)->u64{
  let mut res:u64 = 0;
  for (i, x) in bs.to_bitvec().iter().enumerate(){
    res |= (1u64 << (63 - i)) & (((*x) as u64) * u64::MAX);
  }
  return res;
}


  let b = Board::board_from_fen("r4rk1/pb3pp1/1p2p2p/2Pq2P1/1b2N2P/4PP2/PpQ5/2KR2NR w - - 0 17".to_string());
  // print_board(b.color_masks[Color::White]);
  // print_board(b.color_masks[Color::Black]);
  println!("{}", b);
  let x = is_king_in_check(Color::White, b);
  println!("{}", x);

  let m = Move::default();
  

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