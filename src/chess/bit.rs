use crate::cache::{TERNARY_CACHE, DIAG_MASK_CACHE};



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

#[inline]
pub fn get_ternary_bitmask(piece_idx: u8, friend_mask: u8, foe_mask: u8)->u16{
  return ((piece_idx as u16)  << 13) | (TERNARY_CACHE[friend_mask as usize] as u16 + 2 * TERNARY_CACHE[foe_mask as usize] as u16);
}

pub const GET_DIAG_MASK_MAGIC:u64 = 0x101010101010101;
#[inline]
pub fn get_pos_diag_mask(bb:u64, piece_idx: u8)->u8{
  return (((bb & DIAG_MASK_CACHE[2*(piece_idx as usize)]) * GET_DIAG_MASK_MAGIC) >> 56) as u8;
}

#[inline]
pub fn get_neg_diag_mask(bb:u64, piece_idx: u8)->u8{
  return (((bb & DIAG_MASK_CACHE[2*(piece_idx as usize) + 1]) * GET_DIAG_MASK_MAGIC) >> 56) as u8;
}

