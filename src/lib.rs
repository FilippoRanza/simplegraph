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
mod update_nodes;
pub mod visitor;

use serde::{Deserialize, Serialize};

pub use adjacency_list_graph::AdjList;
pub use graph::Graph;
pub use matrix_graph::MatrixGraph;
pub use visitor::GraphVisitor;

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq, Debug)]
pub enum GraphType {
    Direct,
    Undirect,
}

pub trait GetGraphType {
    fn graph_type(&self) -> GraphType;
}

#[cfg(test)]
mod tests {}

fn empty_list_of_lists<T>(count: usize) -> Vec<Vec<T>> {
    (0..count).map(|_| vec![]).collect()
}

pub struct Node<N>(N);
impl<N> Default for Node<N>
where
    N: num_traits::Num + Default,
{
    fn default() -> Self {
        Self(Default::default())
    }
}
