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

fn game_winning(arr: &Vec<Vec<String>>, mark: &Players) -> bool {
    // Horizontal check
    if arr[0][0] == arr[0][1] && arr[0][1] == arr[0][2] && arr[0][2] == mark.mark.as_str() {
        return true;
    };
    if arr[1][0] == arr[1][1] && arr[1][1] == arr[1][2] && arr[1][2] == mark.mark.as_str() {
        return true;
    };
    if arr[2][0] == arr[2][1] && arr[2][1] == arr[2][2] && arr[2][2] == mark.mark.as_str() {
        return true;
    };

    //Vertical check
    if arr[0][0] == arr[1][0] && arr[1][0] == arr[2][0] && arr[2][0] == mark.mark.as_str() {
        return true;
    };
    if arr[0][1] == arr[1][1] && arr[1][1] == arr[2][1] && arr[2][1] == mark.mark.as_str() {
        return true;
    };
    if arr[0][2] == arr[1][2] && arr[1][2] == arr[2][2] && arr[2][2] == mark.mark.as_str() {
        return true;
    };

    //Diagonal check
    if arr[0][0] == arr[1][1] && arr[1][1] == arr[2][2] && arr[2][2] == mark.mark.as_str() {
        return true;
    };
    if arr[0][2] == arr[1][1] && arr[1][1] == arr[2][0] && arr[2][0] == mark.mark.as_str() {
        return true;
    };

    return false;
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
    println!(
        "{} it's your turn, please enter a coordinate in number like a2",
        player.name
    )
}

fn switch_turn(game: &mut Game) {
    if game.player_turn == Turn::PlayerOne {
        return game.player_turn = Turn::PlayerTwo;
    } else {
        return game.player_turn = Turn::PlayerOne;
    }
}

fn duplication_check() -> bool {
    println!("Square already used, please try again");
    false
}

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

    println!("Player1: {}, Player 2: {}", player1.name, player2.name);

    loop {
        let mut player_input = String::new();
        // First Player start
        match game.player_turn {
            Turn::PlayerOne => player_turn(&player1),
            Turn::PlayerTwo => player_turn(&player2),
        }

        // Players Turn
        let player_next = match game.player_turn {
            Turn::PlayerOne => &player1,
            Turn::PlayerTwo => &player2,
        };

        game_board_map(&row);
        player_turn(&player_next);

        //Dev
        println!("Dev: Turn of {}", player_next.name);

        // Ask input
        io::stdin()
            .read_line(&mut player_input)
            .expect("Failed to read line");
        let player_input = player_input.trim();

        // Find correct coord
        let correct_input: bool = match player_input {
            // "a1" =>  edit_map(&player_next, &row[0][0]),
            "a1" | "1a" => {
                if row[0][0] != " " {
                    duplication_check()
                } else {
                    row[0][0] = player_next.mark.as_str().to_owned();
                    true
                }
            }
            "a2" | "2a" => {
                if row[0][1] != " " {
                    duplication_check()
                } else {
                    row[0][1] = player_next.mark.as_str().to_owned();
                    true
                }
            }
            "a3" | "3a" => {
                if row[0][2] != " " {
                    duplication_check()
                } else {
                    row[0][2] = player_next.mark.as_str().to_owned();
                    true
                }
            }
            "b1" | "1b" => {
                if row[1][0] != " " {
                    duplication_check()
                } else {
                    row[1][0] = player_next.mark.as_str().to_owned();
                    true
                }
            }
            "b2" | "2b" => {
                if row[1][1] != " " {
                    duplication_check()
                } else {
                    row[1][1] = player_next.mark.as_str().to_owned();
                    true
                }
            }
            "b3" | "3b" => {
                if row[1][2] != " " {
                    duplication_check()
                } else {
                    row[1][2] = player_next.mark.as_str().to_owned();
                    true
                }
            }
            "c1" | "1c" => {
                if row[2][0] != " " {
                    duplication_check()
                } else {
                    row[2][0] = player_next.mark.as_str().to_owned();
                    true
                }
            }
            "c2" | "2c" => {
                if row[2][1] != " " {
                    duplication_check()
                } else {
                    row[2][1] = player_next.mark.as_str().to_owned();
                    true
                }
            }
            "c3" | "3c" => {
                if row[2][2] != " " {
                    duplication_check()
                } else {
                    row[2][2] = player_next.mark.as_str().to_owned();
                    true
                }
            }
            _ => {
                println!("Invalid input, please try again");
                false
            }
        };
        let game_won = game_winning(&row, player_next);
        if game_won {
            game_board_map(&row);
            println!("{} won the game", &player_next.name);
            break;
        }
        if correct_input {
            switch_turn(&mut game);
            clear_terminal();
        }
    }
}
