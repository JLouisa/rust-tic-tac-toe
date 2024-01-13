use colored::*;
use std::io;

#[cfg(not(target_os = "windows"))]
fn clear_terminal() {
    std::process::Command::new("clear").status().unwrap();
}

#[cfg(target_os = "windows")]
fn clear_terminal() {
    std::process::Command::new("cmd")
        .args(["/C", "cls"])
        .status()
        .unwrap();
}

fn game_board_map(arr: &Vec<Vec<String>>) {
    println!("   1     2     3  ");
    println!("      |     |     ");
    println!("A  {}  |  {}  |  {}  ", arr[0][0], arr[0][1], arr[0][2]);
    println!(" _____|_____|_____");
    println!("      |     |     ");
    println!("B  {}  |  {}  |  {}  ", arr[1][0], arr[1][1], arr[1][2]);
    println!(" _____|_____|_____");
    println!("      |     |     ");
    println!("C  {}  |  {}  |  {}  ", arr[2][0], arr[2][1], arr[2][2]);
    println!("      |     |     ");
}

fn check_winning_condition(board: &[Vec<String>], player_mark: &str) -> bool {
    // Check horizontal and vertical
    for i in 0..3 {
        if (board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] == player_mark)
            || (board[0][i] == board[1][i]
                && board[1][i] == board[2][i]
                && board[0][i] == player_mark)
        {
            return true;
        }
    }

    // Check diagonals
    if (board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] == player_mark)
        || (board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] == player_mark)
    {
        return true;
    }

    false
}

fn parse_input(input: &str) -> Option<(usize, usize)> {
    if input.len() != 2 {
        return None;
    }

    let mut chars = input.chars();
    let first_char = chars.next()?;
    let second_char = chars.next()?;

    let row = match first_char {
        'a' | '1' => 0,
        'b' | '2' => 1,
        'c' | '3' => 2,
        _ => return None,
    };

    let col = match second_char {
        '1' | 'a' => 0,
        '2' | 'b' => 1,
        '3' | 'c' => 2,
        _ => return None,
    };

    Some((row, col))
}

fn update_board(row: &mut Vec<Vec<String>>, pos: (usize, usize), mark: String) -> bool {
    if row[pos.0][pos.1] != " " {
        duplication_check();
        false
    } else {
        edit_map(&mut row[pos.0][pos.1], mark);
        true
    }
}

#[derive(Debug)]
enum PlayerMark {
    X,
    O,
}
impl PlayerMark {
    fn as_str(&self) -> &str {
        match self {
            PlayerMark::X => "X",
            PlayerMark::O => "O",
        }
    }
}
#[derive(Debug)]
struct Players {
    name: String,
    mark: PlayerMark,
}
impl Players {
    fn new(name: String, mark: PlayerMark) -> Self {
        Self { name, mark }
    }
}

#[derive(PartialEq)]
enum Turn {
    PlayerOne,
    PlayerTwo,
}

struct Game {
    player_turn: Turn,
}

fn player_turn(player: &Players) {
    print!("{} ", player.name.cyan());
    println!("it's your turn, please enter a coordinate in number like a2",)
}

fn switch_turn(game: &mut Game) {
    if game.player_turn == Turn::PlayerOne {
        return game.player_turn = Turn::PlayerTwo;
    } else {
        return game.player_turn = Turn::PlayerOne;
    }
}

fn edit_map(row: &mut String, mark: String) {
    *row = mark;
}

fn duplication_check() {
    println!("{}", "Square already used, please try again".red());
}

//* Main
fn main() {
    // Setup GameBoard
    let mut row = vec![vec![String::from(" "); 3]; 3];

    let mut game = Game {
        player_turn: Turn::PlayerOne,
    };

    // Setup Players
    println!("Welcome to Tic Tac Toe made with Rust!");

    let mut player1_name = String::new();
    let mut player2_name = String::new();

    println!("Please enter player 1's name");
    io::stdin()
        .read_line(&mut player1_name)
        .expect("Failed to read line");
    if player1_name.trim() == "" {
        player1_name = "Player 1".to_owned()
    }
    let player1 = Players::new(player1_name.trim().to_owned(), PlayerMark::X);

    println!("Please enter player 2's name");
    io::stdin()
        .read_line(&mut player2_name)
        .expect("Failed to read line");
    if player2_name.trim() == "" {
        player2_name = "Player 2".to_owned()
    }
    let player2 = Players::new(player2_name.trim().to_owned(), PlayerMark::O);

    // Clear Terminal before game begins
    clear_terminal();

    //* Game logic
    loop {
        // Create player Input
        let mut player_input = String::new();

        // Players Turn
        let player_next = match game.player_turn {
            Turn::PlayerOne => &player1,
            Turn::PlayerTwo => &player2,
        };

        // Draw the first game board in console
        game_board_map(&row);

        // Setup player turn logic
        match game.player_turn {
            Turn::PlayerOne => player_turn(&player1),
            Turn::PlayerTwo => player_turn(&player2),
        }

        // Ask player input
        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read line");
        let player_input = player_input.trim();

        // Check if player input is correct
        let correct_input = match parse_input(&player_input) {
            Some(position) => {
                clear_terminal();
                update_board(&mut row, position, player_next.mark.as_str().to_owned())
            }
            None => {
                clear_terminal();
                println!("{}", "Invalid input, please try again".red());
                false
            }
        };

        // Check if the game is won
        match check_winning_condition(&row, player_next.mark.as_str()) {
            true => {
                game_board_map(&row);
                println!("{} {}", &player_next.name.green(), "won the game!".green());
                break;
            }
            false => {
                if correct_input {
                    switch_turn(&mut game);
                }
            }
        };
    }
}
