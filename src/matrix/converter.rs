use std::collections::HashMap;
use petgraph::{csr::Csr, visit::IntoNeighbors};
use crate::circuit::Channel;
use crate::schematic::*;
use crate::matrix::*;


// define the type of element to build for a given value from the matrix
pub enum ToBuild {
    Wire(Channel),
    Comp(CompType),
    Empty,
}


// generate a schema from the given matrix
pub fn convert_matrix_to_schema<T: Clone + Copy + Eq + Default>
(matrix: &Matrix<T>, is_empty: &FnEmpty<T>, threshold: usize, convert: &dyn Fn(T, usize) -> ToBuild)
-> Schema {

    // from the matrix analysis, generate a schematic
    let (graph, elements, models) = parse_matrix(matrix, is_empty, threshold);

    // build the reverse graph to find input wires
    let rev_graph = reverse_graph(&graph);

    // generate the list of models
    // keep track of the mapping between signature and index
    let mut index = 0;
    let mut signatures = HashMap::<Signature, ModelIndex>::with_capacity(models.len());
    let mut model_list = Vec::<ModelData>::with_capacity(models.len());
    for (signature, model) in models {
        signatures.insert(signature, index);
        model_list.push(model);
        index += 1;
    }

    // generate the list of wires and other elements
    let mut wire_list = Vec::<CompWire>::with_capacity(elements.len());
    let mut comp_list = Vec::<CompData>::with_capacity(elements.len());
    for element in elements.iter() {

        // prepare the model of the element
        let model_attr = ModelAttr {
            position: element.position,
            index: signatures[&element.signature],
        };

        // convert element into schematic component
        match convert(element.value, element.volume) {
            ToBuild::Wire(channel) => {
                // build a new wire
                wire_list.push(CompWire {
                    channel: channel,
                    model_attr: model_attr,
                });
            },
            ToBuild::Comp(comp_type) => {
                // find inputs and outputs
                let pins_in : Vec<CompIndex> = rev_graph.neighbors(element.label).map(|v| v).collect();
                let pins_out: Vec<CompIndex> = graph    .neighbors(element.label).map(|v| v).collect();

                // build a new component
                comp_list.push(CompData{
                    comp_type  : comp_type,
                    pins_in    : pins_in,
                    pins_out   : pins_out,
                    model_attr : model_attr,
                });
            },
            _ => {},
        }
    }

    return Schema {
        wires      : wire_list,
        components : comp_list,
        models     : model_list,
    }
}


// the petgraph::visit::Reversed should have allowed to do this in one line of code...
// but alas it doesn't work with the IntoNeighbors trait
fn reverse_graph(graph: &Csr<Label, ()>) -> Csr<Label, ()> {
    let mut reversed = Csr::<Label, ()>::new();
    let label_count = graph.node_count();
    for label in 1..=label_count as Label {reversed.add_node(label);}
    for l1 in 1..=label_count as Label {
        for l2 in graph.neighbors(l1) {
            reversed.add_edge(l2, l1, ());
        }
    }
    return reversed;
}