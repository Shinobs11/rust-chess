pub type BitBoard = u64;
use bitvec::{prelude::*, view::BitView};
fn get_bit_idx(n:u64)->usize{
  return n.view_bits::<Msb0>().first_one().unwrap();
}