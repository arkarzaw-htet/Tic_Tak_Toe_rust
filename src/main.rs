// ======================================
// Tic Tac Toe with Crossterm
// Organized, Modular, and AI Enabled
// Rust 2024 Edition
// ======================================

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
};
use rand::seq::IteratorRandom;
use rand::rng;
use std::io::{stdout, Result, Stdout, Write};

// ======================================
// CONSTANTS & TYPES
// ======================================

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
const EMPTY_CELLS: [char; 9] = ['1','2','3','4','5','6','7','8','9'];

// ======================================
// MAIN FUNCTION
// ======================================

fn main() -> Result<()> {
    enable_raw_mode()?; // enable raw terminal input
    let mut stdout = stdout();

    loop {
        let mut board = EMPTY_CELLS;
        let mut current_player = PLAYER_X;

        show_welcome_screen(&mut stdout)?;

        // Main game loop
        loop {
            draw_board(&board, &mut stdout)?;
            let pos = if current_player == PLAYER_X {
                get_human_move(&mut stdout, &board, current_player)?
            } else {
                get_ai_move(&board)
            };

            board[pos] = current_player;

            if let Some(winner) = check_winner(&board) {
                draw_board(&board, &mut stdout)?;
                print_winner(&mut stdout, winner)?;
                break;
            }

            if is_draw(&board) {
                draw_board(&board, &mut stdout)?;
                print_draw(&mut stdout)?;
                break;
            }

            current_player = switch_player(current_player);
        }

        if !ask_replay(&mut stdout)? {
            break;
        }
    }

    disable_raw_mode()?; // disable raw mode before exiting
    Ok(())
}

// ======================================
// GAME FLOW FUNCTIONS
// ======================================

/// Display welcome screen
fn show_welcome_screen(stdout: &mut Stdout) -> Result<()> {
    execute!(stdout, Clear(ClearType::All))?;

    // Center X starting column
    let start_x = 20;
    let mut y = 0;

    queue!(stdout, MoveTo(start_x, y), Print("==== Welcome to Tic Tac Toe ===="))?;
    y += 2;
    queue!(stdout, MoveTo(start_x, y), Print("You are 'X'. The computer is 'O'."))?;
    y += 1;
    queue!(stdout, MoveTo(start_x, y), Print("Select cells by typing numbers 1-9."))?;
    y += 1;
    queue!(stdout, MoveTo(start_x, y), Print("Press any key to start..."))?;

    stdout.flush()?;
    read()?; // Wait for any key
    Ok(())
}


/// Switch player between X and O
fn switch_player(current: char) -> char {
    if current == PLAYER_X { PLAYER_O } else { PLAYER_X }
}

/// Check if the game is a draw
fn is_draw(board: &[char; 9]) -> bool {
    board.iter().all(|&c| c == PLAYER_X || c == PLAYER_O)
}

/// Ask player if they want to replay
fn ask_replay(stdout: &mut Stdout) -> Result<bool> {
    queue!(stdout, Print("\nPlay again? (y/n): "))?;
    stdout.flush()?;
    loop {
        if let Event::Key(event) = read()? {
            if let KeyCode::Char(c) = event.code {
                match c {
                    'y' | 'Y' => return Ok(true),
                    'n' | 'N' => return Ok(false),
                    _ => {
                        queue!(stdout, Print("\nInvalid input. Type y or n: "))?;
                        stdout.flush()?;
                    }
                }
            }
        }
    }
}

// ======================================
// DRAWING FUNCTIONS
// ======================================

/// Draw the game board
fn draw_board(board: &[char; 9], stdout: &mut Stdout) -> Result<()> {
    execute!(stdout, Clear(ClearType::All))?;

    let grid = vec![
        format!(" {} | {} | {} ", board[0], board[1], board[2]),
        "---+---+---".to_string(),
        format!(" {} | {} | {} ", board[3], board[4], board[5]),
        "---+---+---".to_string(),
        format!(" {} | {} | {} ", board[6], board[7], board[8]),
    ];

    queue!(stdout, MoveTo(0, 0), Print("Tic Tac Toe\n\n"))?;

    for (i, line) in grid.iter().enumerate() {
        queue!(stdout, MoveTo(0, (i + 2) as u16), Print(line))?;
    }

    queue!(stdout, MoveTo(0, 8), Print("\n"))?;
    stdout.flush()?;
    Ok(())
}

/// Print winner message
fn print_winner(stdout: &mut Stdout, winner: char) -> Result<()> {
    queue!(stdout, Print(format!("\nPlayer {} wins! 🎉\n", winner)))?;
    stdout.flush()?;
    Ok(())
}

/// Print draw message
fn print_draw(stdout: &mut Stdout) -> Result<()> {
    queue!(stdout, Print("\nIt's a draw! 🤝\n"))?;
    stdout.flush()?;
    Ok(())
}

// ======================================
// PLAYER INPUT FUNCTIONS
// ======================================

/// Get move from human player
fn get_human_move(stdout: &mut Stdout, board: &[char; 9], player: char) -> Result<usize> {
    queue!(stdout, Print(format!("Player {}, enter position (1-9): ", player)))?;
    stdout.flush()?;
    loop {
        if let Event::Key(event) = read()? {
            if let KeyCode::Char(c) = event.code {
                if let Some(digit) = c.to_digit(10) {
                    let idx = (digit - 1) as usize;
                    if idx < 9 && board[idx] != PLAYER_X && board[idx] != PLAYER_O {
                        return Ok(idx);
                    }
                }
            }
        }
        queue!(stdout, Print("\nInvalid input or cell occupied. Try again: "))?;
        stdout.flush()?;
    }
}

/// Get move from AI (random empty cell)
fn get_ai_move(board: &[char; 9]) -> usize {
    let mut rng = rng();
    board
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c != PLAYER_X && c != PLAYER_O)
        .map(|(i, _)| i)
        .choose(&mut rng)
        .unwrap()
}

// ======================================
// GAME LOGIC FUNCTIONS
// ======================================

/// Check if a player has won
fn check_winner(board: &[char; 9]) -> Option<char> {
    let wins = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // horizontal
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // vertical
        [0, 4, 8], [2, 4, 6],            // diagonal
    ];

    for &line in &wins {
        if board[line[0]] == board[line[1]]
            && board[line[1]] == board[line[2]]
            && (board[line[0]] == PLAYER_X || board[line[0]] == PLAYER_O)
        {
            return Some(board[line[0]]);
        }
    }
    None
}
