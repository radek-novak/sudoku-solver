pub type BoardArray = [u8; 81];

pub struct Board {
  pub values: BoardArray,
}

pub fn print_board(board: BoardArray) {
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

pub fn get_row(input: &BoardArray, row_idx: usize) -> &[u8] {
  let start = row_idx * 9;
  let end = start + 9;
  return &input[start..end];
}

pub fn get_column(input: &BoardArray, col_i: usize) -> [u8; 9] {
  let mut col: [u8; 9] = [0; 9];

  for n in 0..9 {
    col[n] = input[n * 9 + col_i];
  }

  return col;
}
// pub fn get_square(input: &BoardArray, row_i: usize) -> &[u8] {
//   let start = row_i * 9;
//   let end = start + 9;
//   return &input[start..end];
// }
