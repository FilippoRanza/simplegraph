/***
 * Ciao
 */

macro_rules! enum_mut {
    ($list:expr) => {
        $list.iter_mut().enumerate()
    };
}

pub mod adjacency_list_graph;
pub mod dot;
pub mod graph;
pub mod math_graph;
pub mod matrix_graph;
pub mod path_cost;
mod update_nodes;
pub mod visitor;

use serde::{Deserialize, Serialize};

pub use adjacency_list_graph::AdjList;
pub use graph::Graph;
pub use matrix_graph::MatrixGraph;
pub use visitor::GraphVisitor;

/**
 * Specify, for a graph, if it is direct or not.  
 */
#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Debug)]
pub enum GraphType {
    Direct,
    Undirect,
}

/**
 * Return the graph's type
 */
pub trait GetGraphType {
    fn graph_type(&self) -> GraphType;
}

fn empty_list_of_lists<T>(count: usize) -> Vec<Vec<T>> {
    (0..count).map(|_| vec![]).collect()
}


#[cfg(test)]
mod tests {
    pub fn euclid_distance(p1: &(f64, f64), p2: &(f64, f64)) -> f64 {
        let (x1, y1) = p1;
        let (x2, y2) = p2;
        let dx = x1 - x2;
        let dy = y1 - y2;
        ((dx * dx) + (dy * dy)).sqrt()
    }

    pub fn approx_equal(a: f64, b: f64, tol: f64) {
        let diff = (a - b).abs();
        assert!(diff <= tol, "a: {a}, b: {b}, tol: {tol}");
    }
}


