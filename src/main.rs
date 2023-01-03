use std::collections::{BTreeMap, HashMap};
use std::fmt::format;
use std::io::{self, BufRead};
use std::ops::Add;
use uuid::Uuid;

struct Player {
    id: String,
    name: String,
    ents: i32,
}

struct Settings {
    max_number_on_dice: i32,
    start_ents: i32,
}

struct Board {
    fields: BTreeMap<i32, i32>,
}

impl ToString for Board {
    fn to_string(&self) -> String {
        let mut result = Vec::new();
        for field in &self.fields {
            result.push(format!("{}: {}", field.0, field.1))
        }
        return result.join("\n");
    }
}

impl Board {
    pub fn initialize(dice_size:i32) -> Self{
        let mut map = BTreeMap::new();
        for i in 1..dice_size {
            map.insert(i, 0);
        }
        return Board{
            fields: map
        }
    }
}

struct Throw {
    result: i32,
}

struct Turn {
    current_player: Player,
    trows: Vec<Throw>,
}

struct Game {
    players: Vec<Player>,
    settings: Settings,
    board: Board,
    current_turn: Turn,
}

fn main() {

    let number_of_ents = ask_for_number_input("How many pigs should the players have at start? (default is 10)", 10);
    let dice_size = ask_for_number_input("How big should the dice be? (default is 6)", 6);
    let number_of_players = ask_for_number_input("How many players want to play? (default is 2)", 2);
    println!("You choose to start with {} players", number_of_players);
    let players = enter_player_names(number_of_players, number_of_ents);
    println!(
        "Welcome {}",
        players
            .into_iter()
            .map(|p| p.name)
            .collect::<Vec<String>>()
            .join(", ")
    );
    let board = Board::initialize(dice_size);
    println!("{}", board.to_string());
}

fn ask_for_number_input(text: &str, default: i32) -> i32{
    println!("{}", text);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        if input.len() == 0 {
            return default;
        }
        match input.parse::<i32>() {
            Ok(number) => return number,
            Err(e) => println!("Please enter a number!"),
        }
    }
    return -1
}

fn enter_player_names(number_of_players: i32, number_of_ents: i32) -> Vec<Player> {
    let mut players: Vec<Player> = vec![];
    for p in 1..number_of_players + 1 {
        let player_name = enter_player_name(p);
        players.push(Player {
            name: player_name,
            ents: number_of_ents,
            id: Uuid::new_v4().to_string(),
        })
    }
    return players;
}

fn enter_player_name(number: i32) -> String {
    println!("Please enter your name, Player{}", number);
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let name = line.unwrap();
        if name.len() > 0 {
            return name;
        }
        println!("Please enter your name, Player{}", number);
    }
    return String::from("");
}
