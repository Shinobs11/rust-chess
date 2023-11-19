
use num_enum::FromPrimitive;

use crate::cache::RAY_CACHE;

use super::consts::*;
use super::types::*;
use super::bit::*;

pub fn is_king_in_check(color: Color, board:Board) {
  let king_bb = board.bbs[Piece::from_primitive(color as u8)];
  
  //here we make the bold assumption that there is one king per side.
  let king_idx = king_bb.leading_zeros() as u8;
  let king_col_idx = (king_idx % 8) as u8;
  let king_row_idx = (king_idx / 8) as u8;


  let same_mask = board.color_masks[color];
  let opp_mask = board.color_masks[!color];
  
  //get ray of king for col, diags and row using king idx
  //& those rays with the bitboards of bishops, rooks and queens
  //if the king was a queen instead, and the queen could attack those enemy pieces, then
  //the opposite would be true as well.

  let same_mask_col = get_col_mask(same_mask, king_col_idx);
  let opp_mask_col = get_col_mask(opp_mask, king_col_idx);

  let same_mask_row = get_row_mask(same_mask, king_row_idx);
  let opp_mask_row = get_row_mask(opp_mask, king_row_idx);

  let same_mask_pos_diag = get_pos_diag_mask(same_mask, king_idx);
  let opp_mask_pos_diag = get_pos_diag_mask(opp_mask, king_idx);

  let same_mask_neg_diag = get_neg_diag_mask(same_mask, king_idx);
  let opp_mask_neg_diag = get_neg_diag_mask(opp_mask, king_idx);

  let col_ternary_mask = get_ternary_bitmask(king_idx, same_mask_col, opp_mask_col);
  let row_ternary_mask = get_ternary_bitmask(king_idx, same_mask_row, opp_mask_col);
  let pos_diag_ternary_mask = get_ternary_bitmask(king_idx, same_mask_pos_diag, opp_mask_pos_diag);
  let neg_diag_ternary_mask = get_ternary_bitmask(king_idx, same_mask_neg_diag, opp_mask_neg_diag);

  let col_ray_mask = RAY_CACHE[col_ternary_mask as usize];
  let row_ray_mask = RAY_CACHE[row_ternary_mask as usize];
  let pos_diag_ray_mask = RAY_CACHE[pos_diag_ternary_mask as usize];
  let neg_diag_ray_mask = RAY_CACHE[neg_diag_ternary_mask as usize];

  



  


}