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
pub const TERNARY_CACHE:[u16; 256] = ternary_cache();

const fn rook_cache()->[u8; 65536]{
  let mut cache:[u8; 65536] = [0; 65536];
  let mut r_idx = 0;
  while r_idx < 8 {
    let rook_mask = (1u8 << (7 - r_idx));
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
      let mut range_max = 7 - r_idx; //2
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
      range_max = r_idx;
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
      let res_idx = (ter_friend_mask + ter_foe_mask) | (r_idx << 13);

      cache[res_idx as usize] = res;
      comb += 1;
    }
    r_idx += 1;
  }
  return cache;
}





pub const ROOK_CACHE:[u8; 65536] = rook_cache();
