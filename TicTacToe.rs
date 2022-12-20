use std::io;

const ROWS: usize = 3;
const COLS: usize = 3;

// The game board is represented as a 2D array
// X represents player 1's move, O represents player 2's move
// An empty space is represented by a space character
let mut board: [[char; COLS]; ROWS] = [[' ', ' ', ' '], [' ', ' ', ' '], [' ', ' ', ' ']];

fn print_board() {
    println!("   0 1 2");
    println!("  ----------");
    for (i, row) in board.iter().enumerate() {
        print!("{} |", i);
        for cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
    println!("  ----------");
}

fn get_input() -> (usize, usize) {
    println!("Enter row and column (e.g. '1 2'):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let mut inputs = input.split_whitespace();
    let row: usize = inputs.next().unwrap().parse().expect("Invalid input");
    let col: usize = inputs.next().unwrap().parse().expect("Invalid input");

    (row, col)
}

fn main() {
    println!("Welcome to tic-tac-toe!");
    println!("Player 1 is X, Player 2 is O");

    let mut player = 1;

    loop {
        print_board();

        println!("Player {} turn", player);
        let (row, col) = get_input();

        if board[row][col] != ' ' {
            println!("Invalid move! Try again.");
            continue;
        }

        if player == 1 {
            board[row][col] = 'X';
        } else {
            board[row][col] = 'O';
        }

        player = 3 - player; // switch player (1 -> 2, 2 -> 1)

        // Check if the game is over
        let mut full = true;
        let mut winner = ' ';

}