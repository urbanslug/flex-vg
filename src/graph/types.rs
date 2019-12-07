use std::collections::HashMap;

/// Whether the edge is being traversed in
/// positive(5' to 3') or negative(3' to 5') orientation.
#[derive(PartialEq, Debug)]
pub enum Strand {
    /// Traversing an edge from 5' to 3'
    Positive,

    /// Traversing an edge from 3' to 5'
    Negative,
}

/// An edge in a [variation graph].
///
/// [variation graph]: https://blog.urbanslug.com/posts/2019-06-22-Introduction-to-Variation-Graphs.html
/// This includes the nodes to the right, left and the strand of the edge
/// i.e. the direction of traversal
#[derive(PartialEq, Debug)]
pub struct Link<'a> {
    // Should this edge be traversed in the positive or negative orientation
    strand: Strand,

    // ID of nodes to the left of this edge
    left: &'a str,

    // ID of nodes to the right of this edge
    right: &'a str,
}

/// A vertex or node in a variation graph
pub struct Node<'a> {
    // Required: the piece of sequence associated with the node. A string of alphabet A, T, C, and G.
    segment: &'a str,

    // Offset: When a graph is built out of a reference this is the position of the start of the segment on that reference
    offset: usize,

    // Required: Unique identifier of each node.
    // Currently, a SHA 256 hash of the concatenation of segment, “+” and offset
    // TODO: Not require an offset for de novo graphs to be built
    pub id: &'a str,

    // Optional: ID of the reference from which we got this node
    reference: &'a str,

    // Required: The edges to the right of this node
    links_right: Vec<Link<'a>>,

    // Required: The edges to the left of this node
    links_left: Vec<Link<'a>>,
}

impl<'a> Node<'a> {
    fn new(
        segment: &'a str,
        offset: usize,
        id: &'a str,
        reference: &'a str,
        links_right: Vec<Link<'a>>,
        links_left: Vec<Link<'a>>,
    ) -> Self {
        Node {
            segment,
            offset,
            id,
            reference,
            links_left,
            links_right,
        }
    }
}

// TODO: link id and node
/// A [variation graph] is a HashMap of [`id`] to [`Node`].
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

    fn has_node(&self, id: &'a str) -> bool {
        let hashmap = &self.0;
        hashmap.contains_key(id)
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

    #[test]
    fn can_create_node() {
        let n: Node = yield_node();
        let id = &RAW_SEQ[2..5];

        assert_eq!(n.segment, RAW_SEQ);
        assert_eq!(n.offset, 23);
        assert_eq!(n.id, id);
        assert_eq!(n.reference, RAW_REF);
        assert_eq!(n.links_left, Vec::new());
        assert_eq!(n.links_right, Vec::new());
    }

    #[test]
    fn test_can_create_a_singleton_graph() {
        let id = &RAW_SEQ[2..5];
        let n = yield_node();
        let mut g = Graph::new();
        g.add_node(n);

        assert!(g.has_node(id));
    }
}
