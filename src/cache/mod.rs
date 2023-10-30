

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
    let rook_mask = (1u8 << (7 - p_idx));
    let mut comb = 0;
    while comb < u16::MAX{
      let friends_mask = ((comb & 0x00FF)  as u8); //least significant 8 bits
      let foes_mask = (comb >> 8) as u8; //most significant 8 bits
      if (friends_mask & foes_mask) != 0 {
        comb += 1;
        continue;
      }
      let mut prev = rook_mask;
      let mut attack_toggle:u8 = 0; 
      let mut res:u8 = 0;
      let mut i = 1;
      //positive dir
      let mut range_max = 7 - p_idx; //2
      let mut has_been_blocked: u8;
      while i <= range_max {
        has_been_blocked = !(prev == 0) as u8 * u8::MAX;
        prev = has_been_blocked & (rook_mask >> i) & !friends_mask & !attack_toggle;
        if prev == 0 {
          break;
        }
        attack_toggle = ((prev & foes_mask) > 0) as u8 * u8::MAX;
        res |= prev;
        i += 1;
      }
      range_max = p_idx;
      i = 1;
      prev = rook_mask;
      attack_toggle = 0;
      while i <= range_max {
        has_been_blocked = !(prev == 0) as u8 * u8::MAX;
        prev = has_been_blocked & (rook_mask << i) & !friends_mask & !attack_toggle;
        if prev == 0 {
          break;
        }
        attack_toggle = ((prev & foes_mask) > 0) as u8 * u8::MAX;
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
  let mut i:u8 = 0;
  let offset = [-9, -8, -7, -1, 1, 7, 8, 9];
  while i<64 {
    let mut j = 0;
    while j < 8 {
      let target_pos:i8 = i as i8 + offset[j];
      let col_diff:i8 = ((i % 8) as i8) - (target_pos % 8) as i8;
      if target_pos < 64 && target_pos >= 0 && (col_diff.abs() == 1){
        cache[i as usize] |= 1 << (63 - target_pos);
      }
      j+=1;
    }
    i+=1;
  }
  return cache;
}

const fn knight_cache()->[u64; 64]{
  const KNIGHT_OFFSETS_1:[i32; 4] = [-17, -15, 15, 17];
  const KNIGHT_OFFSETS_2:[i32; 4] = [-10, -6, 6, 10];
  let mut out:[u64; 64] = [0; 64];
  let mut idx = 0;
  while idx < 64 {
    let mut res = 0;
    let mut target_pos:i32 = 0;
    let mut d:i32 = i32::MIN;
    let mut d_idx = 0;
    while d_idx < 4 {
      d = KNIGHT_OFFSETS_1[d_idx];
      target_pos = idx + d;
      if (target_pos >= 0 && target_pos <=63) && (target_pos/8 - idx/8 ) == (2 * d.signum()){
        res |= if (d.is_positive()) {left_shift((1u64 << (63 - idx)) as u64, d as u8)} else {right_shift((1u64 << (63 - idx)) as u64, d.abs() as u8)};
      }
      d_idx += 1;
    }
    d_idx = 0;
    while d_idx < 4 {
      d = KNIGHT_OFFSETS_2[d_idx];
      target_pos = idx + d;
      if (target_pos >= 0 && target_pos <=63) && (target_pos/8 - idx/8) == (d.signum()){
        res |= if (d.is_positive()) {left_shift((1u64 << (63 - idx)) as u64, d as u8)} else {right_shift((1u64 << (63 - idx)) as u64, d.abs() as u8)};
      }
      d_idx += 1;
    }
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
    let mut res = (1 << (63 - i));
    let p_idx = (1 << (63 - i));
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
