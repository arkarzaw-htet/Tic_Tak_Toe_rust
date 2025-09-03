// ======================================
// Tic Tac Toe — Crossterm (Minimized)
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

const WIN_LINES: [[usize; 3]; 8] = [
    [0, 1, 2], [3, 4, 5], [6, 7, 8],
    [0, 3, 6], [1, 4, 7], [2, 5, 8],
    [0, 4, 8], [2, 4, 6],
];

#[derive(Clone, Copy, PartialEq)]
enum Difficulty { Easy, Hard }

#[derive(Clone, Copy)]
enum GameMode { Friend, AI(Difficulty, bool) } // (difficulty, player_first)

// ======================================
// MAIN
// ======================================

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    let (mut score_x, mut score_o, mut score_d) = (0, 0, 0);

    loop {
        let mut board = EMPTY_CELLS;
        let mut current = PLAYER_X;

        show_welcome(&mut stdout)?;
        let mode = ask_game_mode(&mut stdout)?;

        loop {
            draw_board(&board, &mut stdout, None)?;
            print_turn_hint(&mut stdout, mode, current)?;

            let pos = if is_human_turn(mode, current) {
                get_human_move(&mut stdout, &board)?
            } else {
                let ai_mark = if let GameMode::AI(_, player_first) = mode {
                    if player_first { PLAYER_O } else { PLAYER_X }
                } else { unreachable!() };
                match mode {
                    GameMode::AI(diff, _) => get_ai_move(&board, ai_mark, diff),
                    _ => unreachable!(),
                }
            };

            board[pos] = current;

            if let Some((winner, line)) = check_winner(&board) {
                draw_board(&board, &mut stdout, Some(&line))?;
                print_message(&mut stdout, &format!("Player {} wins! 🎉", winner))?;
                if winner == PLAYER_X { score_x += 1 } else { score_o += 1 }
                break;
            }

            if is_draw(&board) {
                draw_board(&board, &mut stdout, None)?;
                print_message(&mut stdout, "It's a draw! 🤝")?;
                score_d += 1;
                break;
            }

            current = switch_player(current);
        }

        print_message(&mut stdout, &format!("Score => X: {} | O: {} | Draws: {}", score_x, score_o, score_d))?;
        if !ask_yesno(&mut stdout, "Play again? (y/n): ")? { break; }
    }

    disable_raw_mode()?;
    Ok(())
}

// ======================================
// GAME FLOW
// ======================================

fn show_welcome(stdout: &mut Stdout) -> Result<()> {
    execute!(stdout, Clear(ClearType::All))?;
    queue!(stdout, MoveTo(8, 0), Print("==== Welcome to Tic Tac Toe ===="))?;
    queue!(stdout, MoveTo(8, 2), Print("Controls: press 1–9 to place your mark."))?;
    queue!(stdout, MoveTo(8, 3), Print("Win by getting 3 in a row."))?;
    queue!(stdout, MoveTo(8, 5), Print("Press any key to continue..."))?;
    stdout.flush()?;
    read()?;
    Ok(())
}

fn ask_choice(stdout: &mut Stdout, prompt: &str, valid: &[(char, char)]) -> Result<char> {
    queue!(stdout, MoveTo(0, 10), Print(prompt))?;
    stdout.flush()?;
    loop {
        if let Event::Key(event) = read()? {
            if let KeyCode::Char(c) = event.code {
                for &(opt, ret) in valid {
                    if c.eq_ignore_ascii_case(&opt) { return Ok(ret); }
                }
            }
        }
        queue!(stdout, Print("\nInvalid input. Try again: "))?;
        stdout.flush()?;
    }
}

fn ask_game_mode(stdout: &mut Stdout) -> Result<GameMode> {
    let c = ask_choice(stdout, "Friend (f) or AI (a)? ", &[('f','f'),('a','a')])?;
    if c == 'f' { return Ok(GameMode::Friend); }
    let d = ask_choice(stdout, "Difficulty (e=easy, h=hard): ", &[('e','e'),('h','h')])?;
    let diff = if d=='e' { Difficulty::Easy } else { Difficulty::Hard };
    let first = ask_choice(stdout, "Who first? (p=player, c=computer): ", &[('p','p'),('c','c')])?;
    Ok(GameMode::AI(diff, first=='p'))
}

fn ask_yesno(stdout: &mut Stdout, prompt: &str) -> Result<bool> {
    let c = ask_choice(stdout, prompt, &[('y','y'),('n','n')])?;
    Ok(c=='y')
}

fn is_human_turn(mode: GameMode, current: char) -> bool {
    match mode {
        GameMode::Friend => true,
        GameMode::AI(_, player_first) => if player_first { current==PLAYER_X } else { current==PLAYER_O }
    }
}

fn switch_player(c: char) -> char { if c==PLAYER_X { PLAYER_O } else { PLAYER_X } }
fn is_draw(board: &[char;9]) -> bool { board.iter().all(|&c| c==PLAYER_X || c==PLAYER_O) }

// ======================================
// DRAWING
// ======================================

fn draw_board(board: &[char; 9], stdout: &mut Stdout, highlight: Option<&[usize; 3]>) -> Result<()> {
    execute!(stdout, Clear(ClearType::All))?;
    queue!(stdout, MoveTo(0, 0), Print("Tic Tac Toe"))?;

    for row in 0..3 {
        if row > 0 { queue!(stdout, MoveTo(0, row as u16 * 2 + 2), Print("---+---+---"))?; }
        let y = row as u16 * 2 + 3;
        for col in 0..3 {
            let idx = row*3+col;
            let x = [1,5,9][col];
            let mark = board[idx];
            if highlight.map_or(false, |h| h.contains(&idx)) {
                queue!(stdout, MoveTo(x, y), SetForegroundColor(Color::Green), Print(mark), ResetColor)?;
            } else {
                queue!(stdout, MoveTo(x, y), Print(mark))?;
            }
            if col<2 { queue!(stdout, MoveTo([3,7][col], y), Print("|"))?; }
        }
    }
    stdout.flush()?;
    Ok(())
}

fn print_turn_hint(stdout: &mut Stdout, mode: GameMode, current: char) -> Result<()> {
    let msg = match mode {
        GameMode::Friend => format!("Player {}'s turn (1-9): ", current),
        GameMode::AI(_, first) => {
            if is_human_turn(mode, current) {
                let mark = if first { PLAYER_X } else { PLAYER_O };
                format!("Your turn ({}). Enter 1-9: ", mark)
            } else {
                let mark = if first { PLAYER_O } else { PLAYER_X };
                format!("Computer's turn ({})...", mark)
            }
        }
    };
    queue!(stdout, MoveTo(0, 10), Print(msg))?;
    stdout.flush()?;
    Ok(())
}

fn print_message(stdout: &mut Stdout, msg: &str) -> Result<()> {
    queue!(stdout, MoveTo(0, 12), Print(format!("\n{}\n", msg)))?;
    stdout.flush()?;
    Ok(())
}

// ======================================
// INPUT
// ======================================

fn get_human_move(stdout: &mut Stdout, board: &[char;9]) -> Result<usize> {
    use crossterm::cursor::MoveToNextLine;
    stdout.flush()?;
    loop {
        if let Event::Key(event) = read()? {
            if let KeyCode::Char(c) = event.code {
                if let Some(d) = c.to_digit(10) {
                    let idx = (d-1) as usize;
                    if idx<9 && board[idx]!=PLAYER_X && board[idx]!=PLAYER_O {
                        return Ok(idx);
                    }
                }
            }
        }
        queue!(stdout, MoveToNextLine(1), Print("Invalid. Try again: "))?;
        stdout.flush()?;
    }
}

// ======================================
// AI
// ======================================

fn get_ai_move(board: &[char;9], ai_mark: char, diff: Difficulty) -> usize {
    match diff {
        Difficulty::Easy => get_ai_move_random(board),
        Difficulty::Hard => {
            let opp = if ai_mark==PLAYER_X { PLAYER_O } else { PLAYER_X };
            for &mark in &[ai_mark, opp] {
                for line in WIN_LINES {
                    let cells = [board[line[0]], board[line[1]], board[line[2]]];
                    if cells.iter().filter(|&&c| c==mark).count()==2 {
                        if let Some(&i) = line.iter().find(|&&i| board[i]!=PLAYER_X && board[i]!=PLAYER_O) {
                            return i;
                        }
                    }
                }
            }
            get_ai_move_random(board)
        }
    }
}

fn get_ai_move_random(board: &[char;9]) -> usize {
    let mut rng = rand::thread_rng();
    board.iter().enumerate()
        .filter(|&(_, &c)| c!=PLAYER_X && c!=PLAYER_O)
        .map(|(i, _)| i)
        .choose(&mut rng).unwrap()
}

// ======================================
// GAME LOGIC
// ======================================

fn check_winner(board: &[char;9]) -> Option<(char, [usize;3])> {
    for &line in &WIN_LINES {
        if board[line[0]]==board[line[1]] && board[line[1]]==board[line[2]]
            && (board[line[0]]==PLAYER_X || board[line[0]]==PLAYER_O) {
            return Some((board[line[0]], line));
        }
    }
    None
}
