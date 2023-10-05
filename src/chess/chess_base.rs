
use num_enum::FromPrimitive;

use crate::chess::types::*;
use crate::chess::consts::*;

/*
Given a hypothetical piece and a board state, can said piece attack/move to the given position from the initial position.
Does not consider checks.
*/
fn in_range(b: &mut Board, p:Piece, p0: u8, pf: u8)->bool{
  
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
    GenericPiece::Pawn => { //TODOS: refactor with less branches when brain is not mushy paste.
      let color = p as u8 % 2;
      let diff = pf as i8 - p0 as i8;
      if color == 1 {
        if diff.signum() != 1 {
          return false;
        }
        if diff == 8 && (b.sq[(p0+8) as usize]) == Piece::Empty as u8 {
          return true;
        }
        if diff == 16 && p0 >= 8 && p0 < 16 && (b.sq[(p0+8) as usize]) == Piece::Empty as u8 && (b.sq[(p0+16) as usize] == Piece::Empty as u8) {
          return true;
        }
        if (diff == 15||diff == 17){
          if b.sq[pf as usize] != Piece::Empty as u8 {
            return true;
          }
          else if b.sq[(pf - 8) as usize] == Piece::BPawn as u8 && b.en_pessant_sq == pf {
            return true;
          }
          else {
            return false;
          }
        }
      }
      else{
        if diff.signum() != -1 {
          return false;
        }
        if diff == -8 && (b.sq[(p0-8) as usize]) == Piece::Empty as u8 {
          return true;
        }
        if diff == -16 && p0 < 56 && p0 >= 48 && (b.sq[(p0-8) as usize] == Piece::Empty as u8) && (b.sq[(p0-16) as usize] == Piece::Empty as u8) {
          return true;
        }
        if (diff == -15||diff == -17){
          if b.sq[pf as usize] != Piece::Empty as u8 {
            return true;
          }
          else if b.sq[(pf + 8) as usize] == Piece::WPawn as u8 && b.en_pessant_sq == pf {
            return true;
          }
          else {
            return false;
          }
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