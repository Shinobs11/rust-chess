
use num_enum::FromPrimitive;

use crate::util::types::*;
use crate::util::consts::*;

/*
Given a hypothetical piece and a board state, can said piece attack the given position from the initial position.
Does not consider checks.
*/
fn in_attack_range(b: &mut Board, p:Piece, p0: u8, pf: u8)->bool{
  
  if p0 > 63 || pf > 63 {
    return false;
  }

  let gen_p:GenericPiece = GenericPiece::from_primitive(p as u8/2);
  if (b.sq[pf as usize] % 2) == (p as u8 % 2) {
    return false;
  }
  
  
  match gen_p {
    GenericPiece::King => {
      let mut in_range:bool = false;
      //by using abs we can cut the numbers to check by half so long as p0 and pf are in bounds.
      let diff = (pf as i8 - p0 as i8).abs();
      for x in [9, 8, 7, 1]{
        in_range = diff == x || in_range;
      }
      return in_range;
    }
    GenericPiece::Queen => {
      let diff:i8 = pf as i8 - p0 as i8;
      let mut in_range_all = false;
      for modulo in [9, 8, 7, 1] {
        let mut in_range:bool = true;
        if diff % modulo == 0 {
          let div = diff / modulo;
          for i in (1..(div.signum()*div)){
            in_range = (b.sq[(p0 + (i*modulo*div.signum()) as u8) as usize] == Piece::Empty as u8) && in_range;
          }
        }
        in_range_all = in_range_all || in_range;
      }
    }
    GenericPiece::Rook => {
      let diff:i8 = pf as i8 - p0 as i8;
      let mut in_range_all = false;
      for modulo in [8, 1] {
        let mut in_range:bool = true;
        if diff % modulo == 0 {
          let div = diff / modulo;
          for i in (1..(div.signum()*div)){
            in_range = (b.sq[(p0 + (i*modulo*div.signum()) as u8) as usize] == Piece::Empty as u8) && in_range;
          }
        }
        in_range_all = in_range_all || in_range;
      }
    }
    GenericPiece::Bishop => {
      let diff:i8 = pf as i8 - p0 as i8;
      let mut in_range_all = false;
      for modulo in [9, 7] {
        let mut in_range:bool = true;
        if diff % modulo == 0 {
          let div = diff / modulo;
          for i in (1..(div.signum()*div)){
            in_range = (b.sq[(p0 + (i*modulo*div.signum()) as u8) as usize] == Piece::Empty as u8) && in_range;
          }
        }
        in_range_all = in_range_all || in_range;
      }
    }
    GenericPiece::Knight => {
      let mut in_range:bool = false;
      //by using abs we can cut the numbers to check by half so long as p0 and pf are in bounds.
      let diff = (pf as i8 - p0 as i8).abs();
      for x in [17, 15, 10, 6]{
        in_range = diff == x || in_range;
      }
      return in_range;
    }
    GenericPiece::Pawn => {
      

      if b.en_pessant_sq < 63{
        if pf == b.en_pessant_sq{

        }
      }
    }
    GenericPiece::Empty => {
      return false;
    }
    _ => {
      return false;
    }
  }
  





  return false;




}