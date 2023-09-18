mod util;



pub fn construct_bitmask_from_vec(v: &Vec<u8>){

}


fn main() {
  // let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
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
  
  print_board()
    
}


fn print_board(){
  for x in (0..8){
    for y in (0..8){
      print!("{:3?}", 8*x + y);
    }
    print!("\n");
  }
}