use std::cmp::Ordering;

use chess_game::{types::*, consts::{WHITE_PIECES, BLACK_PIECES}};





#[test]
fn default_board(){
  
  let a = Board::default();
  let b = Board::board_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string());
  let sq_res = a.sq.cmp(&b.sq);
  assert_eq!(sq_res, Ordering::Equal);

  let a_ps = a.piece_set;
  let b_ps = b.piece_set;
  let pieces = vec![WHITE_PIECES, BLACK_PIECES];
  let flat_pieces = pieces.iter().flatten();
  let ps_res = flat_pieces.fold(true, |acc, x| acc && (a_ps[*x].eq(&b_ps[*x])));
  assert!(ps_res);

  assert_eq!(a.turn, b.turn);

  assert_eq!(a.castle_state, b.castle_state);

  assert_eq!(a.en_pessant_sq, b.en_pessant_sq);


  
}
