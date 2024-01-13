use super::*;
use crate::math::Vec3i;
use crate::schematic;
use petgraph::csr::Csr;
use std::collections::HashMap;

// regroup the type of the component, its position and the model to use
#[derive(Default, Clone)]
pub struct Element<T: Clone + Copy + Eq + Default> {
    pub label: Label,
    pub value: T,
    pub position: Vec3i,
    pub volume: usize,
    pub morph: Morph,
}
impl<T: Clone + Copy + Eq + Default> Element<T> {
    // make a new container of component data
    pub fn new(label: Label, value: T, position: Vec3i, volume: usize, morph: Morph) -> Self {
        Self {
            label,
            value,
            position,
            volume,
            morph,
        }
    }
}

// parse the matrix and deduce data that will be used to make a schematic
pub fn parse_matrix<T: Clone + Copy + Eq + Default>(
    matrix: &Matrix<T>,
    is_empty: &FnEmpty<T>,
    threshold: usize,
) -> (
    Csr<Label, ()>,
    Vec<Element<T>>,
    HashMap<Morph, schematic::Model>,
) {
    // generate a matrix with a label for each component
    let (labels_matrix, labels_mapping) = connected_component_labeling(matrix, is_empty);
    let labels_amount = labels_mapping.len();

    // find how elements are connected togethers
    let graph = find_connections(&labels_matrix, labels_amount, threshold);

    // find the minimal bounding box of each component
    let boxes = find_bounding_boxes(&labels_matrix, labels_amount);

    // build two lists with element data and model
    let mut elements = Vec::<Element<T>>::with_capacity(labels_amount);
    elements.resize(labels_amount, Element::default());
    let mut models = HashMap::<Morph, schematic::Model>::with_capacity(labels_amount);

    // for each label, generate corresponding component data
    for (index, abox) in boxes.iter().enumerate() {
        let label = (index + 1) as Label;
        let value = labels_mapping[&label];

        // find the morphological signature
        let (morph, volume) = generate_morph(&labels_matrix, label, *abox);
        elements[index] = Element::<T>::new(label, value, abox.begin, volume, morph);

        // if the component has a new morphology, generate a model for it
        //if !models.contains_key(&morph) {
        //    models.insert(morph, generate_model(&labels_matrix, label, *abox));
        //}
    }
    models.shrink_to_fit();
    (graph, elements, models)
}
