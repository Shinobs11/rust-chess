use chesslib::chess::check::*;
use chesslib::chess::types::*;
use chesslib::chess::consts::*;
use chesslib::chess::util::*;


//TODOs: figure out a better way to load test data.

#[test]
fn test_is_king_in_check(){
  let mut fen_vec:Vec<String> = vec![];
  retrieve_fens("/home/shino/dev/rust-chess/tests/test_data/10000-checked-kings-unique.fen".to_string(), &mut fen_vec);
  let boards = parse_fens(&fen_vec);
  let mut res: bool = true;
  for (i, board) in boards.iter().enumerate() {
    let x =  is_king_in_check(Color::from(board.turn), &board);
    res &= x > 0;
    if res == false {
      println!("test_is_king_in_check failed for fen: {}", fen_vec[i]);
      break;
    }
  }

  assert_eq!(res, true);

}