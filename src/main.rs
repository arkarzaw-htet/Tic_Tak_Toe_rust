use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
};
use std::io::{stdout, Result, Stdout, Write};

fn main() -> Result<()> {
    let mut board: [char; 9] = ['1','2','3','4','5','6','7','8','9'];
    let mut current_player = 'X';

    enable_raw_mode()?;
    let mut stdout = stdout();

    loop {
        draw_board(&board, &mut stdout)?;

        queue!(stdout, Print(format!("Player {}, enter position (1-9): ", current_player)))?;
        stdout.flush()?;

        let pos = loop {
            if let Event::Key(event) = read()? {
                if let KeyCode::Char(c) = event.code {
                    if let Some(digit) = c.to_digit(10) {
                        let idx = (digit - 1) as usize;
                        if idx < 9 && board[idx] != 'X' && board[idx] != 'O' {
                            break idx;
                        }
                    }
                }
            }
            queue!(stdout, Print("\nInvalid input or cell occupied. Try again: "))?;
            stdout.flush()?;
        };

        board[pos] = current_player;

        if let Some(winner) = check_winner(&board) {
            draw_board(&board, &mut stdout)?;
            queue!(stdout, Print(format!("\nPlayer {} wins! 🎉\n", winner)))?;
            break;
        }

        if board.iter().all(|&c| c == 'X' || c == 'O') {
            draw_board(&board, &mut stdout)?;
            queue!(stdout, Print("\nIt's a draw!\n"))?;
            break;
        }

        current_player = if current_player == 'X' { 'O' } else { 'X' };
    }

    disable_raw_mode()?;
    Ok(())
}
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


fn check_winner(board: &[char; 9]) -> Option<char> {
    let wins = [
        [0,1,2], [3,4,5], [6,7,8], // horizontal
        [0,3,6], [1,4,7], [2,5,8], // vertical
        [0,4,8], [2,4,6],          // diagonal
    ];
    println!("hello");

    for &line in &wins {
        if board[line[0]] == board[line[1]] &&
           board[line[1]] == board[line[2]] &&
           (board[line[0]] == 'X' || board[line[0]] == 'O') {
            return Some(board[line[0]]);
        }
    }
    None
}
