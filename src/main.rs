// ======================================
// Tic Tac Toe — Crossterm
// PVP / AI (Easy/Hard), Winner Highlight, Scoreboard
// Rust 2024 Edition
// ======================================

use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use rand::seq::IteratorRandom;
use std::io::{stdout, Result, Stdout, Write};

// ======================================
// CONSTANTS & TYPES
// ======================================

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
const EMPTY_CELLS: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[derive(Clone, Copy, PartialEq)]
enum Difficulty {
    Easy,
    Hard,
}

#[derive(Clone, Copy)]
enum GameMode {
    Friend,
    AI(Difficulty, bool), // (difficulty, player_first)
}

// ======================================
// MAIN
// ======================================

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();

    let mut score_player_x = 0;
    let mut score_player_o = 0;
    let mut score_draws = 0;

    loop {
        let mut board = EMPTY_CELLS;
        let mut current_player = PLAYER_X;

        show_welcome_screen(&mut stdout)?;
        let game_mode = ask_game_mode(&mut stdout)?;

        // Game loop
        loop {
            draw_board(&board, &mut stdout)?;
            print_turn_hint(&mut stdout, game_mode, current_player)?;

            let pos = if is_human_turn(game_mode, current_player) {
                get_human_move(&mut stdout, &board, current_player)?
            } else {
                match game_mode {
                    GameMode::AI(difficulty, player_first) => {
                        let _computer_mark = if player_first { PLAYER_O } else { PLAYER_X };
                        match difficulty {
                            Difficulty::Easy => get_ai_move_random(&board),
                            Difficulty::Hard => get_ai_move_blocking(&board),
                        }
                    }
                    GameMode::Friend => unreachable!(),
                }
            };

            board[pos] = current_player;

            if let Some((winner, line)) = check_winner(&board) {
                draw_board_highlight(&board, &mut stdout, &line)?;
                print_winner(&mut stdout, winner)?;
                if winner == PLAYER_X {
                    score_player_x += 1;
                } else {
                    score_player_o += 1;
                }
                break;
            }

            if is_draw(&board) {
                draw_board(&board, &mut stdout)?;
                print_draw(&mut stdout)?;
                score_draws += 1;
                break;
            }

            current_player = switch_player(current_player);
        }

        print_scoreboard(&mut stdout, score_player_x, score_player_o, score_draws)?;
        if !ask_replay(&mut stdout)? {
            break;
        }
    }

    disable_raw_mode()?;
    Ok(())
}

// ======================================
// GAME FLOW
// ======================================

fn show_welcome_screen(stdout: &mut Stdout) -> Result<()> {
    execute!(stdout, Clear(ClearType::All))?;
    queue!(stdout, MoveTo(8, 0), Print("==== Welcome to Tic Tac Toe ===="))?;
    queue!(stdout, MoveTo(8, 2), Print("Controls: press number keys 1–9 to place your mark."))?;
    queue!(stdout, MoveTo(8, 3), Print("Win by getting 3 in a row (rows, columns, diagonals)."))?;
    queue!(stdout, MoveTo(8, 5), Print("Press any key to continue..."))?;
    stdout.flush()?;
    read()?; // wait any key
    Ok(())
}

fn ask_game_mode(stdout: &mut Stdout) -> Result<GameMode> {
    execute!(stdout, Clear(ClearType::All))?;
    queue!(stdout, MoveTo(0, 0), Print("Tic Tac Toe\n\n"))?;
    queue!(stdout, MoveTo(0, 2), Print("Play with a friend (f) or AI (a)? "))?;
    stdout.flush()?;

    loop {
        if let Event::Key(event) = read()? {
            if let KeyCode::Char(c) = event.code {
                match c {
                    'f' | 'F' => return Ok(GameMode::Friend),
                    'a' | 'A' => {
                        let difficulty = ask_difficulty(stdout)?;
                        let player_first = ask_first_player(stdout)?;
                        return Ok(GameMode::AI(difficulty, player_first));
                    }
                    _ => {
                        queue!(stdout, Print("\nInvalid input. Type f or a: "))?;
                        stdout.flush()?;
                    }
                }
            }
        }
    }
}

fn ask_difficulty(stdout: &mut Stdout) -> Result<Difficulty> {
    queue!(stdout, MoveTo(0, 4), Print("Select difficulty: (e)asy or (h)ard: "))?;
    stdout.flush()?;
    loop {
        if let Event::Key(event) = read()? {
            if let KeyCode::Char(c) = event.code {
                match c {
                    'e' | 'E' => return Ok(Difficulty::Easy),
                    'h' | 'H' => return Ok(Difficulty::Hard),
                    _ => {
                        queue!(stdout, Print("\nInvalid input. Type e or h: "))?;
                        stdout.flush()?;
                    }
                }
            }
        }
    }
}

fn ask_first_player(stdout: &mut Stdout) -> Result<bool> {
    queue!(
        stdout,
        MoveTo(0, 6),
        Print("Who goes first? (p = player, c = computer): ")
    )?;
    stdout.flush()?;
    loop {
        if let Event::Key(event) = read()? {
            if let KeyCode::Char(c) = event.code {
                match c {
                    'p' | 'P' => return Ok(true),
                    'c' | 'C' => return Ok(false),
                    _ => {
                        queue!(stdout, Print("\nInvalid input. Type p or c: "))?;
                        stdout.flush()?;
                    }
                }
            }
        }
    }
}

fn is_human_turn(game_mode: GameMode, current: char) -> bool {
    match game_mode {
        GameMode::Friend => true,
        GameMode::AI(_difficulty, player_first) => {
            if player_first {
                current == PLAYER_X
            } else {
                current == PLAYER_O
            }
        }
    }
}

fn switch_player(current: char) -> char {
    if current == PLAYER_X {
        PLAYER_O
    } else {
        PLAYER_X
    }
}

fn is_draw(board: &[char; 9]) -> bool {
    board.iter().all(|&c| c == PLAYER_X || c == PLAYER_O)
}

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
// DRAWING
// ======================================

fn draw_board(board: &[char; 9], stdout: &mut Stdout) -> Result<()> {
    execute!(stdout, Clear(ClearType::All))?;
    queue!(stdout, MoveTo(0, 0), Print("Tic Tac Toe\n"))?;
    queue!(stdout, MoveTo(0, 1), Print("========================\n"))?;

    for row in 0..3 {
        if row > 0 {
            queue!(stdout, MoveTo(0, (row as u16) * 2 + 2), Print("---+---+---"))?;
        }
        let y = (row as u16) * 2 + 3;
        for col in 0..3 {
            let idx = row * 3 + col;
            let x = match col {
                0 => 1,
                1 => 5,
                _ => 9,
            } as u16;

            let color = match board[idx] {
                PLAYER_X => Color::Red,
                PLAYER_O => Color::Blue,
                _ => Color::White,
            };

            queue!(
                stdout,
                MoveTo(x, y),
                SetForegroundColor(color),
                Print(board[idx]),
                ResetColor
            )?;

            if col < 2 {
                let bar_x = match col {
                    0 => 3,
                    _ => 7,
                } as u16;
                queue!(stdout, MoveTo(bar_x, y), Print("|"))?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}

fn draw_board_highlight(board: &[char; 9], stdout: &mut Stdout, line: &[usize; 3]) -> Result<()> {
    execute!(stdout, Clear(ClearType::All))?;
    queue!(stdout, MoveTo(0, 0), Print("Tic Tac Toe\n"))?;
    queue!(stdout, MoveTo(0, 1), Print("========================\n"))?;

    for row in 0..3 {
        if row > 0 {
            queue!(stdout, MoveTo(0, (row as u16) * 2 + 2), Print("---+---+---"))?;
        }
        let y = (row as u16) * 2 + 3;
        for col in 0..3 {
            let idx = row * 3 + col;
            let x = match col {
                0 => 1,
                1 => 5,
                _ => 9,
            } as u16;

            let color = if line.contains(&idx) {
                Color::Green
            } else {
                match board[idx] {
                    PLAYER_X => Color::Red,
                    PLAYER_O => Color::Blue,
                    _ => Color::White,
                }
            };

            queue!(
                stdout,
                MoveTo(x, y),
                SetForegroundColor(color),
                Print(board[idx]),
                ResetColor
            )?;

            if col < 2 {
                let bar_x = match col {
                    0 => 3,
                    _ => 7,
                } as u16;
                queue!(stdout, MoveTo(bar_x, y), Print("|"))?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}

fn print_turn_hint(stdout: &mut Stdout, mode: GameMode, current: char) -> Result<()> {
    let hint = match mode {
        GameMode::Friend => format!("Player {}, enter position (1-9): ", current),
        GameMode::AI(_d, player_first) => {
            if is_human_turn(mode, current) {
                let you_mark = if player_first { PLAYER_X } else { PLAYER_O };
                format!("Your turn ({}). Enter position (1-9): ", you_mark)
            } else {
                let comp_mark = if player_first { PLAYER_O } else { PLAYER_X };
                format!("Computer's turn ({})...", comp_mark)
            }
        }
    };

    let color = if is_human_turn(mode, current) {
        Color::Yellow
    } else {
        Color::Magenta
    };

    queue!(
        stdout,
        MoveTo(0, 10),
        SetForegroundColor(color),
        Print(hint),
        ResetColor
    )?;
    stdout.flush()?;
    Ok(())
}

fn print_winner(stdout: &mut Stdout, winner: char) -> Result<()> {
    let color = if winner == PLAYER_X { Color::Red } else { Color::Blue };
    queue!(
        stdout,
        MoveTo(0, 12),
        SetForegroundColor(color),
        Print(format!("\nPlayer {} wins! 🎉\n", winner)),
        ResetColor
    )?;
    stdout.flush()?;
    Ok(())
}

fn print_draw(stdout: &mut Stdout) -> Result<()> {
    queue!(
        stdout,
        MoveTo(0, 12),
        SetForegroundColor(Color::Yellow),
        Print("\nIt's a draw! 🤝\n"),
        ResetColor
    )?;
    stdout.flush()?;
    Ok(())
}

fn print_scoreboard(stdout: &mut Stdout, px: i32, po: i32, draws: i32) -> Result<()> {
    queue!(
        stdout,
        SetForegroundColor(Color::Red),
        Print(format!("X: {} ", px)),
        SetForegroundColor(Color::Blue),
        Print(format!("O: {} ", po)),
        SetForegroundColor(Color::Yellow),
        Print(format!("Draws: {}\n", draws)),
        ResetColor
    )?;
    stdout.flush()?;
    Ok(())
}

// ======================================
// INPUT
// ======================================

fn get_human_move(stdout: &mut Stdout, board: &[char; 9], _player: char) -> Result<usize> {
    stdout.flush()?;
    loop {
        if let Event::Key(event) = read()? {
            if let KeyCode::Char(c) = event.code {
                if let Some(d) = c.to_digit(10) {
                    let idx = (d - 1) as usize;
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

// ======================================
// AI
// ======================================

fn get_ai_move_random(board: &[char; 9]) -> usize {
    let mut rng = rand::thread_rng();
    board
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c != PLAYER_X && c != PLAYER_O)
        .map(|(i, _)| i)
        .choose(&mut rng)
        .unwrap()
}

fn get_ai_move_blocking(board: &[char; 9]) -> usize {
    let wins = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for &mark in &[PLAYER_O, PLAYER_X] {
        for line in &wins {
            let cells = [board[line[0]], board[line[1]], board[line[2]]];
            let count_mark = cells.iter().filter(|&&c| c == mark).count();
            let empties: Vec<usize> = line
                .iter()
                .cloned()
                .filter(|&i| board[i] != PLAYER_X && board[i] != PLAYER_O)
                .collect();
            if count_mark == 2 && !empties.is_empty() {
                return empties[0];
            }
        }
    }

    get_ai_move_random(board)
}

// ======================================
// GAME LOGIC
// ======================================

fn check_winner(board: &[char; 9]) -> Option<(char, [usize; 3])> {
    let wins = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
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
