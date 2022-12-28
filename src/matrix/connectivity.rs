use std::collections::HashMap;
use petgraph::csr::Csr;
use super::*;


// analyze the matrix to find connections between components
pub fn find_connections(matrix: &Matrix<Label>, labels_amount: usize, threshold: usize) 
-> Csr<Label, ()> {

    // prepare a graph with all the nodes
    let mut graph = Csr::<Label, ()>::new();
    for label in 1 ..= labels_amount as Label {graph.add_node(label);}

    // traverse the matrix looking for connections
    matrix.for_each(&mut |x, y, z| {
        let label1 = matrix.get(x, y, z);
        if label1 > 0 {

            // find all neighbors
            let neighbors = matrix.get_neighbors(x, y, z, 0);

            // count the number of occurence of each neighbors label
            let mut counters = HashMap::<Label, usize>::with_capacity(6);
            for label2 in neighbors {
                if let Some(count) = counters.get_mut(&label2) {
                    *count += 1;
                } else {
                    counters.insert(label2, 1);
                }
            }

            // find labels which adjacent amount is greater than the threshold
            for (label2, count) in counters {
                if label2 > 0 && label1 != label2 && count >= threshold {
                    graph.add_edge(label1, label2, ());
                }
            }
        }
    });

    return graph;
}