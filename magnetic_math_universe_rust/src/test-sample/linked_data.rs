struct Node {
    id: i32,
    value: i32,
    is_even: bool, // True = North (right way), False = South (wrong way)
}

struct Edge {
    from_id: i32,
    to_id: i32,
}

struct MagneticGraph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl MagneticGraph {
    // 2. magic funtion The EMP Collapse
    fn trigger_magnetic_collapse(&mut self) {
        println!("WARNING: Triggering System-Wide Magnetic Collapse!");

        let mut surviving_edges: Vec<Edge> = Vec::new();

        // Har pointer (edge) ko check karte h pahle
        for edge in &self.edges {
            let node_from = self.get_node(edge.from_id);
            let node_to = self.get_node(edge.to_id);

            // RULE 10.3: THE CLASH (Short Circuit)
            if node_from.is_even != node_to.is_even {
                // Polarity alag hai Connection blast ho gaya.
                println!("Short Circuit edge between {} and {} destroyed!", edge.from_id, edge.to_id);
                // Ise surviving_edges mein add nahi karenge (delete ho gaya)
            } else {
                // Polarity same hai (Even to Even). Connection safe hai.
                println!("table tether edge between {} and {} is safe.", edge.from_id, edge.to_id);
                surviving_edges.push(Edge { from_id: edge.from_id, to_id: edge.to_id });
            }
        }

        // Puraani edges ko nayi (filter hui) edges se replace kar do
        self.edges = surviving_edges;
        println!("Collapse complete only the true path remains.");
    }

    // Helper function to find node by id
    fn get_node(&self, id: i32) -> &Node {
        self.nodes.iter().find(|&n| n.id == id).unwrap()
    }
}