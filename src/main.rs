// ======================================
// Tic Tac Toe with Crossterm
// Rust 2024 Edition
// Week 3: Difficulty + Colors + Score Tracking
// ======================================

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute, queue,
    style::{Print, Stylize},
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
};
use rand::seq::IteratorRandom;
use std::io::{stdout, Result, Stdout, Write};

// ======================================
// CONSTANTS & TYPES
// ======================================

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
const EMPTY_CELLS: [char; 9] = ['1','2','3','4','5','6','7','8','9'];

#[derive(Clone, Copy)]
enum Difficulty {
    Easy,
    Hard,
}

struct Score {
    x_wins: u32,
    o_wins: u32,
    draws: u32,
}

// ======================================
// MAIN FUNCTION
// ======================================

fn main() -> Result<()> {
    enable_raw_mode()?; // enable raw terminal input
    let mut stdout = stdout();

    // Choose difficulty once
    let difficulty = choose_difficulty(&mut stdout)?;
    let mut score = Score { x_wins: 0, o_wins: 0, draws: 0 };

    loop {
        let mut board = EMPTY_CELLS;
        let mut current_player = PLAYER_X;

        show_welcome_screen(&mut stdout)?;

        // Main game loop
        loop {
            draw_board(&board, &mut stdout, &score)?;
            let pos = if current_player == PLAYER_X {
                get_human_move(&mut stdout, &board, current_player)?
            } else {
                get_ai_move(&board, difficulty)
            };

            board[pos] = current_player;

            if let Some((winner, line)) = check_winner(&board) {
                draw_board(&board, &mut stdout, &score)?;
                print_winner(&mut stdout, winner, &board, line)?;
                match winner {
                    PLAYER_X => score.x_wins += 1,
                    PLAYER_O => score.o_wins += 1,
                    _ => {}
                }
                break;
            }

            if is_draw(&board) {
                draw_board(&board, &mut stdout, &score)?;
                print_draw(&mut stdout)?;
                score.draws += 1;
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

/// Difficulty selection at start
fn choose_difficulty(stdout: &mut Stdout) -> Result<Difficulty> {
    execute!(stdout, Clear(ClearType::All))?;
    queue!(stdout, MoveTo(0,0), Print("Choose difficulty:\n"))?;
    queue!(stdout, MoveTo(0,1), Print("1) Easy (random)\n"))?;
    queue!(stdout, MoveTo(0,2), Print("2) Hard (blocks + wins)\n"))?;
    queue!(stdout, MoveTo(0,4), Print("Press 1 or 2: "))?;
    stdout.flush()?;

    loop {
        if let Event::Key(event) = read()? {
            if let KeyCode::Char(c) = event.code {
                return match c {
                    '1' => Ok(Difficulty::Easy),
                    '2' => Ok(Difficulty::Hard),
                    _ => {
                        queue!(stdout, Print("\nInvalid. Press 1 or 2: "))?;
                        stdout.flush()?;
                        continue;
                    }
                };
            }
        }
    }
}

/// Display welcome screen
fn show_welcome_screen(stdout: &mut Stdout) -> Result<()> {
    execute!(stdout, Clear(ClearType::All))?;
    queue!(stdout, MoveTo(0, 0), Print("==== Welcome to Tic Tac Toe ====\n"))?;
    queue!(stdout, MoveTo(0, 2), Print("You are 'X'. The computer is 'O'.\n"))?;
    queue!(stdout, MoveTo(0, 3), Print("Select cells by typing numbers 1-9.\n"))?;
    queue!(stdout, MoveTo(0, 5), Print("Press any key to start..."))?;
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

/// Draw the game board + score
fn draw_board(board: &[char; 9], stdout: &mut Stdout, score: &Score) -> Result<()> {
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

    // Show score
    queue!(stdout, MoveTo(0, 10), Print(format!(
        "Score: X={} | O={} | Draws={}\n",
        score.x_wins, score.o_wins, score.draws
    )))?;

    stdout.flush()?;
    Ok(())
}

/// Print winner message with colors
fn print_winner(stdout: &mut Stdout, winner: char, board: &[char; 9], line: [usize; 3]) -> Result<()> {
    // Highlight winning line in green
    for &idx in &line {
        let row = idx / 3;
        let col = idx % 3;
        let x = col as u16 * 4; // rough positioning
        let y = row as u16 * 2 + 2;
        queue!(stdout, MoveTo(x, y), Print(board[idx].to_string().green().bold()))?;
    }

    queue!(stdout, MoveTo(0, 12), Print(format!("\nPlayer {} wins! 🎉\n", winner)))?;
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
    queue!(stdout, MoveTo(0, 14), Print(format!("Player {}, enter position (1-9): ", player)))?;
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

/// Get move from AI (easy = random, hard = smart block/win)
fn get_ai_move(board: &[char; 9], difficulty: Difficulty) -> usize {
    match difficulty {
        Difficulty::Easy => {
            let mut rng = rand::thread_rng();
            board.iter()
                .enumerate()
                .filter(|&(_, &c)| c != PLAYER_X && c != PLAYER_O)
                .map(|(i, _)| i)
                .choose(&mut rng)
                .unwrap()
        }
        Difficulty::Hard => smart_ai_move(board),
    }
}

/// Smarter AI: block or win if possible
fn smart_ai_move(board: &[char; 9]) -> usize {
    let wins = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8], // rows
        [0, 3, 6], [1, 4, 7], [2, 5, 8], // cols
        [0, 4, 8], [2, 4, 6],            // diagonals
    ];

    // Try to win
    for &line in &wins {
        let cells: Vec<char> = line.iter().map(|&i| board[i]).collect();
        if cells.iter().filter(|&&c| c == PLAYER_O).count() == 2 &&
           cells.iter().any(|&c| c != PLAYER_X && c != PLAYER_O) {
            return line.iter().find(|&&i| board[i] != PLAYER_X && board[i] != PLAYER_O).copied().unwrap();
        }
    }

    // Try to block X
    for &line in &wins {
        let cells: Vec<char> = line.iter().map(|&i| board[i]).collect();
        if cells.iter().filter(|&&c| c == PLAYER_X).count() == 2 &&
           cells.iter().any(|&c| c != PLAYER_X && c != PLAYER_O) {
            return line.iter().find(|&&i| board[i] != PLAYER_X && board[i] != PLAYER_O).copied().unwrap();
        }
    }

    // Else random
    let mut rng = rand::thread_rng();
    board.iter()
        .enumerate()
        .filter(|&(_, &c)| c != PLAYER_X && c != PLAYER_O)
        .map(|(i, _)| i)
        .choose(&mut rng)
        .unwrap()
}

// ======================================
// GAME LOGIC FUNCTIONS
// ======================================

/// Check if a player has won (returns winner + line)
fn check_winner(board: &[char; 9]) -> Option<(char, [usize; 3])> {
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
            return Some((board[line[0]], line));
        }
    }
    None
}
