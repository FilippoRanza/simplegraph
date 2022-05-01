pub trait UpdateNodes<N> {
    fn update_all_nodes_weight_iter<I>(&mut self, iter: I)
    where
        I: Iterator<Item = N>;

    fn update_indexed_nodes_weight<I>(&mut self, iter: I)
    where
        I: Iterator<Item = (usize, N)>;
}
