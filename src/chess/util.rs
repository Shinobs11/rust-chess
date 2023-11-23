use bitvec::{prelude::*, view::BitView};
pub fn print_bb(b:u64){
  let slice = b.view_bits::<Msb0>();
  for x in (0..8){
    for y in (0..8){
      print!("{:3?}", slice[x*8 + y] as u8);
    }
    print!("\n");
  }
}