use super::empty_list_of_lists;
use super::visitor;
use super::GraphType;

pub struct AdjList<N> {
    gtype: GraphType,
    nodes: Vec<N>,
    lists: Vec<Vec<AdjArc<N>>>,
}

impl<N> AdjList<N>
where
    N: num_traits::Num + Default + Clone + Copy,
{
    pub fn new_direct(node_count: usize) -> Self {
        Self::new(node_count, GraphType::Direct)
    }

    pub fn new_undirect(node_count: usize) -> Self {
        Self::new(node_count, GraphType::Undirect)
    }

    pub fn new(node_count: usize, gtype: GraphType) -> Self {
        let nodes = vec![Default::default(); node_count];
        let lists = empty_list_of_lists(node_count);
        Self {
            gtype,
            nodes,
            lists,
        }
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
        for (i, list) in enum_mut! {self.lists} {
            for (j, arc) in list.iter_mut().enumerate() {
                arc.weight = f(i, j, arc.weight);
            }
        }
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
        let arc = AdjArc::new(weight, dst);
        self.lists[src].push(arc);
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

impl<N> super::GetGraphType for &AdjList<N> {
    fn graph_type(&self) -> GraphType {
        self.gtype
    }
}

impl<N> visitor::GraphVisitor<N> for &AdjList<N>
where
    N: num_traits::Num + Default + Clone + Copy,
{
    fn node_visitor<F: FnMut(usize, N)>(&self, mut f: F) {
        self.node_iterator().for_each(|(i, n)| f(i, n))
    }

    fn arc_visitor<G: FnMut(usize, usize, N)>(&self, mut g: G) {
        self.arc_iterator().for_each(|(i, j, n)| g(i, j, n))
    }
}

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

    fn make_graph() -> AdjList<f64> {
        let mut graph = AdjList::new_undirect(4);
        graph.add_new_arc(0, 1, 1.0);
        graph.add_new_arc(1, 2, 2.0);
        graph.add_new_arc(2, 3, 3.0);
        graph.add_new_arc(3, 0, 4.0);
        graph
    }
}
