/*!
 * Common interface to access a graph's topology.
 */

/**
 * Visit a graph's inner topology with callbacks.
 */
pub trait GraphVisitor<N>
where
    N: Copy,
{
    /**
     * Call function *f* for each node in the graph.
     * At each call the first argument is a node's index
     * and the second is the current node weight.
     */
    fn node_visitor<F: FnMut(usize, N)>(&self, f: F);

    /**
     * Call function *g* for each arc in the graph.
     * At each call the first argument is the source node index,
     * the second is destination node index and the third the current arc weight.
     */
    fn arc_visitor<G: FnMut(usize, usize, N)>(&self, g: G);

    /**
     * Return the number of nodes in the graph.
     */
    fn node_count(&self) -> usize;

    /**
     * Return the number of arcs in the graph.
     */
    fn arc_count(&self) -> usize;

    /**
     * Return the total number of nodes and arcs in the
     * graph.
     */
    fn total_entries(&self) -> usize {
        self.arc_count() + self.node_count()
    }
}
