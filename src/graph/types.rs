use std::collections::HashMap;

type NodeId<'a> = &'a str;
type EdgeList<'a> = Vec<NodeId<'a>>;

/// A vertex or node in a variation graph
#[derive(Debug, PartialEq, Clone)]
pub struct Node<'a> {
    // Required: the piece of sequence associated with the node. A string of alphabet A, T, C, and G.
    pub segment: &'a str,

    // Offset: When a graph is built out of a reference this is the position of the start of the segment on that reference
    offset: usize,

    // Required: Unique identifier of each node.
    // Currently, a SHA 256 hash of the concatenation of segment, “+” and offset
    // TODO: Not require an offset for de novo graphs to be built
    pub id: NodeId<'a>,

    // Optional: ID of the reference from which we got this node
    reference: &'a str,

    // Required: The edges to the right of this node
    pub nodes_right: EdgeList<'a>,

    // Required: The edges to the left of this node
    nodes_left: EdgeList<'a>,
}

impl<'a> Node<'a> {
    pub fn new(
        segment: &'a str,
        offset: usize,
        id: &'a str,
        reference: &'a str,
        nodes_right: EdgeList<'a>,
        nodes_left: EdgeList<'a>,
    ) -> Self {
        Node {
            segment,
            offset,
            id,
            reference,
            nodes_left,
            nodes_right,
        }
    }
}

// TODO: link id and node
/// A [variation graph] is a HashMap of [`id`] to [`Node`].
/// This has several advantages for us:
///  - duplicates: we get to avoid duplicates for "free"
///
/// [variation graph]: https://blog.urbanslug.com/posts/2019-06-22-Introduction-to-Variation-Graphs.html
/// [`id`]: ../../vg/graph/struct.Node.html
/// [`Node`]: ../../vg/graph/struct.Node.html
///
/// Attempting compatibility with https://github.com/vgteam/libhandlegraph
pub struct Graph<'a>(HashMap<&'a str, Node<'a>>);

impl<'a> Graph<'a> {
    // Create a new empty graph
    pub fn new() -> Graph<'a> {
        let vg: HashMap<&'a str, Node<'a>> = HashMap::new();
        Graph(vg)
    }

    pub fn hashmap(&self) -> &HashMap<&'a str, Node<'a>> {
        &self.0
    }

    // Check whether a node exists
    fn has_node(&self, id: NodeId) -> bool {
        let hashmap = &self.0;
        hashmap.contains_key(id)
    }

    // Get an immutable reference to a node
    fn get_node(&self, id: NodeId) -> Option<&Node<'a>> {
        let hashmap = &self.0;

        hashmap.get(id)
    }

    // Get a mutable reference to the node
    fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node<'a>> {
        let hashmap = &mut self.0;

        hashmap.get_mut(id)
    }

    // Functions that mutate the graph
    // ---

    // We want the graph to own its nodes.
    // TODO: why must a graph own its nodes?
    pub fn add_node(&mut self, n: Node<'a>) {
        let hashmap = &mut self.0;
        let id = n.id;
        hashmap.insert(id, n);
    }

    // Does an edge from x to y exist?
    fn edge_exists(&self, x: NodeId<'a>, y: NodeId<'a>) -> bool {
        // Does y exist in x's right nodes
        let x_right_nodes = &self.get_node(x).unwrap().nodes_right;

        // Does x exist in y's right nodes
        let y_left_nodes = &self.get_node(y).unwrap().nodes_left;

        // Check that the nodes point to each other
        x_right_nodes.contains(&y) && y_left_nodes.contains(&x)
    }

    // Add an edge from x to y
    // TODO: Use Errors and the Result type
    fn add_edge(&self, x: &mut Node<'a>, y: &mut Node<'a>) {
        if self.has_node(x.id) && self.has_node(y.id) {
            // Add the id of x to the left nodes list of y
            y.nodes_left.push(x.id);

            // Add the id of y to the right nodes list of x
            x.nodes_right.push(y.id);
        } else {
            if !self.has_node(x.id) && !self.has_node(y.id) {
                // Both x and y aren't in the graph
                panic!("Both nodes {} {} aren't in the graph", x.id, y.id)
            } else if !self.has_node(x.id) {
                // x isn't in the graph
                panic!("Node {} isn't in the graph", x.id)
            } else {
                // y isn't in the graph
                panic!("Node {} isn't in the graph", y.id)
            }
        }
    }

    pub fn add_edge_from_id(&mut self, x: NodeId<'a>, y: NodeId<'a>) {
        if self.has_node(x) && self.has_node(y) {
            // Add the id of x to the left nodes list of y
            self.get_node_mut(y).unwrap().nodes_left.push(x);

            // Add the id of y to the right nodes list of x
            //x.adjacent.push(y.id);
            self.get_node_mut(x).unwrap().nodes_right.push(y);
        } else {
            if !self.has_node(x) && !self.has_node(y) {
                // Both x and y aren't in the graph
                panic!("Both nodes {} {} aren't in the graph", x, y)
            } else if !self.has_node(x) {
                // x isn't in the graph
                panic!("Node {} isn't in the graph", x)
            } else {
                // y isn't in the graph
                panic!("Node {} isn't in the graph", y)
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const RAW_SEQ: &str = "ACTGATGATCTGATCGGATA";
    const RAW_REF: &str = "GHR38";
    const OFFSET: usize = 23;

    fn yield_node<'a>() -> Node<'a> {
        Node::new(
            &RAW_SEQ[..],
            OFFSET,
            &RAW_SEQ[2..5],
            &RAW_REF[..],
            Vec::new(),
            Vec::new(),
        )
    }

    // Node
    #[test]
    fn test_can_create_node() {
        let n: Node = yield_node();
        let id = &RAW_SEQ[2..5];
        let empty_node_list: EdgeList = Vec::new();

        assert_eq!(n.segment, RAW_SEQ);
        assert_eq!(n.offset, 23);
        assert_eq!(n.id, id);
        assert_eq!(n.reference, RAW_REF);
        assert_eq!(n.nodes_left, empty_node_list);
        assert_eq!(n.nodes_right, empty_node_list);
    }

    // Graph
    #[test]
    fn test_can_create_a_singleton_graph() {
        let id = &RAW_SEQ[2..5];
        let n = yield_node();
        let mut g = Graph::new();
        g.add_node(n);

        assert!(g.has_node(id));
    }

    #[test]
    fn test_add_edge() {
        let n: Node = yield_node();
        let other_seq: &str = "TGATCTACTGATGATCTGAT";

        let n_id = &RAW_SEQ[2..5];
        let r_id = &other_seq[2..5];
        let s_id = &RAW_SEQ[1..3];
        let t_id = &other_seq[10..];

        let r = Node::new(
            &other_seq[..],
            OFFSET,
            &other_seq[2..5],
            &RAW_REF[..],
            Vec::new(),
            Vec::new(),
        );

        let s = Node::new(
            &other_seq[..],
            10,
            s_id,
            &RAW_REF[..],
            Vec::new(),
            Vec::new(),
        );

        let t = Node::new(
            &other_seq[..],
            10,
            t_id,
            &RAW_REF[..],
            Vec::new(),
            Vec::new(),
        );

        let mut g = add_nodes![n, r, s, t];

        g.add_edge_from_id(n_id, r_id);
        g.add_edge_from_id(n_id, s_id);
        g.add_edge_from_id(r_id, t_id);
        g.add_edge_from_id(s_id, t_id);

        assert!(g.edge_exists(n_id, r_id));
        assert!(!g.edge_exists(n_id, t_id));
    }
}
