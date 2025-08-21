use crate::map::{Direction, Exits};

pub trait Room {
    fn get_exits(&self) -> Exits;
    fn get_info(&self) -> String;
    fn new(description: String, exits: Exits) -> Self;
    fn new_random_with_entry(description: String, entry_direction: Direction) -> Self;
}
