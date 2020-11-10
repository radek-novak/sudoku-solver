// magic row, refer to idx.txt
const FIRST_ROW: [usize; 9] = [0, 1, 2, 9, 10, 11, 18, 19, 20];

pub type BoardArray = [usize; 81];

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

fn get_row(input: &BoardArray, row_idx: usize) -> [usize; 9] {
  let mut row: [usize; 9] = [0; 9];

  for i in 0..9 {
    row[i] = input[row_idx * 9 + i];
  }

  return row;
  // let start = row_idx * 9;
  // let end = start + 9;
  // return &input[start..end];
}

fn get_column(input: &BoardArray, col_i: usize) -> [usize; 9] {
  let mut col: [usize; 9] = [0; 9];

  for n in 0..9 {
    col[n] = input[n * 9 + col_i];
  }

  return col;
}

fn get_square(input: &BoardArray, sq_idx: usize) -> [usize; 9] {
  let mut sq: [usize; 9] = [0; 9];

  let start_n = FIRST_ROW[sq_idx] * 3;

  for n in 0..9 {
    let idx_n = start_n + FIRST_ROW[n];
    sq[n] = input[idx_n];
  }
  return sq;
}

fn valid_nine(nine: [usize; 9]) -> bool {
  let mut results: [bool; 9] = [false; 9];

  for i in 0..9 {
    let current = nine[i];
    if results[current] {
      return false;
    }
    if current == 0 {
      continue;
    }
    results[current] = true;
  }

  return true;
}

pub fn valid_board(input: &BoardArray) -> bool {
  for s in 0..9 {
    if !valid_nine(get_row(input, s))
      || !valid_nine(get_column(input, s))
      || !valid_nine(get_square(input, s))
    {
      return false;
    }
  }

  return true;
}

#[cfg(test)]
mod tests {
  #[test]
  fn check_nine_works() {
    assert_eq!(super::valid_nine([0, 1, 2, 3, 4, 5, 6, 7, 8]), true);
    assert_eq!(super::valid_nine([0, 1, 2, 3, 0, 5, 6, 7, 8]), true);
    assert_eq!(super::valid_nine([3, 1, 1, 1, 1, 2, 2, 3, 3]), false);
    assert_eq!(super::valid_nine([0, 1, 1, 1, 1, 2, 2, 3, 3]), false);
    assert_eq!(super::valid_nine([0, 1, 2, 3, 4, 5, 6, 8, 8]), false);
    assert_eq!(super::valid_nine([0, 1, 2, 3, 4, 5, 6, 1, 8]), false);
  }
}
