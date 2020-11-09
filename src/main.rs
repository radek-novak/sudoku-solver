use sudoku;

use std::env;
use std::fs;

fn parse_board(s: String) -> sudoku::BoardArray {
    let mut parsed_board: [u8; 81] = [0; 81];
    let mut pos = 0;

    for l in s.chars() {
        match l.to_digit(10) {
            Some(val) => {
                parsed_board[pos] = val as u8;
                pos += 1;
            }
            None => continue,
        }
    }

    parsed_board
}

fn retrieve_filename() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("not enough arguments!");
    }

    return args[1].clone();
}

fn main() {
    let filename = retrieve_filename();
    let contents = fs::read_to_string(filename).expect("Couldn't read the file");

    let parsed_board = parse_board(contents);

    sudoku::print_board(parsed_board);

    for s in 0..9 {
        println!("{:?}", sudoku::get_row(&parsed_board, s));
        println!("{:?}", sudoku::get_column(&parsed_board, s));
        println!("{:?}", sudoku::get_square(&parsed_board, s));
    }
}
