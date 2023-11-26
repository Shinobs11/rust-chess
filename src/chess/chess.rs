use crate::chess::*;
use crate::chess::attack_bitmask::*;
use crate::chess::consts::*;
use crate::chess::types::*;




const KIND_NORMAL:u8 = 0;
const KIND_CASTLE:u8 = 1;
const KIND_EN_PESSANT:u8 = 2;





fn make_move(mut board: Board, mov: Move) -> bool{
  


  //First we need to check if the move is legal
  if (mov.piece as u8 % 2) != (board.turn as u8) {
    return false;
  }
  let piece_mask = 1u64 << (63 - mov.arg1);
  let piece_bb = board.bbs[mov.piece];
  if (piece_bb & piece_mask) == 0 {
    return false;
  }

  if mov.kind == KIND_NORMAL {


    let attack_mask = match mov.piece {
      Piece::WKing => king_attack_mask(piece_mask, board.color_masks[board.turn] , board.color_masks[!board.turn]),
      Piece::BKing => king_attack_mask(piece_mask, board.color_masks[board.turn] , board.color_masks[!board.turn]),
      Piece::WQueen => queen_attack_mask(piece_mask, board.color_masks[board.turn] , board.color_masks[!board.turn]),
      Piece::BQueen => queen_attack_mask(piece_mask, board.color_masks[board.turn] , board.color_masks[!board.turn]),
      Piece::WRook => rook_attack_mask(piece_mask, board.color_masks[board.turn] , board.color_masks[!board.turn]),
      Piece::BRook => rook_attack_mask(piece_mask, board.color_masks[board.turn] , board.color_masks[!board.turn]),
      Piece::WBishop => bishop_attack_mask(piece_mask, board.color_masks[board.turn] , board.color_masks[!board.turn]),
      Piece::BBishop => bishop_attack_mask(piece_mask, board.color_masks[board.turn] , board.color_masks[!board.turn]),
      Piece::WKnight => knight_attack_mask(piece_mask, board.color_masks[board.turn]),
      Piece::BKnight => knight_attack_mask(piece_mask, board.color_masks[board.turn]),
      Piece::WPawn => pawn_attack_mask(piece_mask, board.color_masks[board.turn] , board.color_masks[!board.turn], board.turn, board.en_pessant_sq),
      Piece::BPawn => pawn_attack_mask(piece_mask, board.color_masks[board.turn] , board.color_masks[!board.turn], board.turn, board.en_pessant_sq)
      Piece::Empty => unimplemented!()
    };

    let target_mask = 1u64 << (63 - mov.arg2);
    if (target_mask & attack_mask) > 0 {
      //here i'm making the move and then checking if the king would be in check as a result
      //there may be opportunity to optimize here
      board.bbs[mov.piece] = (board.bbs[mov.piece] & !piece_mask) | target_mask;
      board.bbs[mov.piece]
    }


    
    






    return false;
  }
  else if mov.kind == KIND_CASTLE {

    return false;
  }
  else {
    return false;
  }





}






