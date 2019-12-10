use std::collections::HashMap;



type NodeId<'a> =  &'a str;
type EdgeList<'a> = Vec<NodeId<'a>>;

/// A vertex or node in a variation graph
#[derive(Debug, PartialEq, Clone)]
pub struct Node<'a> {
    // Required: the piece of sequence associated with the node. A string of alphabet A, T, C, and G.
    segment: &'a str,

    // Offset: When a graph is built out of a reference this is the position of the start of the segment on that reference
    offset: usize,

    // Required: Unique identifier of each node.
    // Currently, a SHA 256 hash of the concatenation of segment, “+” and offset
    // TODO: Not require an offset for de novo graphs to be built
    pub id: NodeId<'a>,

    // Optional: ID of the reference from which we got this node
    reference: &'a str,

    // Required: The edges to the right of this node
    nodes_right: EdgeList<'a>,

    // Required: The edges to the left of this node
    nodes_left: EdgeList<'a>,}


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

macro_rules! digraph {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_graph = Graph::new();
            $(
                let (n, r) = $x;
                temp_graph.add_node_right(n, r);
            )*
                temp_graph
        }
    };
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
    fn new() -> Graph<'a> {
        let vg: HashMap<&'a str, Node<'a>> = HashMap::new();
        Graph(vg)
    }

    fn add_node(&mut self, n: Node<'a>) {
        let hashmap = &mut self.0;
        let id = n.id;
        hashmap.insert(id, n);
    }

    fn has_node(&self, id: NodeId) -> bool {
        let hashmap = &self.0;
        hashmap.contains_key(id)
    }

    fn get_node(&mut self, id: NodeId) -> Option< &Node<'a> > {
        let hashmap = &self.0;

        hashmap.get(id)
    }

    fn get_node_mut(&mut self, id: NodeId) -> Option< &mut Node<'a> > {
        let hashmap = &mut self.0;

        hashmap.get_mut(id)
    }

    // Add a node to the right of the current node
    fn add_node_right(&mut self, mut n: Node<'a>, mut r: Node<'a>){

        // Add node r to the right of n
        n.nodes_right.push(r.id);

        // Add node n to the right of r
        r.nodes_left.push(n.id);

        let hm = &mut self.0;

        // Update the backing hashmap.
        // Replaces the current node.
        hm.insert(n.id, n);
        hm.insert(r.id, r);
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
    fn test_add_node_right() {
        let n: Node = yield_node();
        let other_seq: &str = "TGATCTACTGATGATCTGAT";

        let n_id = &RAW_SEQ[2..5];
        let r_id = &other_seq[2..5];

        let r = Node::new(
            &other_seq[..],
            OFFSET,
            &other_seq[2..5],
            &RAW_REF[..],
            Vec::new(),
            Vec::new(),
        );

        let mut g = Graph::new();

        g.add_node_right(n, r);

        assert!(g.has_node(n_id));
        assert!(g.has_node(r_id));
    }

    #[test]
    fn test_graph_macro() {
        let n: Node = yield_node();
        let n_copy = n.clone();
        let other_seq: &str = "TGATCTACTGATGATCTGAT";

        let n_id = &RAW_SEQ[2..5];
        let r_id = &other_seq[2..5];

        let r = Node::new(
            &other_seq[..],
            3,
            &other_seq[2..5],
            &RAW_REF[..],
            Vec::new(),
            Vec::new(),
        );

        let s = Node::new(
            &other_seq[..],
            10,
            &other_seq[1..3],
            &RAW_REF[..],
            Vec::new(),
            Vec::new(),
        );

        let mut g = digraph![
            (n, r),
            (r, s),
            (n,s)
        ];

        assert!(g.has_node(n_id));
        assert!(g.has_node(r_id));
        assert_eq!(g.get_node(n_id).unwrap(), &n_copy);
    }
}
