use crate::map::{Direction, Exits, Room};

pub struct LevelRoom {
    pub description: String,
    pub exits: Exits,
}

impl Room for LevelRoom {
    fn get_exits(&self) -> Exits {
        self.exits
    }

    fn get_info(&self) -> String {
        let mut exit_names = Vec::new();

        // Check each bitflag and add the corresponding string to the vector.
        if self.exits.contains(Exits::NORTH) {
            exit_names.push("north");
        }
        if self.exits.contains(Exits::EAST) {
            exit_names.push("east");
        }
        if self.exits.contains(Exits::SOUTH) {
            exit_names.push("south");
        }
        if self.exits.contains(Exits::WEST) {
            exit_names.push("west");
        }

        // Join the exit names with commas, handling the last 'and' properly.
        let exits_str = match exit_names.len() {
            0 => "none".to_string(),        // No exits
            1 => exit_names[0].to_string(), // One exit
            _ => {
                let last = exit_names.pop().unwrap(); // Take the last one out
                format!("{} and {}", exit_names.join(", "), last) // Join the rest and add the last one
            }
        };

        format!("{}\nExits are {}", self.description, exits_str)
    }

    fn new(description: String, exits: Exits) -> Self {
        Self { description, exits }
    }

    fn new_random_with_entry(description: String, entry_direction: Direction) -> Self {
        let exits = Exits::random_exits_with_entry(entry_direction);

        Self::new(description, exits)
    }
}
