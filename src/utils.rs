pub fn print_board(board: sudoku::BoardArray) {
  let mut col = 0;
  for digit in board.iter() {
    if col == 8 {
      println!("{} ", digit);
    } else {
      print!("{} ", digit);
    }
    col += 1;
    col %= 9;
  }
}
