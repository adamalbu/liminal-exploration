use std::{fmt, io::Write};

use map::{Direction, Map, Room};
use user_input::get_user_input;

mod map;
mod user_input;

#[allow(dead_code)]
fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    if v1.len() != v2.len() {
        return 0.0; // Vectors must be of the same dimension
    }

    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    let magnitude1: f32 = v1.iter().map(|a| a * a).sum::<f32>().sqrt();
    let magnitude2: f32 = v2.iter().map(|a| a * a).sum::<f32>().sqrt();

    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        return 0.0;
    }

    dot_product / (magnitude1 * magnitude2)
}

enum Command {
    Travel(Direction),
}

enum ParseCommandError {
    NoCommandPresent,
    UnknownCommand(String),
    IncorrectArgument(String),
    NotEnoughArguments(u8),
}

impl std::fmt::Display for ParseCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseCommandError::IncorrectArgument(arg) => write!(
                f,
                "Incorrect argument '{}'. Please enter a correct argument.",
                arg
            ),
            ParseCommandError::NoCommandPresent => write!(f, "Please enter a command."),
            ParseCommandError::NotEnoughArguments(num) => write!(
                f,
                "Not enough arguments. Please enter {} argument{}.",
                num,
                if *num == 1 { "" } else { "s" }
            ),
            ParseCommandError::UnknownCommand(command) => write!(
                f,
                "Command '{}' not found. Please enter a valid command.",
                command
            ),
        }
    }
}

impl Command {
    fn parse_command(input_string: String) -> Result<Self, ParseCommandError> {
        let input_lowercase = input_string.to_lowercase();
        let mut parts = input_lowercase.split_whitespace();

        let command_str = parts.next().ok_or(ParseCommandError::NoCommandPresent)?;

        match command_str {
            "go" | "travel" => {
                let direction_str = parts
                    .next()
                    .ok_or(ParseCommandError::NotEnoughArguments(1))?;
                let direction = match direction_str {
                    "north" => Direction::North,
                    "east" => Direction::East,
                    "south" => Direction::South,
                    "west" => Direction::West,
                    _ => return Err(ParseCommandError::IncorrectArgument(direction_str.into())),
                };
                Ok(Self::Travel(direction))
            }
            _ => Err(ParseCommandError::UnknownCommand(command_str.into())),
        }
    }
}

fn main() {
    let root_room = Room::new_random_with_entry(
        "The room is an endless maze of peeling, \
       yellowed wallpaper and damp, stained carpet, all under the harsh, unblinking glare of a \
       humming fluorescent light. There are a few scattered electrical outlets, but no other \
       resources to be found."
            .into(),
        Direction::North,
    );
    let mut map = Map::new(root_room);
    println!("Welcome to Liminal Exploration!");
    loop {
        println!("{}", map.get_current_room().get_info());
        print!("> ");
        std::io::stdout().flush().unwrap();

        let input = get_user_input().to_lowercase();

        let parsed_command = match Command::parse_command(input) {
            Ok(command) => command,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        match parsed_command {
            Command::Travel(direction) => match map.travel(direction) {
                Ok(_) => println!("You went {}", direction),
                Err(err) => println!("{}", err),
            },
        }
    }
}
