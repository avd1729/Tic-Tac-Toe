use std::io;

const ROWS: usize = 3;
const COLS: usize = 3;

fn main() {
    let mut input = String::new();
    let mut board = [[' '; COLS]; ROWS];
    let mut player = 'X';
    let mut game_over = false;

    while !game_over {
        print_board(&board);
        println!("Player {} enter (row col): ", player);

        input.clear(); // Clear the input buffer for each iteration

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() == 2 {
            let row: usize = parts[0].parse().expect("Invalid input for row");
            let col: usize = parts[1].parse().expect("Invalid input for col");

            if row < ROWS && col < COLS && board[row][col] == ' ' {
                board[row][col] = player;
                game_over = have_won(&board, player);

                if game_over {
                    println!("Player {} has won!", player);
                } else {
                    player = if player == 'X' { 'O' } else { 'X' };
                }
            } else {
                println!("Invalid move. Try again!");
            }
        } else {
            println!("Invalid input. Please enter row and col separated by space.");
        }
    }
}

fn have_won(board: &[[char; COLS]; ROWS], player: char) -> bool {
    // Rows
    for i in 0..ROWS {
        if board[i][0] == player && board[i][1] == player && board[i][2] == player {
            return true;
        }
    }

    // Columns
    for i in 0..COLS {
        if board[0][i] == player && board[1][i] == player && board[2][i] == player {
            return true;
        }
    }

    // Diagonals
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }

    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }

    false
}

fn print_board(board: &[[char; COLS]; ROWS]) {
    for row in board.iter() {
        for &element in row {
            print!("{} | ", element);
        }
        println!();
    }
}


