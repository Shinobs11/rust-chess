#![feature(portable_simd)]
#![feature(stdsimd)]
mod util;

use std::arch::x86_64::{_mm512_and_epi64, _mm512_or_epi64};
use std::{arch::x86_64::_popcnt64, ops::Shl, ops::Shr};
use Which::{First, Second};
use chess_game::attack_bitmask::*;
use util::consts::*;
use util::chess::*;
use std::iter::*;
use crate::util::types::Board;
use std::simd::*;

pub fn construct_bitmask_from_vec(v: &Vec<u8>){

}

pub type BitBoard = u64;

fn main() {
  let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";



  // let rook_mask:BitBoard =  0b0000000000000000000000000000000000000000000000000100000000000000;
  // let bishop_mask:BitBoard =   0b0000000000000000000000000000000000001000000000000000000000000000;
  // // let rook_mask:BitBoard = 0b0000000000000010000000000000000000000000000000000000000000000000;
  // let white_mask:BitBoard =  0b0000000000000000000000000000010000000000001000001111111111111111;
  // let black_mask:BitBoard =  0b0100000101000001000000000000000000000000000100000000000000000000;
  


  // println!("{}", rook_mask << -8);
  
  // println!("bishop_mask");
  // print_board(bishop_mask);
  // println!("white_mask");
  // print_board(white_mask);
  // println!("black_mask");
  // print_board(black_mask);
  // println!("res");
  // print_board(branchless_bishop(bishop_mask, white_mask, black_mask));





  // println!("res");
  // print_board(res);


  for (i, x) in knight_cache().iter().enumerate() {
    // println!("{}", x);
    println!("idx: {}", i);
    print_board(*x);
    // println!();
  }


    
}

use bitvec::{prelude::*, view::BitView};

fn print_board(b:BitBoard){
  let slice = b.view_bits::<Lsb0>();

  for x in (0..8){
    for y in (0..8){
      print!("{:3?}", slice[x*8 + y] as u8);
    }
    print!("\n");
  }
}