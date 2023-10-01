

// pub fn setBit(x:&mut u64, n:u8){
//   *x |= (1 << n);
// }

// pub fn pair_into_bit_offset(p:(i32, i32))->i32 {
//   return p.0 + p.1*8;
// }

use std::arch::x86_64::_lzcnt_u64;

#[inline]
pub const fn left_shift(v:u64, n:u8)->u64{
  return v << n;
}
#[inline]
pub const fn right_shift(v:u64, n:u8)->u64{
  return v >> n;
}

#[inline]
pub const fn get_row_mask(bb: u64, row_idx: u8)->u8{
  //because msb is index 0, we need to shift by (7 - row_idx)*8;
  return ((bb >> ((7 - row_idx) * 8)) & (u8::MAX as u64)) as u8;
}

#[inline]
pub const fn put_row_mask(bb_row: u8, row_idx: u8)->u64{
  return ((bb_row as u64) << ((7-row_idx) * 8));

}

/*
from: https://stackoverflow.com/questions/14537831/isolate-specific-row-column-diagonal-from-a-64-bit-number
holy fuck this is so cool.
my understanding of the mechanism at work here:
first we shift over the column we want to be aligned along the MSB.
so if we're interested in column index 4 (or E), we'd shift to the left by 4 and now they're all on column index 0
next, the const "column_mask" is & with the resulting bitboard to filter out unwanted bits.
the cool part comes next and I'm not sure exactly how it does what it does, but effectively
each column will now be the first column but "upshifted" by their column index. as a consequence,
the first column is now also identical to the first row, allowing us to do a simple bitshift and/or mask to effectively transpose the column
along a row axis.  
*/
const COLUMN_MASK:u64 = 0x8080808080808080u64;
const GET_COL_MAGIC:u64 = 0x2040810204081u64;
#[inline]
pub fn get_col_mask(bb: u64, col: u8)->u8 {
  let mut column = (bb << col) & COLUMN_MASK;
  column *= GET_COL_MAGIC;
  return ((column >> 56) & 0xff) as u8;
}

#[inline]
pub fn put_col_mask(bb_col: u8, col: u8) -> u64 {
  return ((bb_col as u64 * GET_COL_MAGIC) & 0x0101010101010101) << (7 - col);
}


// pub fn select_bit_with_rank(vo: u64, ro: u64)->u64{
//   let mut r = ro;
//   let mut v = vo;
//   let (mut a, mut b, mut c, mut d): (u64, u64, u64, u64) = (0, 0, 0, 0);
//   let mut t:u64 = 0;
//   let mut s:u64 = 0;
//   // Do a normal parallel bit count for a 64-bit integer,                     
//   // but store all intermediate steps.                                        
//   // a = (v & 0x5555...) + ((v >> 1) & 0x5555...);
//   a =  v - ((v >> 1) & !0u64/3);
//   // b = (a & 0x3333...) + ((a >> 2) & 0x3333...);
//   b = (a & !0u64/5) + ((a >> 2) & !0u64/5);
//   // c = (b & 0x0f0f...) + ((b >> 4) & 0x0f0f...);
//   c = (b + (b >> 4)) & !0u64/0x11;
//   // d = (c & 0x00ff...) + ((c >> 8) & 0x00ff...);
//   d = (c + (c >> 8)) & !0u64/0x101;
//   t = (d >> 32) + (d >> 48);
//   // Now do branchless select!                                                
//   s  = 64;
//   // if (r > t) {s -= 32; r -= t;}
//   s -= ((t - r) & 256) >> 3; r -= (t & ((t - r) >> 8));
//   t  = (d >> (s - 16)) & 0xff;
//   // if (r > t) {s -= 16; r -= t;}
//   s -= ((t - r) & 256) >> 4; r -= (t & ((t - r) >> 8));
//   t  = (c >> (s - 8)) & 0xf;
//   // if (r > t) {s -= 8; r -= t;}
//   s -= ((t - r) & 256) >> 5; r -= (t & ((t - r) >> 8));
//   t  = (b >> (s - 4)) & 0x7;
//   // if (r > t) {s -= 4; r -= t;}
//   s -= ((t - r) & 256) >> 6; r -= (t & ((t - r) >> 8));
//   t  = (a >> (s - 2)) & 0x3;
//   // if (r > t) {s -= 2; r -= t;}
//   s -= ((t - r) & 256) >> 7; r -= (t & ((t - r) >> 8));
//   t  = (v >> (s - 1)) & 0x1;
//   // if (r > t) s--;
//   s -= ((t - r) & 256) >> 8;
//   s = 65 - s;
//   return s;
// } 
use bitvec::{prelude::*, view::BitView};


//from: me
//set bits to zero according to mask
// w = w & !m

//from: https://graphics.stanford.edu/~seander/bithacks.html#ConditionalSetOrClearBitsWithoutBranching
//conditionally set bits without branching
// bool f;         // conditional flag
// unsigned int m; // the bit mask
// unsigned int w; // the word to modify:  if (f) w |= m; else w &= ~m; 

// w ^= (-f ^ w) & m;

// // OR, for superscalar CPUs:
// w = (w & ~m) | (-f & m);

//find minimum of two integers without branching.
//r = y + ((x - y) & ((x - y) >> (sizeof(int) * CHAR_BIT - 1))); // min(x, y)