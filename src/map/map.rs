use crate::map::{Direction, Room};
use petgraph::{
    Directed, Graph,
    graph::{EdgeReference, NodeIndex},
    visit::EdgeRef,
};

#[derive(Debug)]
pub struct Map<T: Room> {
    graph: Graph<T, Direction, Directed>,
    pub current_room_id: NodeIndex,
}

#[derive(Clone, Copy, Debug)]
pub enum TravelError {
    NoExit,
}

impl std::fmt::Display for TravelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoExit => write!(f, "No exit"),
        }
    }
}

impl<T: Room> Map<T> {
    pub fn new(root_room: T) -> Self {
        let mut graph = Graph::<T, Direction, Directed>::new();
        let root_room_id = graph.add_node(root_room);
        Self {
            graph,
            current_room_id: root_room_id,
        }
    }

    pub fn travel(&mut self, direction: Direction) -> Result<String, TravelError> {
        let current_room = self.get_current_room();
        if current_room.get_exits().contains(direction.into()) {
            let new_room_id = match self.get_edge(self.current_room_id, direction) {
                Some(edge_ref) => edge_ref.target(),
                None => {
                    let new_room_id = self.generate_room(direction);
                    self.connect_rooms(self.current_room_id, new_room_id, direction);
                    new_room_id
                }
            };
            self.current_room_id = new_room_id;
            Ok(self.get_current_room().get_info())
        } else {
            Err(TravelError::NoExit)
        }
    }

    fn generate_room(&mut self, entry_direction: Direction) -> NodeIndex {
        let new_room = Room::new_random_with_entry("[generated room]".into(), entry_direction); // TODO: Create actual description
        self.graph.add_node(new_room)
    }

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

    #[allow(dead_code)]
    pub fn get_room(&self, room_id: NodeIndex<u32>) -> &T {
        &self.graph[room_id]
    }

    pub fn get_current_room(&self) -> &T {
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
