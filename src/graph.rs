/*!
 * Update a graph's arcs and weights.
 */

use super::GraphType;

/**
 * This trait provide a common interface
 * to create and update a graph.
 */
pub trait Graph<N> {
    /**
     * Initialize a new directed or undirecte graph with the
     * given number of nodes.
     */
    fn new(node_count: usize, gtype: GraphType) -> Self;

    /**
     * Create a new arc from ```src``` to ```dst``` (and from ```dst``` to ```src``` if the graph is undirected)
     * and associtate with this new arc the cost ```num_traits::Num::zero()```
     */
    fn add_new_default_arc(&mut self, src: usize, dst: usize);

    /**
     * Create a new arc from ```src``` to ```dst``` (and from ```dst``` to ```src``` if the graph is undirected)
     * and associtate with this new arc the cost weight
     */
    fn add_new_arc(&mut self, src: usize, dst: usize, weight: N);

    /**
     * Update all arcs weight using the given callback function.
     * At each function call the first argument is the index
     * of the source node, the second is the index of the
     * destination node and the third is the current weight
     * of the arc.
     * Note: in undirect graph the arc (i, j) is equivalent to the node
     * (j, i) and the implementation should, for efficiency and reduce
     * mistakes, call *f* for just one arc.
     */
    fn update_all_arcs_weight<F>(&mut self, f: F)
    where
        F: Fn(usize, usize, N) -> N;

    /**
     * Update all nodes weights using the given callback
     * function. The first argument to each call is the
     * index of the node and the second is its current weight.
     *
     */
    fn update_all_nodes_weight<F>(&mut self, f: F)
    where
        F: Fn(usize, N) -> N;
}
