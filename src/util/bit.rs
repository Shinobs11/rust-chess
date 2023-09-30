

// pub fn setBit(x:&mut u64, n:u8){
//   *x |= (1 << n);
// }

// pub fn pair_into_bit_offset(p:(i32, i32))->i32 {
//   return p.0 + p.1*8;
// }

use std::arch::x86_64::_lzcnt_u64;

pub const fn left_shift(v:u64, n:u8)->u64{
  return v << n;
}
pub const fn right_shift(v:u64, n:u8)->u64{
  return v >> n;
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