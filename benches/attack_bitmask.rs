use criterion::{black_box, criterion_group, criterion_main, Criterion};
use num_enum::FromPrimitive;
use chesslib::chess::consts::*;
use chesslib::chess::types::*;
use chesslib::chess::attack_bitmask::*;
use chesslib::chess::chess::*;
use chesslib::chess::util::*;
pub type BitBoard = u64;

pub fn batch_pawn_attack_mask(v: &Vec<(u64, u64, u64, Color, u8)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask, color, ep_square) in v {
    res.push(pawn_attack_mask(*piece_mask, *friend_mask, *foe_mask, *color, *ep_square));
  }
  return res;
}

pub fn batch_knight_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(knight_attack_mask(*piece_mask, *friend_mask));
  }
  return res;
}


pub fn batch_bishop_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(bishop_attack_mask(*piece_mask, *friend_mask, *foe_mask));
  }
  return res;
}

pub fn batch_rook_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(rook_attack_mask(*piece_mask, *friend_mask, *foe_mask));
  }
  return res;
}

pub fn batch_queen_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(queen_attack_mask(*piece_mask, *friend_mask, *foe_mask));
  }
  return res;
}

pub fn batch_king_attack_mask(v: &Vec<(u64, u64, u64)>)->Vec<u64>{
  let mut res:Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for (piece_mask, friend_mask, foe_mask) in v {
    res.push(king_attack_mask(*piece_mask, *friend_mask, *foe_mask));
  }
  return res;
}


pub fn batch_all_attack_mask(v: &Vec<(u64, u64, u64, GenericPiece)>)-> Vec<u64> {
  let mut res: Vec<u64> = Vec::<u64>::with_capacity(v.len());
  for x in v {
    let piece_mask = x.0;
    let friend_mask = x.1;
    let foe_mask = x.2;
    let piece_type = x.3;

    let r = match piece_type {
      GenericPiece::King => 0,
      GenericPiece::Queen => queen_attack_mask(piece_mask, friend_mask, foe_mask),
      GenericPiece::Rook => rook_attack_mask(piece_mask, friend_mask, foe_mask),
      GenericPiece::Bishop => bishop_attack_mask(piece_mask, friend_mask, foe_mask),
      GenericPiece::Knight => knight_attack_mask(piece_mask, friend_mask),
      GenericPiece::Pawn => 0,
      GenericPiece::Empty => 0
    };
    res.push(r);
  }
  return res;

}

fn _benchmark(c: &mut Criterion) {
  
  fn get_position_masks(path: &str, target_piece: GenericPiece)->Vec<(u64, u64, u64)> {
    let mut fen_vec: Vec<String> = vec![];
    retrieve_fens(path.to_string(), &mut fen_vec);
    let boards = parse_fens(&fen_vec);

    let mut masks:Vec<(u64, u64, u64)> = Vec::<(u64, u64, u64)>::with_capacity(boards.len());
    for (i, b) in (&boards).iter().enumerate() {

      let specific_piece = target_piece.to_color(b.turn);
      let piece_mask:u64 = 1 << (63 - b.bbs[specific_piece].leading_zeros());
      let friend_mask:u64 = b.color_masks[b.turn];
      let opp_mask:u64 = b.color_masks[!b.turn];

      

      masks.push((piece_mask, friend_mask, opp_mask));
    }
    return masks;
  }
  let knight_masks = get_position_masks("/home/shino/chess-datasets/1000-N-positions.fen", GenericPiece::Knight);
  let bishop_masks = get_position_masks("/home/shino/chess-datasets/1000-B-positions.fen", GenericPiece::Bishop);
  let rook_masks = get_position_masks("/home/shino/chess-datasets/1000-R-positions.fen", GenericPiece::Rook);
  let queen_masks = get_position_masks("/home/shino/chess-datasets/1000-Q-positions.fen", GenericPiece::Queen);
  
  let mut combined_masks = Vec::<(u64, u64, u64, GenericPiece)>::new();
  
  let mut knight_masks_copy = Vec::<(u64, u64, u64)>::new();
  knight_masks_copy.clone_from(&knight_masks);
  let mut bishop_masks_copy = Vec::<(u64, u64, u64)>::new();
  bishop_masks_copy.clone_from(&bishop_masks);
  let mut rook_masks_copy = Vec::<(u64, u64, u64)>::new();
  rook_masks_copy.clone_from(&rook_masks);
  let mut queen_masks_copy = Vec::<(u64, u64, u64)>::new();
  queen_masks_copy.clone_from(&queen_masks);


  
  let mut rng = rand::thread_rng();
  let mut  n = 0;
  for _ in 0..(bishop_masks_copy.len() + knight_masks_copy.len() + rook_masks_copy.len() + queen_masks_copy.len()) {
    let n_piece = GenericPiece::from_primitive(n);
    match n_piece {
      GenericPiece::Knight => {
        if knight_masks_copy.is_empty() {
          continue;
        }
        else {
          let x = knight_masks_copy.pop().unwrap();
          combined_masks.push((x.0, x.1, x.2, GenericPiece::Knight));
        }
      },
      GenericPiece::Bishop => {
        if bishop_masks_copy.is_empty() {
          continue;
        }
        else {
          let x = bishop_masks_copy.pop().unwrap();
          combined_masks.push((x.0, x.1, x.2, GenericPiece::Bishop));
        }
      },
      GenericPiece::Rook => {
        if rook_masks_copy.is_empty() {
          continue;
        }
        else {
          let x = rook_masks_copy.pop().unwrap();
          combined_masks.push((x.0, x.1, x.2, GenericPiece::Rook));
        }
      },
      GenericPiece::Queen => {
        if queen_masks_copy.is_empty() {
          continue;
        }
        else {
          let x = queen_masks_copy.pop().unwrap();
          combined_masks.push((x.0, x.1, x.2, GenericPiece::Queen));
        }
      },
      GenericPiece::King => (),
      GenericPiece::Pawn => (),
      GenericPiece::Empty => ()
    }
    n += 1;
    if n >= 4 {
      n = 0;
    }
  }

  c.bench_function("batch_knight_attack_mask", |b| b.iter(|| batch_knight_attack_mask(&knight_masks)));
  c.bench_function("batch_bishop_attack_mask", |b| b.iter(|| batch_bishop_attack_mask(&bishop_masks)));
  c.bench_function("batch_rook_attack_mask", |b| b.iter(|| batch_rook_attack_mask(&rook_masks)));
  c.bench_function("batch_queen_attack_mask", |b| b.iter(|| batch_queen_attack_mask(&queen_masks)));
  c.bench_function("batch_all_attack_mask", |b| b.iter(|| batch_all_attack_mask(&combined_masks)));
}



criterion_group!(attack_bitmask_benchmark, _benchmark);
