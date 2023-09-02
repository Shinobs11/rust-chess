mod util;


use crate::util::chess::*;
fn main() {
  let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
  let test = FENToBitBoard(s.to_string());
  

}
