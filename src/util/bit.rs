

pub fn setBit(x:&mut u64, n:u8){
  *x |= (1 << n);
}

pub fn pair_into_bit_offset(p:(i32, i32))->i32 {
  return p.0 + p.1*8;
}