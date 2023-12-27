

pub const TERNARY_CACHE:[u16; 256] = ternary_cache();
pub const RAY_CACHE:[u8; 65536] = ray_cache();
pub const KING_CACHE:[u64; 64] = king_cache();
pub const KNIGHT_CACHE:[u64; 64] = knight_cache();
pub const DIAG_MASK_CACHE:[u64; 128] = diagonal_mask_cache();

const fn ternary_cache()->[u16; 256]{
  let mut cache:[u16; 256] = [0; 256];
  let mut i:u16 = 0;
  while i <= 255 {
    let mut j:u8 = 0;
    let mut res:u16 = 0;
    while j <= 7 {
      res += (((1 << j) & i) >> j) as u16 * (u16::pow(3, j as u32));
      j+=1;
    }
    cache[i as usize] = res;
    i+=1;
  }
  return cache;
}


const fn ray_cache()->[u8; 65536]{
  let mut cache:[u8; 65536] = [0; 65536];
  let mut p_idx = 0;
  while p_idx < 8 {
    let piece_mask = (1u8 << (7 - p_idx));
    let mut comb = 0;
    while comb < u16::MAX{
      let friends_mask = ((comb & 0x00FF)  as u8); //least significant 8 bits
      let foes_mask = (comb >> 8) as u8; //most significant 8 bits
      if (friends_mask & foes_mask) != 0 {
        comb += 1;
        continue;
      }
      let mut prev = piece_mask;
      let mut has_captured:u8 = 0; 
      let mut res:u8 = 0;
      let mut i = 1;
      //positive dir
      let mut range_max = 7 - p_idx; //2
      let mut has_not_been_blocked: u8;
      while i <= range_max {
        has_not_been_blocked = !(prev == 0) as u8 * u8::MAX;
        prev = has_not_been_blocked & (piece_mask >> i) & !friends_mask & !has_captured;
        if prev == 0 {
          break;
        }
        has_captured = ((prev & foes_mask) > 0) as u8 * u8::MAX;
        res |= prev;
        i += 1;
      }
      range_max = p_idx;
      i = 1;
      prev = piece_mask;
      has_captured = 0;
      while i <= range_max {
        has_not_been_blocked = !(prev == 0) as u8 * u8::MAX;
        prev = has_not_been_blocked & (piece_mask << i) & !friends_mask & !has_captured;
        if prev == 0 {
          break;
        }
        has_captured = ((prev & foes_mask) > 0) as u8 * u8::MAX;
        res |= prev;
        i += 1;
      }
      
      let ter_friend_mask = TERNARY_CACHE[friends_mask as usize];
      let ter_foe_mask = 2*TERNARY_CACHE[foes_mask as usize];
      let res_idx = (ter_friend_mask + ter_foe_mask) | (p_idx << 13);

      cache[res_idx as usize] = res;
      comb += 1;
    }
    p_idx += 1;
  }
  return cache;
}

const fn king_cache()->[u64; 64]{
  let mut cache:[u64; 64] = [0; 64];
  let mut king_idx:u8 = 0;

  while king_idx<64 {
    let king_bitmask = (1u64 << (63 - king_idx));
    let left = ((king_bitmask & 0x7f7f7f7f7f7f7f7f) << 1);
    let upper_left = left << 8;
    let lower_left = left >> 8; 
    let right = ((king_bitmask & 0xfefefefefefefefe) >> 1);
    let upper_right = right << 8;
    let lower_right = right >> 8;
    let top = king_bitmask << 8;
    let bottom = king_bitmask >> 8;
    cache[king_idx as usize] = left | upper_left | lower_left | right | upper_right | lower_right | top | bottom;
    king_idx+=1;
  }
  return cache;
}

const fn knight_cache()->[u64; 64]{
  let mut out:[u64; 64] = [0; 64];
  let mut idx = 0;
  while idx < 64 {
    let mut res = 0;
    let l1:u64 = ((1 << (63 - idx)) >> 1) & 0x7f7f7f7f7f7f7f7f;
    let l2:u64  = ((1 << (63 - idx)) >> 2) & 0x3f3f3f3f3f3f3f3f;
    let r1: u64 = ((1 << (63 - idx)) << 1) & 0xfefefefefefefefe;
    let r2: u64 = ((1 << (63 - idx)) << 2) & 0xfcfcfcfcfcfcfcfc;
    let h1 = l1 | r1;
    let h2 = l2 | r2;
    res = (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8);
    out[idx as usize] = res;
    idx+=1;
  }
  return out;

}
//111
//113
const fn diagonal_mask_cache()->[u64; 128]{
  let mut cache: [u64; 128] = [0; 128];
  let mut i = 0;
  while i < 64 {
    let mut range_x:[i32; 2] = [i % 8, 7 - (i % 8)];
    let mut range_y:[i32; 2] = [i / 8, 7 - (i/8)];
    let mut offsets: [i32; 2] = [-9,  9];
    let mut j = 0;
    let mut res = 1 << (63 - i);
    let p_idx = 1 << (63 - i);
    while j < 2 {
      let range_max = if range_x[j] < range_y[j] {range_x[j]} else {range_y[j]};
      let mut k = 1;

      while k <= range_max {
        if offsets[j].is_negative(){
          res |= p_idx << (k*offsets[j].abs());
        }
        else {
          res |= p_idx >> (k*offsets[j]);
        }
        k+=1;
      }
      j+=1;
    }
    cache[(i*2) as usize] = res;
    j = 0;
    res = (1 << (63 - i));
    range_x = [7 - (i % 8), (i % 8)];
    range_y = [i / 8, 7 - (i / 8)];
    offsets = [-7, 7];
    while j < 2 {
      let range_max = if range_x[j] < range_y[j] {range_x[j]} else {range_y[j]};
      let mut k = 1;
      while k <= range_max {
        if offsets[j].is_negative(){
          res |= (p_idx << (k*offsets[j].abs()));
        }
        else {
          res |= (p_idx >> (k*offsets[j]));
        }
        k+=1;
      }
      j+=1;
    }
    cache[(i*2+1) as usize] = res;
    i+=1;
  }
  return cache;
}

#[inline]
const fn left_shift(v:u64, n:u8)->u64{
  return v << n;
}
#[inline]
const fn right_shift(v:u64, n:u8)->u64{
  return v >> n;
}
