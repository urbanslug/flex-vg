#![allow(unused_macros)]

#![macro_use]
macro_rules! add_nodes {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_graph = Graph::new();
            $(
                temp_graph.add_node($x);
            )*
                temp_graph
        }
    };
}
