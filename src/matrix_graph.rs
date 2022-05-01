use super::math_graph;
use super::update_nodes;
use super::visitor;
use super::{Graph, GraphType, GetGraphType};
use ndarray::{Array2, Zip};
use num_traits;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[serde(from = "math_graph::MathGraph<N>", into = "math_graph::MathGraph<N>")]
pub struct MatrixGraph<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    arc_count: usize,
    gtype: GraphType,
    nodes: Vec<N>,
    adj_mat: Array2<bool>,
    weight_mat: Array2<N>,
}

impl<N> MatrixGraph<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{

    pub fn new_direct(node_count: usize) -> Self {
        Self::new(node_count, GraphType::Direct)
    }

    pub fn new_undirect(node_count: usize) -> Self {
        Self::new(node_count, GraphType::Undirect)
    }

    

    fn make_arc(&mut self, src: usize, dst: usize, weight: N) {
        if let Some(adj) = self.adj_mat.get_mut((src, dst)) {
            if !*adj {
                *adj = true;
                self.weight_mat[(src, dst)] = weight;
                self.arc_count += 1;
            }
        } else {
            panic! {"No entry at ({src}, {dst})"}
        }
    }

    pub fn node_iterator(&'_ self) -> impl Iterator<Item = (usize, N)> + '_ {
        self.nodes.iter().copied().enumerate()
    }

    pub fn arc_iterator(&'_ self) -> impl Iterator<Item = (usize, usize, N)> + '_ {
        self.adj_mat
            .indexed_iter()
            .zip(self.weight_mat.iter())
            .filter_map(|(((i, j), a), w)| if *a { Some((i, j, *w)) } else { None })
    }

    pub fn successor_iterator(
        &'_ self,
        node: usize,
    ) -> impl Iterator<Item = (usize, usize, N)> + '_ {
        let nc = self.nodes.len();
        (0..nc).filter_map(move |j| {
            if self.adj_mat[(node, j)] {
                Some((node, j, self.weight_mat[(node, j)]))
            } else {
                None
            }
        })
    }
}

impl<N> GetGraphType for MatrixGraph<N> 
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn graph_type(&self) -> GraphType {
        self.gtype
    }
}

impl<N> GetGraphType for &MatrixGraph<N> 
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn graph_type(&self) -> GraphType {
        self.gtype
    }
}

impl<N> Graph<N> for MatrixGraph<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{

    fn new(node_count: usize, gtype: GraphType) -> Self {
        let nodes = vec![Default::default(); node_count];
        let mat_size = (node_count, node_count);
        let adj_mat = Array2::default(mat_size);
        let weight_mat = Array2::zeros(mat_size);
        Self {
            arc_count: 0,
            nodes,
            gtype,
            adj_mat,
            weight_mat,
        }
    }
     fn add_new_default_arc(&mut self, src: usize, dst: usize) {
        self.add_new_arc(src, dst, Default::default());
    }

     fn add_new_arc(&mut self, src: usize, dst: usize, weight: N) {
        match self.gtype {
            GraphType::Direct => {
                self.make_arc(src, dst, weight);
            }
            GraphType::Undirect => {
                self.make_arc(src, dst, weight);
                self.make_arc(dst, src, weight);
            }
        }
    }

     fn update_all_arcs_weight<F>(&mut self, f: F)
    where
        F: Fn(usize, usize, N) -> N,
    {
        Zip::indexed(&mut self.weight_mat)
            .and(&self.adj_mat)
            .for_each(|(i, j), w, a| {
                if *a {
                    *w = f(i, j, *w)
                }
            });
    }

     fn update_all_nodes_weight<F>(&mut self, f: F)
    where
        F: Fn(usize, N) -> N,
    {
        for (i, n) in enum_mut! {self.nodes} {
            *n = f(i, *n);
        }
    }
}

impl<N> visitor::GraphVisitor<N> for &MatrixGraph<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn node_visitor<F: FnMut(usize, N)>(&self, mut f: F) {
        self.node_iterator().for_each(|(i, j)| f(i, j))
    }
    fn arc_visitor<G: FnMut(usize, usize, N)>(&self, mut g: G) {
        self.arc_iterator().for_each(|(i, j, n)| g(i, j, n))
    }

    fn node_count(&self) -> usize {
        self.nodes.len()
    }

    fn arc_count(&self) -> usize {
        self.arc_count
    }
}

impl<N> From<math_graph::MathGraph<N>> for MatrixGraph<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn from(g: math_graph::MathGraph<N>) -> Self {
        let gtype = g.graph_type();
        let node_count = g.node_count();
        let (nodes, arcs) = g.dismount();
        Self::new(node_count, gtype)
            .apply_weights(nodes)
            .apply_arcs(arcs)
    }
}

impl<N> MatrixGraph<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn apply_weights(mut self, nodes: math_graph::Nodes<N>) -> Self {
        math_graph::apply_nodes(&mut self, nodes);
        self
    }

    fn apply_arcs(mut self, arcs: math_graph::Arcs<N>) -> Self {
        math_graph::apply_arcs(&mut self, arcs);
        self
    }
}

impl<N> update_nodes::UpdateNodes<N> for MatrixGraph<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn update_all_nodes_weight_iter<I>(&mut self, iter: I)
    where
        I: Iterator<Item = N>,
    {
        self.nodes.iter_mut().zip(iter).for_each(|(n, i)| *n = i);
    }
    fn update_indexed_nodes_weight<I>(&mut self, iter: I)
    where
        I: Iterator<Item = (usize, N)>,
    {
        for (i, w) in iter {
            self.nodes[i] = w;
        }
    }
}

impl<N> Into<math_graph::MathGraph<N>> for MatrixGraph<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn into(self) -> math_graph::MathGraph<N> {
        let arcs = math_graph::Arcs::new_weighted(self.arc_iterator());
        let nodes = math_graph::Nodes::new_extended(self.nodes);
        math_graph::MathGraph::new(nodes, arcs, self.gtype)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::visitor::GraphVisitor;

    #[test]
    fn test_direct_graph() {
        let mut graph = MatrixGraph::new_direct(4);
        graph.add_new_arc(0, 1, 1.0);
        graph.add_new_arc(1, 2, 2.0);
        graph.add_new_arc(2, 3, 3.0);
        graph.add_new_arc(3, 0, 4.0);

        assert_eq!(graph.weight_mat[(0, 1)], 1.0);
        assert_eq!(graph.weight_mat[(1, 2)], 2.0);
        assert_eq!(graph.weight_mat[(2, 3)], 3.0);
        assert_eq!(graph.weight_mat[(3, 0)], 4.0);

        assert_eq!(graph.weight_mat[(1, 0)], 0.0);
        assert_eq!(graph.weight_mat[(2, 1)], 0.0);
        assert_eq!(graph.weight_mat[(3, 2)], 0.0);
        assert_eq!(graph.weight_mat[(0, 3)], 0.0);
    }

    #[test]
    fn test_mut_nodes_weights() {
        let mut graph = MatrixGraph::<f64>::new_direct(5);
        for n in &graph.nodes {
            assert_eq!(*n, 0.0);
        }

        graph.update_all_nodes_weight(|i, _| 1.5 * (i as f64));

        for (i, n) in graph.node_iterator() {
            assert_eq!(n, 1.5 * (i as f64));
        }
    }

    #[test]
    fn test_mut_arc_weights() {
        let mut graph = make_graph();

        graph.update_all_arcs_weight(|_, _, w| 2.0 * w);

        for (i, j, w) in graph.arc_iterator() {
            match (i, j) {
                (0, 1) | (1, 0) => assert_eq!(2.0, w),
                (1, 2) | (2, 1) => assert_eq!(4.0, w),
                (2, 3) | (3, 2) => assert_eq!(6.0, w),
                (3, 0) | (0, 3) => assert_eq!(8.0, w),
                (a, b) => panic!("Not existing arc ({a} {b}) with weight {w}"),
            }
        }

        assert_eq!(graph.weight_mat[(0, 1)], 2.0);
        assert_eq!(graph.weight_mat[(1, 2)], 4.0);
        assert_eq!(graph.weight_mat[(2, 3)], 6.0);
        assert_eq!(graph.weight_mat[(3, 0)], 8.0);

        assert_eq!(graph.weight_mat[(1, 0)], 2.0);
        assert_eq!(graph.weight_mat[(2, 1)], 4.0);
        assert_eq!(graph.weight_mat[(3, 2)], 6.0);
        assert_eq!(graph.weight_mat[(0, 3)], 8.0);
    }

    #[test]
    fn test_node_visitor() {
        let mut graph = make_graph();
        graph.update_all_nodes_weight(|i, _| (i as f64));
        let mut visit_list: Vec<(usize, f64)> = vec![];
        (&graph).node_visitor(|i, n| visit_list.push((i, n)));
        assert_eq!(vec![(0, 0.0), (1, 1.0), (2, 2.0), (3, 3.0)], visit_list);
    }

    #[test]
    fn test_arc_visitor() {
        let graph = make_graph();
        let mut visit_list: Vec<(usize, usize, f64)> = vec![];
        (&graph).arc_visitor(|i, j, n| visit_list.push((i, j, n)));
        let expect = vec![
            (0, 1, 1.0),
            (0, 3, 4.0),
            (1, 0, 1.0),
            (1, 2, 2.0),
            (2, 1, 2.0),
            (2, 3, 3.0),
            (3, 0, 4.0),
            (3, 2, 3.0),
        ];
        assert_eq!(expect, visit_list);
    }

    fn make_graph() -> MatrixGraph<f64> {
        let mut graph = MatrixGraph::new_undirect(4);
        graph.add_new_arc(0, 1, 1.0);
        graph.add_new_arc(1, 2, 2.0);
        graph.add_new_arc(2, 3, 3.0);
        graph.add_new_arc(3, 0, 4.0);
        graph
    }
}
