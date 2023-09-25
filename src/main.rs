mod util;

use std::arch::x86_64::_popcnt64;

use util::chess::*;

use crate::util::types::Board;

pub fn construct_bitmask_from_vec(v: &Vec<u8>){

}

pub type BitBoard = u64;
fn main() {
  let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
  // let test = FENToBitBoard(s.to_string());
  // println!("{}", printBitBoard(test.w_pawn, false));
  

  

  // let mut bb = FullBitBoard{..Default::default()};
  
  // println!("{}", printBitBoard(bb.empty as u64, false));
  // setBit(&mut bb.empty, 63);
  // println!("{}", printBitBoard(bb.empty as u64, false));

  
  // let king_offsets = (
  //   (1, 1), (1, -1), (-1, 1), (-1, 1), (1, 0), (0, 1), (-1, 0), (0, -1)
  // );
  // let mut rook_offsets_x: &mut Vec<(i32, i32)> = Vec::from_iter((-7..=7).map(|x:i32| (x as i32, 0 as i32))).as_mut();
  // let mut rook_offsets_y: &mut Vec<(i32, i32)> = Vec::from_iter((-7..=7).map(|x:i32|(0 as i32, x as i32))).as_mut();
  // rook_offsets_x.append(rook_offsets_y);
  // let rook_offsets = rook_offsets_x.clone();
  // let mut bishop_offsets_pos: &mut Vec<(i32, i32)> = Vec::from_iter((-7..=7).map(|x|(x, x))).as_mut();
  // let mut bishop_offsets_neg: &mut Vec<(i32, i32)> = Vec::from_iter((-7..=7).map(|x|(x, -x))).as_mut();
  // bishop_offsets_pos.append(bishop_offsets_neg);
  // let bishop_offsets = bishop_offsets_pos.clone();
  // let queen_offsets = Vec::new().append(rook_offsets.clone());

  // println!("{:?}", rook_offsets.0);




  // let b = Board::default();
  
  // let bb = Board::board_from_fen(s.to_string());
  

  // let arr = [0; 64];
  // println!("{}", b.to_string());
  // println!("{}", bb.to_string());

  fn branchless_white_rook(rook_mask: u64, white_mask: u64, black_mask: u64)->u64{
    const FILE_DELTAS:&'static[i32; 2] = &[-8, 8];
    const RANK_DELTAS:&'static[i32; 2] = &[-1, 1];
    let rook_idx = rook_mask.leading_zeros();
    let file_bounds:(i32, i32) = (0, 63);
    let rank_bounds:(i32, i32) = ((rook_idx & !7) as i32, (rook_idx | 7) as i32);
    let mut result:u64 = 0;
    
    for delta in FILE_DELTAS.iter(){
      //potential optimization, I might be able to avoid using an 8 element array here, since I only need to consider the immediately previous result.
      let mut offset_results:[u64; 8] = [rook_mask, 0, 0, 0, 0, 0, 0, 0];
      let mut attack_toggle:u64 = 0;
      for i in (1..8){
        let idx = (rook_idx as i32 + i*delta);
        let in_bounds = (idx >= file_bounds.0 &&  idx <= file_bounds.1) as u64 * u64::MAX;
        offset_results[i as usize] = (in_bounds) & (rook_mask << (*delta + i as i32 * *delta)) & !white_mask & offset_results[(i-1) as usize] & !attack_toggle;
        attack_toggle = ((offset_results[i as usize] & black_mask) > 0) as u64 * u64::MAX;  
      }
      for i in (1..8) {
        result |= offset_results[i];
      }
    }
    for delta in RANK_DELTAS.iter(){
      //potential optimization, I might be able to avoid using an 8 element array here, since I only need to consider the immediately previous result.
      let mut offset_results:[u64; 8] = [rook_mask, 0, 0, 0, 0, 0, 0, 0];
      let mut attack_toggle:u64 = 0;
      for i in (1..8){
        let idx = (rook_idx as i32 + i*delta);
        let in_bounds = (idx >= rank_bounds.0 &&  idx <= rank_bounds.1) as u64 * u64::MAX;
        println!("bounds: {}", in_bounds);
        offset_results[i as usize] = (in_bounds) & (rook_mask << (*delta + i as i32 * *delta)) & !white_mask & offset_results[(i-1) as usize] & !attack_toggle;
        attack_toggle = ((offset_results[i as usize] & black_mask) > 0) as u64 * u64::MAX;  
      }
      println!("bleh: {:?}", offset_results);
      for i in (1..8) {
        result |= offset_results[i];
      }
    }
    return result;
  }




  let rook_mask:BitBoard =  0b0000000000000000000000000000000000000000000000000100000000000000;
  // let rook_mask:BitBoard = 0b0000000000000010000000000000000000000000000000000000000000000000;
  let white_mask:BitBoard =  0b0000000000000000000000000000000000000000000000001111111111111111;
  let black_mask:BitBoard = 0b0100000101000001000000000000000000000000000000000000000000000000;
  


  println!("{}", rook_mask << -8);
  

  // print_board(rook_mask);
  // println!("white_mask");
  // print_board(white_mask);
  // println!("black_mask");
  // print_board(black_mask);
  // println!("res");



  // let rook_idx:u8 = rook_mask.leading_zeros() as u8;
  // println!("idx: {}", rook_idx.view_bits::<Msb0>());
  // let lb = (rook_idx & !7);
  // println!("lb: {}", lb.view_bits::<Msb0>());
  // println!("lb: {}", lb);
  // let ub = (rook_idx | 7);
  // println!("ub: {}", ub);






  // let mut toggle:u64 = 0;
  // let mut res:u64 = 0;
  // let a = (rook_mask << 8) & !white_mask;
  // res |= a;
  // toggle = ((res & black_mask) > 0 ) as u64 * u64::MAX;
  // let b = (rook_mask << 16) & !white_mask & a << 8 & !toggle as u64;
  // res |= b;
  // toggle = ((res & black_mask) > 0 ) as u64 * u64::MAX;
  // let c = (rook_mask << 24) & !white_mask & b << 8 & !toggle as u64;
  // res |= c;
  // toggle = ((res & black_mask) > 0 ) as u64 * u64::MAX;
  // let d = (rook_mask << 32) & !white_mask & c << 8 & !toggle as u64;
  // res |= d;
  // toggle = ((res & black_mask) > 0 ) as u64 * u64::MAX;
  // let e = (rook_mask << 40) & !white_mask & d << 8 & !toggle as u64;
  // res |= e;
  // toggle = ((res & black_mask) > 0 ) as u64 * u64::MAX;
  // let f = (rook_mask << 48) & !white_mask & e << 8 & !toggle as u64;
  // res |= f;
  // toggle = ((res & black_mask) > 0 ) as u64 * u64::MAX;


  // let mut res:u64 = 0;
  // let mut shift:u8 = 8;
  // let rook_init_pos = rook_mask.view_bits::<Msb0>().first_one().unwrap();
  // while true {
  //   let rook_shift = (rook_mask << shift);
  //   res |= (rook_shift & !white_mask);
  //   if (res & black_mask) > 0 {
  //     break;
  //   }
  //   shift+=8;
  // }

  let res = branchless_white_rook(rook_mask, white_mask, black_mask);



  print_board(res);



    
}

use bitvec::{prelude::*, view::BitView};

fn print_board(b:BitBoard){
  let slice = b.view_bits::<Msb0>();

  for x in (0..8){
    for y in (0..8){
      print!("{:3?}", slice[x*8 + y] as u8);
    }
    print!("\n");
  }
}