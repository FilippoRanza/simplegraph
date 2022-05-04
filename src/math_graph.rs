/*!
 * Pseudo-math representation of a graph. Used only as a command middleman 
 * for [Serde](https://serde.rs).
 */
use super::graph;
use super::update_nodes;
use super::{GetGraphType, GraphType};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MathGraph<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    gtype: GraphType,
    nodes: Nodes<N>,
    arcs: Arcs<N>,
}

impl<N> MathGraph<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    pub fn new(nodes: Nodes<N>, arcs: Arcs<N>, gtype: GraphType) -> Self {
        Self { nodes, arcs, gtype }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.node_count()
    }

    pub fn graph_type(&self) -> GraphType {
        self.gtype
    }

    pub fn dismount(self) -> (Nodes<N>, Arcs<N>) {
        (self.nodes, self.arcs)
    }
}

#[derive(Deserialize, Serialize)]
pub enum Nodes<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    Extended(Vec<N>),
    Compact(CompactNodes<N>),
}

impl<N> Nodes<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    pub fn new(vec: Vec<N>) -> Self {
        let total = vec.len();
        let zeros = count_zeros(vec.iter());
        if 2 * zeros > (total + 1) {
            Self::new_compact(vec.into_iter().enumerate(), total)
        } else {
            Self::new_extended(vec)
        }
    }

    fn new_extended(vect: Vec<N>) -> Self {
        Self::Extended(vect)
    }

    fn new_compact<Ni>(ni: Ni, count: usize) -> Self
    where
        Ni: Iterator<Item = (usize, N)>,
    {
        let weights = ni.filter(|(_, n)| !n.is_zero()).collect();
        let compact = CompactNodes::new(count, weights);
        Self::Compact(compact)
    }

    fn node_count(&self) -> usize {
        match self {
            Self::Compact(compact) => compact.count,
            Self::Extended(nodes) => nodes.len(),
        }
    }
}

fn count_zeros<'a, I, N: 'a>(iter: I) -> usize
where
    I: Iterator<Item = &'a N>,
    N: num_traits::Num,
{
    iter.filter(|n| n.is_zero()).count()
}

#[derive(Deserialize, Serialize)]
pub struct CompactNodes<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    count: usize,
    weights: Vec<(usize, N)>,
}

impl<N> CompactNodes<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    pub fn new(count: usize, weights: Vec<(usize, N)>) -> Self {
        Self { count, weights }
    }

    pub fn iter_weights(self) -> impl Iterator<Item = (usize, N)> {
        self.weights.into_iter()
    }
}

#[derive(Deserialize, Serialize)]
pub enum Arcs<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    Simple(Vec<(usize, usize)>),
    Weighted(Vec<(usize, usize, N)>),
}

impl<N> Arcs<N>
where
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    pub fn new_simple<Ni>(ni: Ni) -> Self
    where
        Ni: Iterator<Item = (usize, usize, N)>,
    {
        let vect = ni.map(|(i, j, _)| (i, j)).collect();
        Self::Simple(vect)
    }

    pub fn new_weighted<Ni>(ni: Ni) -> Self
    where
        Ni: Iterator<Item = (usize, usize, N)>,
    {
        Self::Weighted(ni.collect())
    }
}

pub fn apply_nodes<G, N>(g: &mut G, nodes: Nodes<N>)
where
    G: update_nodes::UpdateNodes<N>,
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    match nodes {
        Nodes::Extended(weights) => g.update_all_nodes_weight_iter(weights.into_iter()),
        Nodes::Compact(compact) => g.update_indexed_nodes_weight(compact.iter_weights()),
    }
}

pub fn apply_arcs<G, N>(g: &mut G, arcs: Arcs<N>)
where
    G: graph::Graph<N> + GetGraphType,
    N: num_traits::Num + Default + Clone + Copy + Serialize,
{
    match arcs {
        Arcs::Simple(simple) => simple.into_iter().for_each(|(i, j)| {
            conditional_insert_arc(
                g.graph_type(),
                |i, j, w| g.add_new_arc(i, j, w),
                i,
                j,
                N::default(),
            )
        }),
        Arcs::Weighted(weighted) => weighted.into_iter().for_each(|(i, j, w)| {
            conditional_insert_arc(g.graph_type(), |i, j, w| g.add_new_arc(i, j, w), i, j, w)
        }),
    }
}

fn conditional_insert_arc<F, T>(gt: GraphType, mut f: F, i: usize, j: usize, t: T)
where
    F: FnMut(usize, usize, T),
{
    match gt {
        GraphType::Direct => f(i, j, t),
        GraphType::Undirect if i <= j => f(i, j, t),
        _ => {}
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_nodes_constructor() {
        let zero_nodes = vec![0; 10];
        let nodes = Nodes::new(zero_nodes);
        assert!(matches!(nodes, Nodes::Compact(c) if c.count == 10 && c.weights == vec![]));

        let ones = vec![1; 10];
        let nodes = Nodes::new(ones);
        assert!(matches!(nodes, Nodes::Extended(e) if e == vec![1; 10]));

        let mixed = vec![0, 1, 0, 1, 0, 1, 0, 1, 0, 0];
        let nodes = Nodes::new(mixed);
        assert!(
            matches!(nodes, Nodes::Compact(c) if c.count == 10 && c.weights == vec![(1, 1), (3, 1), (5, 1), (7, 1)])
        );
    }

    #[test]
    fn test_count_zeros() {
        let zeros = vec![0; 10];
        assert_eq!(count_zeros(zeros.iter()), 10);

        let ones = vec![1; 10];
        assert_eq!(count_zeros(ones.iter()), 0);

        let mixed = vec![0, 1, 0, 1, 0];
        assert_eq!(count_zeros(mixed.iter()), 3);
    }
}
