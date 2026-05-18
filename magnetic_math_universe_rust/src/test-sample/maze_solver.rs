use std::collections::HashMap;

#[derive(Debug)]
struct Node {
    id: i32,
    value: i32,
    is_even: bool, // True = North (Stable), False = South (Volatile)
}

#[derive(Debug, Clone)]
struct Edge {
    from_id: i32,
    to_id: i32,
}

struct MagneticGraph {
    nodes: HashMap<i32, Node>,
    edges: Vec<Edge>,
}

impl MagneticGraph {
    fn new() -> Self {
        MagneticGraph {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    // Yanha node add hone se pahle khud check karega ki EVEN h ya OOD
    fn add_node(&mut self, id: i32, value: i32) {
        let is_even = value % 2 == 0;
        self.nodes.insert(id, Node { id, value, is_even });
    }

    fn add_edge(&mut self, from_id: i32, to_id: i32) {
        self.edges.push(Edge { from_id, to_id });
    }

    // 2. Magic fun  EMP Collapse (P=NP Simulator)
    fn trigger_magnetic_collapse(&mut self) {
        println!("\n WARNING: Triggering System-Wide Magnetic Collapse! ⚡\n");

        let mut surviving_edges = Vec::new();

        // System saare edges (pointers) ko ek sath scan karega
        for edge in &self.edges {
            let node_from = self.nodes.get(&edge.from_id).unwrap();
            let node_to = self.nodes.get(&edge.to_id).unwrap();

            // RULE 10.3: THE CLASH (Short Circuit)
            if node_from.is_even != node_to.is_even {
                println!("Short Circuit edge between Node {} (Even:{}) and Node {} (Even:{}) destroyed",
                         node_from.id, node_from.is_even, node_to.id, node_to.is_even);
                // Edge surviving_edges array mein nahi jayegi means delete ho gaya
            } else {
                println!("Stable tether edge between Node {} and Node {} remains safe.",
                         node_from.id, node_to.id);
                surviving_edges.push(edge.clone()); // Edge bach gayi
            }
        }

        // Purani edges ko nayi filter hui edges se replace kar kiya
        self.edges = surviving_edges;
        println!("\nCollapse complete Graph stabilized.");
    }

    fn print_paths(&self) {
        for edge in &self.edges {
            println!("Node {} ---> Node {}", edge.from_id, edge.to_id);
        }
    }
}

pub(crate) fn main() {
    let mut maze = MagneticGraph::new();

    // ---------------------------------------------------------
    // Problem setup THE MAZE
    // Right way (Even nodes): 2 -> 4 -> 6 -> 8
    // Wrong way 1 (Odd nodes): 4 -> 3 -> 5
    // Wrong way 2 (Odd nodes): 6 -> 7 -> 9
    // ---------------------------------------------------------

    // Insert right Nodes (Even)
    maze.add_node(1, 2); // Start
    maze.add_node(2, 4);
    maze.add_node(3, 6);
    maze.add_node(4, 8); // Exit

    // Insert Wrong Nodes (Odd / Dead-ends)
    maze.add_node(5, 3);
    maze.add_node(6, 5);
    maze.add_node(7, 7);
    maze.add_node(8, 9);

    // make here Edges (Pointers)
    // right way
    maze.add_edge(1, 2);
    maze.add_edge(2, 3);
    maze.add_edge(3, 4);

    // Wrong way graph se jodte hain
    maze.add_edge(2, 5); // 4 se 3 (Even to Odd!)
    maze.add_edge(5, 6);
    maze.add_edge(3, 7); // 6 se 7 (Even to Odd!)
    maze.add_edge(7, 8);

    println!("--- Maze before MAGNETIC collapse (Total Chaos) ---");
    maze.print_paths();

    // **********************************************************************************
    // Algo execution  (The P=NP Magic)
    // *************************************************************************
    maze.trigger_magnetic_collapse();

    println!("\n--- Maze after MAGNETIC collapse (The True Path) ---");
    maze.print_paths();
}