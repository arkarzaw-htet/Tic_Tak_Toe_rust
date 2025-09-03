
# 🎮 Tic-Tac-Toe  (Rust Edition)

A console-based Tic-Tac-Toe game built in **Rust 1.70+**, featuring:
- Player vs Player mode
- Player vs AI with three difficulty levels (Easy, Hard/Minimax)
- Scoreboard that tracks wins, losses, and draws
- Highlighted winning cells
- Replay option after each round

---

## 🚀 Features
- **Multiple Game Modes**  
  - PvP: Two human players  
  - PvAI: Human vs AI (Easy, Hard)  

- **AI Levels**  
  - Random → Picks any available move  
  - Easy → Blocks immediate threats, otherwise random  
  - Hard → Perfect minimax algorithm (cannot be beaten)  

- **Gameplay Enhancements**  
  - Scoreboard persists for session  
  - Highlighted winning line  
  - Replay system  

---

## 🛠️ Installation

Make sure you have **Rust (>=1.70)** installed.  
If not, install via [rustup](https://rustup.rs).

Clone this repository and build:

```bash
git clone https://github.com/arkarzaw-htet/Tic_Tak_Toe_rust.git
cd Tic_Tac_Toe_rust
cargo run
````

---

## 🎮 How to Play

1. Run the game with `cargo run`.
2. Choose your game mode from the menu.
3. Players take turns entering positions `1-9`:

```
1 | 2 | 3
---------
4 | 5 | 6
---------
7 | 8 | 9
```

4. Win by getting **three in a row** horizontally, vertically, or diagonally.
5. After the game ends, choose to replay or exit.

---

## 📦 Dependencies

* [`crossterm`](https://crates.io/crates/crossterm) → terminal control & styling
* [`rand`](https://crates.io/crates/rand) → random move generation

## 📊 Scoreboard

The scoreboard keeps track of:

* Player 1 Wins
* Player 2 / AI Wins
* Draws

Displayed after each match.

---

## 📹 Demo

A gameplay video is included in the final submission.

---

## ✨ Author

* **Arkar Zaw Htet**

---

## 📜 License

This project is for educational purposes (Mini Rust Game Project).
Free to use and modify.


