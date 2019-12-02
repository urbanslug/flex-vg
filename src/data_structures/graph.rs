/// Whether the strand is being traversed in positive or negative orientation
pub enum Strand {
    /// Traversing an edge from 5' to 3'
    Positive,

    /// Traversing an edge from 3' to 5'
    Negative,
}

/// An edge in a variation graph.
/// This includes the nodes to the right, left and the strand of the edge
/// i.e. the direction of traversal
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

    //Required: Unique identifier of each node.
    // Currently, a SHA 256 hash of the concatenation of segment, “+” and offset
    // TODO: Not require an offset for de novo graphs to be built
    id: &'a str,

    // Optional: ID of the reference from which we got this node
    refence: &'a str,

    // Required: The edges to the right of this node
    links_right: Vec<Link<'a>>,

    // Required: The edges to the left of this node
    links_left: Vec<Link<'a>>,
}
