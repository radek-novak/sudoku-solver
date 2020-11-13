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

#[derive(Copy, Clone)]
pub struct Board {
  values: BoardArray,
}

impl Board {
  pub fn new(input: BoardArray) -> Board {
    Board { values: input }
  }

  pub fn print_board(&self) {
    print_board(self.values);
  }

  pub fn valid_board(&self) -> bool {
    valid_board(&self.values)
  }

  pub fn get_next_empty(&self) -> Option<usize> {
    return self.values.iter().position(|&x| x == 0);
  }

  fn get_next_clone(&self, idx: usize, num: usize) -> Board {
    let mut b = Board::new(self.values);
    b.values[idx] = num;

    return b;
  }

  pub fn get_next_boards(&self) -> Option<Vec<Board>> {
    match self.get_next_empty() {
      Some(i) => {
        let mut boards: Vec<Board> = vec![];
        for num in 1..10 {
          boards.push(self.get_next_clone(i, num));
        }
        return Some(boards);
      }
      None => return None,
    };
  }
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
  let mut results: [bool; 10] = [false; 10];

  for i in 0..9 {
    let current = nine[i];
    if current == 0 {
      continue;
    }
    if results[current] {
      return false;
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

pub fn parse_board(s: String) -> BoardArray {
  let mut parsed_board: BoardArray = [0; 81];
  let mut pos = 0;

  for l in s.chars() {
    match l.to_digit(10) {
      Some(val) => {
        parsed_board[pos] = val as usize;
        pos += 1;
      }
      None => continue,
    }
  }

  parsed_board
}

#[cfg(test)]
mod tests {
  const VALID_BOARD: &str =
    "040000179002008054006005008080070910050090030019060040300400700570100200928000060";
  const INVALID_BOARD: &str =
    "440000179002008054006005008080070910050090030019060040300400700570100200928000060";
  #[test]
  fn check_nine_works() {
    assert_eq!(super::valid_nine([0, 1, 2, 3, 4, 5, 6, 7, 8]), true);
    assert_eq!(super::valid_nine([0, 1, 2, 3, 0, 5, 6, 9, 8]), true);
    assert_eq!(super::valid_nine([3, 1, 1, 1, 1, 2, 2, 3, 3]), false);
    assert_eq!(super::valid_nine([0, 1, 1, 1, 1, 2, 2, 3, 3]), false);
    assert_eq!(super::valid_nine([0, 1, 2, 3, 4, 5, 6, 8, 8]), false);
    assert_eq!(super::valid_nine([0, 1, 2, 3, 4, 5, 6, 1, 8]), false);
  }
  #[test]
  fn check_board() {
    let board_array = super::parse_board(VALID_BOARD.to_string());
    let inv_board_array = super::parse_board(INVALID_BOARD.to_string());
    assert_eq!(super::valid_board(&board_array), true);
    assert_eq!(super::valid_board(&inv_board_array), false);
  }
  #[test]
  fn next_num() {
    let valid_board_array = super::parse_board(VALID_BOARD.to_string());
    let invalid_board_array = super::parse_board(INVALID_BOARD.to_string());
    let vb = super::Board::new(valid_board_array);
    let ib = super::Board::new(invalid_board_array);

    assert_eq!(vb.get_next_empty(), Some(0));
    assert_eq!(ib.get_next_empty(), Some(2));
  }
}
