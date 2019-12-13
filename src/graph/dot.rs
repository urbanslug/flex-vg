use crate::graph::types::{Graph, Node};
use std::env;
use std::fs::File;
use std::io::Write;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn fmt_graph_to_dot(g: &Graph) {
    let hm = g.hashmap();

    let mut file = File::create("foo.dot").unwrap();

    // Write header
    writeln!(
        &mut file,
        "\
digraph {{
\t// Generated by {} version {}

\trankdir=LR;
\tdpi=300;
\tedge [color=blue, tailport=ne, headport=nw, arrowtail=dot, arrowhead=normal, arrowsize=.5];
\tnode [shape=box];
",
        NAME, VERSION
    );

    // Write nodes
    writeln!(&mut file, "\t// nodes");

    for (k, v) in hm.iter() {
        writeln!(&mut file, "\t{} [label=\"{}\"]", k, &v.segment[..5]);
    }

    // write edges
    writeln!(&mut file, "\n\t// edges");

    for (k, Node { nodes_right, .. }) in hm.iter() {
        for n in nodes_right.iter() {
            writeln!(&mut file, "\t{} -> {}", k, n);
        }
    }

    writeln!(&mut file, "}}");
}

#[cfg(test)]
mod tests {

    use super::*;
    //use crate::graph::macros;

    const RAW_SEQ: &str = "ACTGATGATCTGATCGGATA";
    const RAW_REF: &str = "GHR38";
    const OFFSET: usize = 23;

    #[test]
    fn test_digraph_to_dot() {
        let other_seq: &str = "TGATCTACTGATGATCTGAT";

        let n_id = &RAW_SEQ[2..5];
        let r_id = &other_seq[2..5];
        let s_id = &RAW_SEQ[1..3];
        let t_id = &other_seq[10..];

        let n = Node::new(
            &RAW_SEQ[..],
            OFFSET,
            "a",
            &RAW_REF[..],
            Vec::new(),
            Vec::new(),
        );

        let r = Node::new(
            "LIMSPABDJJHDGJJGASJGDJHSBJKHKJS",
            OFFSET,
            "b",
            &RAW_REF[..],
            Vec::new(),
            Vec::new(),
        );

        let s = Node::new(
            "EROROPOOOOEOOEROROPOOOERO",
            10,
            "c",
            &RAW_REF[..],
            Vec::new(),
            Vec::new(),
        );

        let t = Node::new(
            &other_seq[..],
            10,
            "d",
            &RAW_REF[..],
            Vec::new(),
            Vec::new(),
        );

        let mut g = add_nodes![n, r, s, t];

        g.add_edge_from_id("a", "b");
        g.add_edge_from_id("a", "c");
        g.add_edge_from_id("b", "d");
        g.add_edge_from_id("c", "d");

        fmt_graph_to_dot(&g);
    }
}
