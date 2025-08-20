use bitflags::bitflags;
use petgraph::{
    Directed, Graph,
    graph::{EdgeReference, NodeIndex},
    visit::EdgeRef,
};
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Debug)]
pub struct Room {
    pub description: String,
    pub exits: Exits,
}

impl std::fmt::Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

        write!(f, "{}\nExits are {}", self.description, exits_str)
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
        } else if random_num > 0.65 {
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

#[derive(Debug)]
pub struct Map {
    graph: Graph<Room, Direction, Directed>,
    pub current_room_id: NodeIndex,
}

#[derive(Clone, Copy, Debug)]
enum TravelError {
    NoExit,
}

enum GetEdgeError {
    NoEdge,
}

impl Map {
    pub fn new(root_room: Room) -> Self {
        let mut graph = Graph::<Room, Direction, Directed>::new();
        let root_room_id = graph.add_node(root_room);
        Self {
            graph,
            current_room_id: root_room_id,
        }
    }

    pub fn travel(&mut self, direction: Direction) -> Result<(), TravelError> {
        let current_room = self.get_current_room();
        if current_room.exits.contains(direction.into()) {
            let new_room_id = match self.get_edge(self.current_room_id, direction) {
                Some(edge_ref) => edge_ref.target(),
                None => {
                    let new_room_id = self.generate_room(direction);
                    self.connect_rooms(self.current_room_id, new_room_id, direction);
                    new_room_id
                }
            };
            self.current_room_id = new_room_id;
            Ok(())
        } else {
            Err(TravelError::NoExit)
        }
    }

    fn generate_room(&mut self, entry_direction: Direction) -> NodeIndex {
        let new_room = Room::new(
            "Generated room".into(),
            Exits::WEST | entry_direction.opposite().into(),
        );
        self.graph.add_node(new_room)
    } // TODO: Implement actual generation

    fn connect_rooms(
        &mut self,
        from_room: NodeIndex<u32>,
        to_room: NodeIndex<u32>,
        direction: Direction,
    ) {
        self.graph.add_edge(from_room, to_room, direction);
        self.graph
            .add_edge(to_room, from_room, direction.opposite());
    }

    fn get_room(&self, room_id: NodeIndex<u32>) -> &Room {
        &self.graph[room_id]
    }

    fn get_current_room(&self) -> &Room {
        &self.graph[self.current_room_id]
    }

    fn get_edge(
        &self,
        room_id: NodeIndex,
        target: Direction,
    ) -> Option<EdgeReference<'_, Direction>> {
        for edge_ref in self
            .graph
            .edges_directed(room_id, petgraph::Direction::Outgoing)
        {
            let edge_weight = edge_ref.weight();
            if *edge_weight == target {
                return Some(edge_ref);
            };
        }
        None
    }
}

fn main() {
    let mut map = Map::new(Room::new("main room".into(), Exits::NORTH | Exits::EAST));
    println!("{}", map.get_current_room());
    let result = map.travel(Direction::North);
    match result {
        Ok(()) => println!("{}", map.get_current_room()),
        Err(e) => println!("{:?}", e),
    }
    let result = map.travel(Direction::South);
    match result {
        Ok(()) => println!("{}", map.get_current_room()),
        Err(e) => println!("{:?}", e),
    }
}
