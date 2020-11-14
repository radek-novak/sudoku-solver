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

    println!("Puzzle:");
    initial_board.print_board();
    let zero_count = parsed_board.iter().fold(0, |acc, num| {
        if *num == 0 {
            return acc + 1;
        }
        return acc;
    });
    println!("Clues: {}", 81 - zero_count);

    let mut queue = vec![initial_board];
    let mut current = queue[0];
    let mut max_queue_len: usize = queue.len();
    let mut boards_checked: u128 = 0;

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
                    println!("Solution:");
                    current.print_board();
                    println!("checked {} boards", boards_checked);
                    println!("max queue length was: {}", max_queue_len);

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
        };

        let current_len = queue.len();
        if current_len > max_queue_len {
            max_queue_len = current_len;
        }

        boards_checked += 1;
    }
}
