/*!
 * Compute the cost of all sub path.
 */

use std::ops::Range;
use std::slice::Iter;

/**
 * Implementations of this trait allows a direct
 * query of an arcs' costs.
 */
pub trait ArcCost<N>
where
    N: num_traits::Num,
{
    /**
     * Return the weight associated to given arc.
     */
    fn cost(&self, src: usize, dst: usize) -> N;
}

/**
 * Implement an iterator over the
 * cost (length)  of all the sub-path
 * of the given path on graph G.
 * Note: only the subpath in one direction
 * are yielded by the iterator, this behavior is coherent
 * with direct graphs and it is trivial to adapt the
 * result to undirected graphs.
 *
 * ```
 * use simplegraph::MatrixGraph;
 * use simplegraph::Graph;
 * use simplegraph::path_cost::AllSubPathCost;
 *
 * // Create a new Graph
 * let mut graph = MatrixGraph::<f64>::new_direct(4);
 * graph.add_new_arc(0, 1, 1.0);
 * graph.add_new_arc(1, 2, 2.0);
 * graph.add_new_arc(2, 3, 3.0);
 * graph.add_new_arc(3, 0, 4.0);
 *
 * // Iterate through all subpath costs
 * let mut path_cost_iter = AllSubPathCost::new(&graph, &[0, 1, 2, 3]);
 * assert_eq!(path_cost_iter.next(), Some((0, 1, 1.0)));
 * assert_eq!(path_cost_iter.next(), Some((0, 2, 3.0)));
 * assert_eq!(path_cost_iter.next(), Some((0, 3, 6.0)));
 * assert_eq!(path_cost_iter.next(), Some((1, 2, 2.0)));
 * assert_eq!(path_cost_iter.next(), Some((1, 3, 5.0)));
 * assert_eq!(path_cost_iter.next(), Some((2, 3, 3.0)));
 * assert_eq!(path_cost_iter.next(), None);
 * ```
 *
 */
pub struct AllSubPathCost<'a, G, N>
where
    G: ArcCost<N>,
    N: num_traits::Num,
{
    curr: usize,
    weight: N,
    graph: G,
    path: &'a [usize],
    range: Range<usize>,
    succ_iter: SuccessorIterator<'a>,
}

impl<'a, G, N> AllSubPathCost<'a, G, N>
where
    G: ArcCost<N>,
    N: num_traits::Num,
{
    /**
     * Initialize an AllSubPathCost for graph *g* with
     * the given *path*. Each entry in *path* is the index
     * of a node in *g*, the first entry is considered
     * the start node and the last is the destination.
     * All the other nodes in the path are the visited
     * nodes in order.
     */
    pub fn new(g: G, path: &'a [usize]) -> Self {
        let curr = 0;
        let weight = N::zero();
        let range = 1..path.len();
        let succ_iter = SuccessorIterator::new(path.iter());
        Self {
            curr,
            graph: g,
            path,
            weight,
            range,
            succ_iter,
        }
    }

    fn get_next_arc(&mut self) -> Option<(usize, usize)> {
        if let Some(next) = self.succ_iter.next() {
            Some(next)
        } else {
            self.step_next_node()
        }
    }

    fn step_next_node(&mut self) -> Option<(usize, usize)> {
        self.curr = self.range.next()?;
        let sub_nodes = &self.path[self.curr..];
        self.succ_iter = SuccessorIterator::new(sub_nodes.iter());
        self.weight = N::zero();
        self.succ_iter.next()
    }
}

impl<'a, G, N> Iterator for AllSubPathCost<'a, G, N>
where
    G: ArcCost<N>,
    N: num_traits::Num + Copy,
{
    /**
     * In this tuple the first item is the source node,
     * second item is destination node and last is the
     * associated cost (length).
     */
    type Item = (usize, usize, N);

    fn next(&mut self) -> Option<Self::Item> {
        let (src, dst) = self.get_next_arc()?;
        let w = self.graph.cost(src, dst);
        self.weight = self.weight + w;
        Some((self.curr, dst, self.weight))
    }
}

struct SuccessorIterator<'a> {
    iter: Iter<'a, usize>,
    prev: Option<usize>,
}

impl<'a> SuccessorIterator<'a> {
    fn new(iter: Iter<'a, usize>) -> Self {
        Self { iter, prev: None }
    }
}

impl<'a> Iterator for SuccessorIterator<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (prev, curr) = if let Some(prev) = self.prev {
            (prev, *self.iter.next()?)
        } else {
            (*self.iter.next()?, *self.iter.next()?)
        };

        self.prev = Some(curr);

        Some((prev, curr))
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::Graph;
    use crate::MatrixGraph;

    #[test]
    fn test_path_cost_iterator() {
        let mut graph = MatrixGraph::<f64>::new_direct(4);
        graph.add_new_arc(0, 1, 1.0);
        graph.add_new_arc(1, 2, 2.0);
        graph.add_new_arc(2, 3, 3.0);
        graph.add_new_arc(3, 0, 4.0);

        let mut path_cost_iter = AllSubPathCost::new(&graph, &[0, 1, 2, 3]);
        assert_eq!(path_cost_iter.next(), Some((0, 1, 1.0)));
        assert_eq!(path_cost_iter.next(), Some((0, 2, 3.0)));
        assert_eq!(path_cost_iter.next(), Some((0, 3, 6.0)));
        assert_eq!(path_cost_iter.next(), Some((1, 2, 2.0)));
        assert_eq!(path_cost_iter.next(), Some((1, 3, 5.0)));
        assert_eq!(path_cost_iter.next(), Some((2, 3, 3.0)));
        assert_eq!(path_cost_iter.next(), None);
    }

    #[test]
    fn test_successor_iterator() {
        let elements = [1, 2, 3, 4, 5, 6];
        let mut iter = SuccessorIterator::new(elements.iter());
        assert_eq!(iter.next(), Some((1, 2)));
        assert_eq!(iter.next(), Some((2, 3)));
        assert_eq!(iter.next(), Some((3, 4)));
        assert_eq!(iter.next(), Some((4, 5)));
        assert_eq!(iter.next(), Some((5, 6)));
        assert_eq!(iter.next(), None);
    }
}
