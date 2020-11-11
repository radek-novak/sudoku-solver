// Row of indices of the small sudoku squares
//
// Find the first index of any square by multiplying that index in the
// first row by 3.
// E.g. square 7 is [0, 7] -> 19 * 3 -> 57. To find the remaining indices
// add the first row: 57+0, 57+1, 57+2, 57+9, +10, ... +20
//
//      0  1  2  3  4  5  6  7  8
//    ___________________________
// 0 | 00 01 02 09 10 11 18 19 20
// 1 | 03 04 05 12 13 14 21 22 23
// 2 | 06 07 08 15 16 17 24 25 26
// 3 | 27 28 29 36 37 38 47 48 49
// 4 | 30 31 32 39 40 41 50 51 52
// 5 | 33
// 6 | 54
// 7 | 57
// 8 | 60 61 62 69 70 71 78 79 80
//
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
