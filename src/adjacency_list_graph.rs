use super::empty_list_of_lists;
use super::graph::Graph;
use super::math_graph;
use super::path_cost::ArcCost;
use super::visitor;
use super::GraphType;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(from = "math_graph::MathGraph<N>", into = "math_graph::MathGraph<N>")]
pub struct AdjList<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    arc_count: usize,
    gtype: GraphType,
    nodes: Vec<N>,
    lists: Vec<Vec<AdjArc<N>>>,
}

impl<N> AdjList<N>
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
        let arc = AdjArc::new(weight, dst);
        self.lists[src].push(arc);
        self.arc_count += 1;
    }

    pub fn node_iterator(&'_ self) -> impl Iterator<Item = (usize, N)> + '_ {
        self.nodes.iter().copied().enumerate()
    }

    pub fn arc_iterator(&'_ self) -> impl Iterator<Item = (usize, usize, N)> + '_ {
        self.lists
            .iter()
            .enumerate()
            .flat_map(|(i, list)| list.iter().map(move |a| (i, a.next, a.weight)))
    }

    pub fn successor_iterator(
        &'_ self,
        node: usize,
    ) -> impl Iterator<Item = (usize, usize, N)> + '_ {
        self.lists[node]
            .iter()
            .map(move |a| (node, a.next, a.weight))
    }
}

impl<N> Graph<N> for AdjList<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn new(node_count: usize, gtype: GraphType) -> Self {
        let nodes = vec![Default::default(); node_count];
        let lists = empty_list_of_lists(node_count);
        Self {
            arc_count: 0,
            gtype,
            nodes,
            lists,
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
        for (i, list) in enum_mut! {self.lists} {
            for (j, arc) in list.iter_mut().enumerate() {
                arc.weight = f(i, j, arc.weight);
            }
        }
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

impl<N> super::update_nodes::UpdateNodes<N> for AdjList<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn update_all_nodes_weight_iter<I>(&mut self, iter: I)
    where
        I: Iterator<Item = N>,
    {
        self.nodes.iter_mut().zip(iter).for_each(|(n, i)| *n = i)
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

impl<N> super::GetGraphType for AdjList<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn graph_type(&self) -> GraphType {
        self.gtype
    }
}

impl<N> super::GetGraphType for &AdjList<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn graph_type(&self) -> GraphType {
        self.gtype
    }
}

impl<N> visitor::GraphVisitor<N> for &AdjList<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn node_visitor<F: FnMut(usize, N)>(&self, mut f: F) {
        self.node_iterator().for_each(|(i, n)| f(i, n))
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

impl<N> From<math_graph::MathGraph<N>> for AdjList<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn from(g: math_graph::MathGraph<N>) -> Self {
        let node_count = g.node_count();
        let gtype = g.graph_type();
        let (nodes, arcs) = g.dismount();
        Self::new(node_count, gtype)
            .apply_weights(nodes)
            .apply_arcs(arcs)
    }
}
impl<N> AdjList<N>
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

impl<N> From<AdjList<N>> for math_graph::MathGraph<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn from(adj: AdjList<N>) -> Self {
        let arcs = math_graph::Arcs::new_weighted(adj.arc_iterator());
        let nodes = math_graph::Nodes::new(adj.nodes);
        Self::new(nodes, arcs, adj.gtype)
    }
}

impl<N> ArcCost<N> for &AdjList<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    fn cost(&self, src: usize, dst: usize) -> N {
        let src_list = &self.lists[src];
        let arc = src_list.iter().find(|a| a.next == dst).unwrap();
        arc.weight
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct AdjArc<N> {
    weight: N,
    next: usize,
}

impl<N> AdjArc<N>
where
    N: num_traits::Num + Default + Clone + Copy,
{
    fn new(weight: N, next: usize) -> Self {
        Self { weight, next }
    }

    #[inline]
    pub fn weight(&self) -> N {
        self.weight
    }

    #[inline]
    pub fn next(&self) -> usize {
        self.next
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::visitor::GraphVisitor;

    #[test]
    fn test_arc_cost() {
        let graph = make_graph();
        let g_ref = &graph;
        assert_eq!(g_ref.cost(0, 1), 1.0);
        assert_eq!(g_ref.cost(3, 0), 4.0);
    }

    #[test]
    fn test_direct_graph() {
        let mut graph = AdjList::new_direct(4);
        graph.add_new_arc(0, 1, 1.0);
        graph.add_new_arc(1, 2, 2.0);
        graph.add_new_arc(2, 3, 3.0);
        graph.add_new_arc(3, 0, 4.0);

        for (list, expect) in graph.lists.iter().zip([1.0, 2.0, 3.0, 4.0]) {
            assert_eq!(list.len(), 1);
            let arc = &list[0];
            assert_eq!(arc.weight, expect);
        }
    }

    #[test]
    fn test_mut_nodes_weights() {
        let mut graph = AdjList::<f64>::new_direct(5);
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
        let mut graph = AdjList::new_direct(4);
        graph.add_new_arc(0, 1, 1.0);
        graph.add_new_arc(1, 2, 2.0);
        graph.add_new_arc(2, 3, 3.0);
        graph.add_new_arc(3, 0, 4.0);

        graph.update_all_arcs_weight(|_, _, w| 2.0 * w);

        for (i, j, w) in graph.arc_iterator() {
            match (i, j) {
                (0, 1) => assert_eq!(2.0, w),
                (1, 2) => assert_eq!(4.0, w),
                (2, 3) => assert_eq!(6.0, w),
                (3, 0) => assert_eq!(8.0, w),
                (a, b) => panic!("Not existing arc ({a} {b})"),
            }
        }

        for n in 0..4 {
            let mut count = 0;
            for (i, j, w) in graph.successor_iterator(n) {
                count += 1;
                assert_eq!(i, n);
                match (n, j) {
                    (0, 1) => assert_eq!(2.0, w),
                    (1, 2) => assert_eq!(4.0, w),
                    (2, 3) => assert_eq!(6.0, w),
                    (3, 0) => assert_eq!(8.0, w),
                    (a, b) => panic!("Not existing arc ({a} {b})"),
                }
            }
            assert_eq!(count, 1);
        }
    }

    #[test]
    fn test_undirect_graph() {
        let graph = make_graph();
        for list in &graph.lists {
            assert_eq!(list.len(), 2);
        }
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
            (3, 2, 3.0),
            (3, 0, 4.0),
        ];
        assert_eq!(expect, visit_list);
    }

    #[test]
    fn test_conversion() {
        let mut orig_graph = AdjList::new_undirect(4);
        orig_graph.update_all_nodes_weight(|i, _| i);
        orig_graph.add_new_arc(0, 1, 1);
        orig_graph.add_new_arc(1, 2, 2);
        orig_graph.add_new_arc(2, 3, 3);

        let math_graph: math_graph::MathGraph<usize> = orig_graph.clone().into();
        let new_graph = AdjList::from(math_graph);
        assert_eq!(orig_graph, new_graph);

        let mut orig_graph = AdjList::new_direct(4);
        orig_graph.update_all_nodes_weight(|i, _| i);
        orig_graph.add_new_arc(0, 1, 1);
        orig_graph.add_new_arc(1, 2, 2);
        orig_graph.add_new_arc(2, 3, 3);

        let math_graph: math_graph::MathGraph<usize> = orig_graph.clone().into();
        let new_graph = AdjList::from(math_graph);
        assert_eq!(orig_graph, new_graph);
    }

    fn make_graph() -> AdjList<f64> {
        let mut graph = AdjList::new_undirect(4);
        graph.add_new_arc(0, 1, 1.0);
        graph.add_new_arc(1, 2, 2.0);
        graph.add_new_arc(2, 3, 3.0);
        graph.add_new_arc(3, 0, 4.0);
        graph
    }
}
