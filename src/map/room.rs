use crate::map::{Direction, Exits};

#[derive(Debug)]
pub struct Room {
    pub description: String,
    pub exits: Exits,
}

impl Room {
    pub fn get_info(&self) -> String {
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
}

impl std::fmt::Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_info())
    }
}

impl Room {
    pub fn new(description: String, exits: Exits) -> Self {
        Self { description, exits }
    }

    pub fn new_random_with_entry(description: String, entry_direction: Direction) -> Self {
        let random_num = rand::random::<f32>();

        let exits = if random_num > 0.8 {
            Exits::all_2d()
        } else if random_num > 0.4 {
            let mut exits: Exits = entry_direction.opposite().into();
            exits |= Exits::random_exits(2, Exits::all_2d() & !exits);
            exits
        } else if random_num > 0.2 {
            let mut exits: Exits = entry_direction.opposite().into();
            exits |= Exits::random_exits(1, Exits::all_2d() & !exits);
            exits
        } else {
            entry_direction.opposite().into()
        };

        Self::new(description, exits)
    }
}
