
use num_enum::FromPrimitive;

use crate::cache::KNIGHT_CACHE;
use crate::cache::RAY_CACHE;
use crate::chess::util::*;
use super::consts::*;
use super::types::*;
use super::bit::*;

/// Returns a bitboard representing pieces that can legally attack the king.
pub fn is_king_in_check(color: Color, board:&Board) -> u64 {

  let king_bb = board.bbs[Piece::from_primitive(color as u8)];
  
  //here we make the bold assumption that there is one king per side.
  let king_idx = king_bb.leading_zeros() as u8;
  let king_col_idx = (king_idx % 8) as u8;
  let king_row_idx = (king_idx / 8) as u8;


  let f_mask = board.color_masks[color];
  let o_mask = board.color_masks[!color];
  
  //get ray of king for col, diags and row using king idx
  //& those rays with the bitboards of bishops, rooks and queens
  //if the king was a queen instead, and the queen could attack those enemy pieces, then
  //the opposite would be true as well.

  let f_col_mask = get_col_mask(f_mask, king_col_idx);
  let o_col_mask = get_col_mask(o_mask, king_col_idx);

  let f_row_mask = get_row_mask(f_mask, king_row_idx);
  let o_row_mask = get_row_mask(o_mask, king_row_idx);


  let pos_diag_mask = get_pos_diag_mask(king_idx);
  let neg_diag_mask = get_neg_diag_mask(king_idx);

  let f_pos_diag_mask = (((f_mask & pos_diag_mask) * GET_DIAG_MASK_MAGIC) >> 56) as u8;
  let o_pos_diag_mask = (((o_mask & pos_diag_mask) * GET_DIAG_MASK_MAGIC) >> 56) as u8;
  let f_neg_diag_mask = (((f_mask & neg_diag_mask) * GET_DIAG_MASK_MAGIC) >> 56) as u8;
  let o_neg_diag_mask = (((o_mask & neg_diag_mask) * GET_DIAG_MASK_MAGIC) >> 56) as u8;

  let col_ternary_mask = get_ternary_bitmask(king_row_idx, f_col_mask, o_col_mask);
  let row_ternary_mask = get_ternary_bitmask(king_col_idx, f_row_mask, o_row_mask);
  let pos_diag_ternary_mask = get_ternary_bitmask(king_col_idx, f_pos_diag_mask, o_pos_diag_mask);
  let neg_diag_ternary_mask = get_ternary_bitmask(king_col_idx, f_neg_diag_mask, o_neg_diag_mask);

  let col_ray_mask = RAY_CACHE[col_ternary_mask as usize];
  let row_ray_mask = RAY_CACHE[row_ternary_mask as usize];
  let pos_diag_ray_mask = RAY_CACHE[pos_diag_ternary_mask as usize];
  let neg_diag_ray_mask = RAY_CACHE[neg_diag_ternary_mask as usize];

  let col_attack_mask = put_col_mask(col_ray_mask, king_col_idx);
  let row_attack_mask = put_row_mask(row_ray_mask, king_row_idx);
  let pos_diag_attack_mask = (pos_diag_ray_mask as u64 * GET_DIAG_MASK_MAGIC) & pos_diag_mask;
  let neg_diag_attack_mask = (neg_diag_ray_mask as u64 * GET_DIAG_MASK_MAGIC) & neg_diag_mask;

  //now that we have the appropriate ray masks, we can start checking opposing bishops, rooks and queens to see
  //if any of them can attack the king.
  let bishop_attack_mask = pos_diag_attack_mask | neg_diag_attack_mask;
  let rook_attack_mask = col_attack_mask | row_attack_mask;
  let queen_attack_mask = bishop_attack_mask | rook_attack_mask;

  let o_bishop_bb = board.bbs[6 + (!color as u8)];
  let o_rook_bb = board.bbs[4 + (!color as u8)];
  let o_queen_bb = board.bbs[2 + (!color as u8)];

  let can_bishop_attack = bishop_attack_mask & o_bishop_bb;
  let can_rook_attack = rook_attack_mask & o_rook_bb;
  let can_queen_attack = queen_attack_mask & o_queen_bb;


  let mut pawn_attack_mask:u64;
  if color == Color::White {
    pawn_attack_mask = (king_bb & 0xfefefefefefefefe) << 7; //TODOs: look into if this can be cut down a bit. 
    pawn_attack_mask |= (king_bb & 0x7f7f7f7f7f7f7f7f) << 9;
  }
  else {
    pawn_attack_mask = (king_bb & 0x7f7f7f7f7f7f7f7f) >> 7;
    pawn_attack_mask |= (king_bb & 0xfefefefefefefefe)  >> 9;   
  }

  let o_pawn_bb = board.bbs[10 + (!color as u8)];
  let can_pawn_attack = pawn_attack_mask & o_pawn_bb;

  
  let o_knight_bb = board.bbs[8 + (!color as u8)];
  let can_knight_attack = KNIGHT_CACHE[king_idx as usize] & o_knight_bb;

  


  return can_pawn_attack | can_knight_attack | can_bishop_attack | can_rook_attack | can_queen_attack;

}