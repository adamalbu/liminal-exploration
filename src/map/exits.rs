use crate::map::Direction;
use bitflags::bitflags;
use rand::prelude::*;

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Exits: u8 {
        const NORTH = 1 << 0;
        const EAST = 1 << 1;
        const SOUTH = 1 << 2;
        const WEST = 1 << 3;
        const UP = 1 << 4;
        const DOWN = 1 << 5;
    }
}

impl Exits {
    pub fn all_2d() -> Self {
        Self::NORTH | Self::EAST | Self::SOUTH | Self::WEST
    }

    pub fn random_exits(num_exits: u8, mut available_exits: Self) -> Self {
        let mut generated_exits = Exits::empty();
        let mut rng = rand::rng();

        // Iterate up to the number of exits requested.
        for _ in 0..num_exits {
            if available_exits.is_empty() {
                // Stop if there are no more available exits to choose from.
                break;
            }

            // Get a list of the available individual flags.
            let available_flags: Vec<_> = available_exits.iter().collect();

            // Choose a random one.
            if let Some(chosen_flag) = available_flags.choose(&mut rng) {
                // Add the chosen flag to our generated set.
                generated_exits.insert(*chosen_flag);

                // Remove it from the available exits to ensure it's not chosen again.
                available_exits.remove(*chosen_flag);
            }
        }
        generated_exits
    }

    pub fn random_exits_with_entry(entry_direction: Direction) -> Exits {
        let random_num = rand::random::<f32>();

        if random_num > 0.8 {
            Exits::all_2d()
        } else if random_num > 0.5 {
            let mut exits: Exits = entry_direction.opposite().into();
            exits |= Exits::random_exits(2, Exits::all_2d() & !exits);
            exits
        } else if random_num > 0.15 {
            let mut exits: Exits = entry_direction.opposite().into();
            exits |= Exits::random_exits(1, Exits::all_2d() & !exits);
            exits
        } else {
            entry_direction.opposite().into()
        }
    }
}

impl From<Direction> for Exits {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::North => Self::NORTH,
            Direction::East => Self::EAST,
            Direction::South => Self::SOUTH,
            Direction::West => Self::WEST,
            Direction::Up => Self::UP,
            Direction::Down => Self::DOWN,
        }
    }
}
