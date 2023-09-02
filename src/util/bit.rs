

pub fn setBit(x:&mut u64, n:u8){
  *x |= (1 << n);
}