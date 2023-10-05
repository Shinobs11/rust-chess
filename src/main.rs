#![feature(portable_simd)]
#![feature(stdsimd)]


use std::arch::x86_64::{_mm512_and_epi64, _mm512_or_epi64};
use std::{arch::x86_64::_popcnt64, ops::Shl, ops::Shr};
use Which::{First, Second};
use chesslib::cache::*;
use chesslib::chess::bit::*;
use chesslib::chess::consts::*;
use chesslib::chess::types::*;
use std::iter::*;
use std::simd::*;

pub fn construct_bitmask_from_vec(v: &Vec<u8>){

}

pub type BitBoard = u64;

fn main() {
  let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";



  let rook_mask:BitBoard =      0b0000000000000000000000000000000000000000010000000000000000000000;
  // let bishop_mask:BitBoard =    0b0000000000000000000000000000000000001000000000000000000000000000;
  // println!("{}", bishop_mask.leading_zeros());
  // // let rook_mask:BitBoard = 0b0000000000000010000000000000000000000000000000000000000000000000;
  let white_mask:BitBoard =     0b0000000000000000000000000000010000000000000000011111111111111111;
  let mut black_mask:BitBoard =     0b0100000101000001000000000000000000000000000100000000000000000000;
  // black_mask |= bishop_mask << 17;
  // black_mask |= bishop_mask << 15;
  // black_mask |= bishop_mask << 10;
  // black_mask |= bishop_mask << 6;
  




  // // println!("{}", rook_mask << -8);
  
  // println!("rook_mask");
  // print_board(rook_mask);
  // println!("white_mask");
  // print_board(white_mask);
  // println!("black_mask");
  // print_board(black_mask);
  // println!("res");
  // print_board(rook_attack_mask(rook_mask, white_mask, black_mask));


  // let friend:u8 = 0b01100000;
  // let foe:u8 = 0b00000000;
  // let rook_mask:u8 = 0b00000001;
  // let rook_idx:u16 = rook_mask.leading_zeros() as u16;
  // let comb = (foe as u16) << 8 | friend as u16;
  // let res = ROOK_CACHE[(rook_idx * u16::MAX) as usize + comb as usize];
  // // let rc = rook_cache();
  // println!("{:#010b}", res);

  // let mut n = 0;
  // for i in ROOK_CACHE.iter() {
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

    
    let rook_idx: u8 = 2;
    let friend_mask:u8 = 0b000000010;
    let foe_mask:u8 = 0b00000010;
    //0b0001000000000001
    let m = get_ternary_bitrow(rook_idx, friend_mask, foe_mask);
    println!("{:#010b}", ROOK_CACHE[m as usize]);



}

use bitvec::{prelude::*, view::BitView};

fn print_board(b:BitBoard){
  let slice = b.view_bits::<Msb0>();

  for x in (0..8){
    for y in (0..8){
      print!("{:3?}", slice[x*8 + y] as u8);
    }
    print!("\n");
  }
}