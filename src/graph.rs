use super::GraphType;

pub trait Graph<N> {
    fn new(node_count: usize, gtype: GraphType) -> Self;
    fn add_new_default_arc(&mut self, src: usize, dst: usize);
    fn add_new_arc(&mut self, src: usize, dst: usize, weight: N);

    fn update_all_arcs_weight<F>(&mut self, f: F)
    where
        F: Fn(usize, usize, N) -> N;

    fn update_all_nodes_weight<F>(&mut self, f: F)
    where
        F: Fn(usize, N) -> N;
}
