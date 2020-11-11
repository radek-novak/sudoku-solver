use sudoku;

use std::env;
use std::fs;

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

    let parsed_board = sudoku::parse_board(contents);

    // sudoku::print_board(parsed_board);

    let b = sudoku::Board::new(parsed_board);

    b.print_board();

    // for s in 0..9 {
    //     println!("{:?}", sudoku::get_row(&parsed_board, s));
    //     println!("{:?}", sudoku::get_column(&parsed_board, s));
    //     println!("{:?}", sudoku::get_square(&parsed_board, s));
    // }

    // println!("{:?}", sudoku::valid_board(&parsed_board));
}
