extern crate rand;
use rand::Rng;
use std::io;
use colored::Colorize;


const BOARD_SIZE: usize = 3;

fn print_board(board: &[char; BOARD_SIZE*BOARD_SIZE]) {

    println!("");
    
    let mut counter: usize = 0;
    for cell in board.iter() {

        if cell == &'X' {
            print!(" {} ", cell.to_string().red().bold());
        } else if cell == &'O' {
            print!(" {} ", cell.to_string().green().bold());
        } else {
            print!(" {} ", cell);
        }

        counter += 1;
        if counter == BOARD_SIZE*BOARD_SIZE {
            print!("\n\n");
            break;
        }
        if (counter) % 3 == 0 {
            println!("\n---|---|---");
        } else {
            print!("|");
        }
    }
}

fn input_move(board: &mut [char; BOARD_SIZE*BOARD_SIZE], player: &String, input: usize) {

    board[input] = player.chars().next().unwrap();
}

fn validate_input(board: &[char; BOARD_SIZE*BOARD_SIZE], input: usize) -> bool {

    if input < 1 || input > BOARD_SIZE*BOARD_SIZE {
        println!("\n{} {}.", "Invalid input. Please enter a number between 1 and".red(), (BOARD_SIZE*BOARD_SIZE).to_string().red());
        return false;
    }

    if board[input - 1] == 'X' || board[input - 1] == 'O' {
        println!("\n{}", "Cell already occupied. Please choose another cell.".red());
        return false;
    }

    return true;

}

fn minimax(board: &mut [char; BOARD_SIZE*BOARD_SIZE], computer: &String, is_maximizing:bool) -> (usize, i32) {

    let mut best_move: usize = 0;
    let mut best_score: i32 = if is_maximizing { i32::MIN } else { i32::MAX };
    let mut score: i32;
    let mut winner:char;

    for i in 0..BOARD_SIZE*BOARD_SIZE {
        if board[i] != 'X' && board[i] != 'O' {
            let current_char = if is_maximizing { computer.chars().next().unwrap() } else { if computer == "X" { 'O' } else { 'X' } };
            board[i] = current_char;
            winner = check_winner(board);
            if winner == computer.chars().next().unwrap() {
                score = 1;
            } else if winner != '_' && winner != 'D' {
                score = -1;
            } else if winner == 'D' {
                score = 0;
            } else {
                score = minimax(board, computer, !is_maximizing).1;
            }

            board[i] = char::from_digit((i + 1) as u32, 10).unwrap();

            if is_maximizing {
                if score > best_score {
                    best_score = score;
                    best_move = i;
                }
            } else {
                if score < best_score {
                    best_score = score;
                    best_move = i;
                }
            }



        }
    }

    return (best_move, best_score);
}

fn computer_move(board: &mut [char; BOARD_SIZE*BOARD_SIZE], computer: &String, mode: &str) {

    let mut input: usize;

    if mode == "easy" {
        
        loop {
            input = rand::rng().random_range(0..BOARD_SIZE*BOARD_SIZE);

            if board[input] != 'X' && board[input] != 'O' {
                break;
            }
        }

    } else if mode == "hard" {
        input = minimax(board, computer, true).0;

    } else {

        panic!("Invalid mode");
        
    }
    

    input_move(board, computer, input);
}

fn check_winner(board: &[char; BOARD_SIZE*BOARD_SIZE]) -> char {

    let win_combinations = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
        [0, 4, 8], [2, 4, 6],            // Diagonals
    ];

    for combo in win_combinations.iter() {
        if board[combo[0]] == board[combo[1]] && board[combo[1]] == board[combo[2]] {
            return board[combo[0]];
        }
    }


    for i in 0..BOARD_SIZE*BOARD_SIZE {
        if board[i] != 'X' && board[i] != 'O' {
            return '_';
        }
    }

    return 'D';

}


fn main() {

    let mut board = [' '; BOARD_SIZE*BOARD_SIZE];

    for i in 0..BOARD_SIZE*BOARD_SIZE {
        board[i] = char::from_digit((i + 1) as u32, 10).unwrap();
    }

    let mut player: String = String::new();
    let computer: String;
    let mut winner:char;

    let mode: &str;
    let mut input_mode: String= String::new();

    println!("\n{}", "Welcome to Tic Tac Toe!".bold());

    println!("\n{}", "Choose hard mode for a challenge or easy mode for a quick game.".bold());
    println!("\n{}{}{}{}", "Type ".bold(), "hard".red().bold(), " or ".bold(), "easy".green().bold());

    io::stdin()
            .read_line(&mut input_mode)
            .expect("Failed to read input");


    if input_mode.trim().to_lowercase() == "hard" {
        mode = "hard";
        println!("\n{}{} {}{}", "You have chosen ".bold(), "unbeatable".strikethrough().bold().black(), mode.red().bold(), " mode.".bold());
    } else {
        mode = "easy";
        println!("\n{}{}{}", "You have chosen ".bold(), mode.green().bold(), " mode.".bold());
    }



    println!("\n{}{}{}{}", "Choose whether you will be ".bold(), "O".green().bold(), " or ", "X".red().bold());

    io::stdin()
            .read_line(&mut player)
            .expect("Failed to read input");

    player = player.trim().to_string().to_uppercase();

    if player.trim() == "O" || player.trim() == "o" {
        computer = "X".to_string();
        player = "O".to_string();
        println!("\nYou are {}. Computer is {}.\n", player.green().bold(), computer.red().bold());

    } else {
        computer = "O".to_string();
        player = "X".to_string();
        println!("\nYou are {}. Computer is {}.\n", player.red().bold(), computer.green().bold());

        // AS O STARTS FIRST, COMPUTER MAKES THE FIRST MOVE
        computer_move(&mut board, &computer, &mode);
    }



    loop {

        print_board(&board);

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input: usize = input.trim().parse().expect("Please type a number!");

        if !validate_input(&board, input) {
            continue;
        }

        input_move(&mut board, &player, input - 1);
        winner = check_winner(&board);

        if winner != 'D' && winner != '_' {
            print_board(&board);
            println!("\n{}\n", "You win!".green().bold());
            break;
        } else if winner == 'D' {
            print_board(&board);
            println!("\n{}\n", "It's a draw!".bold());
            break;
        }

        computer_move(&mut board, &computer, &mode);
        winner = check_winner(&board);

        if winner != 'D' && winner != '_' {
            print_board(&board);
            if mode == "easy" {
                println!("\n{}", "Computer wins!".red().bold());
                println!("{}\n", "THE COMPUTER WAS CHOOSING RANDOM MOVES. HOW THE HELL DID YOU LOSE?".black().bold());
            } else {
                println!("\n{}", "Computer wins!".red().bold());
                println!("{}{}{}\n", "Maybe it ".black().bold(), "was ".bold().black().italic(), "unbeatable.".black().bold());
            }
            break;
            
        } else if winner == 'D' {
            print_board(&board);
            println!("\n{}\n", "It's a draw!".bold());
            break;
        }

    }
}