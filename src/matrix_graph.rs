use super::visitor;
use super::GraphType;
use ndarray::{Array2, Zip};
use num_traits;

pub struct MatrixGraph<N> {
    gtype: GraphType,
    nodes: Vec<N>,
    adj_mat: Array2<bool>,
    weight_mat: Array2<N>,
}

impl<N> MatrixGraph<N>
where
    N: num_traits::Num + Default + Clone + Copy,
{
    pub fn new(node_count: usize, gtype: GraphType) -> Self {
        let nodes = vec![Default::default(); node_count];
        let mat_size = (node_count, node_count);
        let adj_mat = Array2::default(mat_size);
        let weight_mat = Array2::zeros(mat_size);
        Self {
            nodes,
            gtype,
            adj_mat,
            weight_mat,
        }
    }

    pub fn new_direct(node_count: usize) -> Self {
        Self::new(node_count, GraphType::Direct)
    }

    pub fn new_undirect(node_count: usize) -> Self {
        Self::new(node_count, GraphType::Undirect)
    }

    pub fn add_new_default_arc(&mut self, src: usize, dst: usize) {
        self.add_new_arc(src, dst, Default::default());
    }

    pub fn add_new_arc(&mut self, src: usize, dst: usize, weight: N) {
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

    pub fn update_all_arcs_weight<F>(&mut self, f: F)
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

    pub fn update_all_nodes_weight<F>(&mut self, f: F)
    where
        F: Fn(usize, N) -> N,
    {
        for (i, n) in enum_mut! {self.nodes} {
            *n = f(i, *n);
        }
    }

    fn make_arc(&mut self, src: usize, dst: usize, weight: N) {
        self.adj_mat[(src, dst)] = true;
        self.weight_mat[(src, dst)] = weight;
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

impl<N> super::GetGraphType for &MatrixGraph<N> {
    fn graph_type(&self) -> GraphType {
        self.gtype
    }
}

impl<N> visitor::GraphVisitor<N> for &MatrixGraph<N>
where
    N: num_traits::Num + Default + Clone + Copy,
{
    fn node_visitor<F: FnMut(usize, N)>(&self, mut f: F) {
        self.node_iterator().for_each(|(i, j)| f(i, j))
    }
    fn arc_visitor<G: FnMut(usize, usize, N)>(&self, mut g: G) {
        self.arc_iterator().for_each(|(i, j, n)| g(i, j, n))
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
