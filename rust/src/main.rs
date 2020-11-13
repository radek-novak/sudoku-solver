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
    let initial_board = sudoku::Board::new(parsed_board);

    // initial_board.print_board();

    let mut queue = vec![initial_board];
    let mut current = queue[0];

    loop {
        if current.valid_board() {
            let next_boards = current.get_next_boards();

            match next_boards {
                Some(nb) => {
                    for next_board in nb {
                        queue.push(next_board);
                    }
                }
                None => {
                    current.print_board();
                    break;
                }
            }
        }

        let last = queue.pop();

        current = match last {
            Some(board) => board,
            None => {
                // Ran out of valid options,
                // either the puzzle doesn't have solution or
                // the program is incorrect.
                panic!("Solving error, puzzle is invalid");
            }
        }
    }
}
