pub trait GraphVisitor<N>
where
    N: Copy,
{
    fn node_visitor<F: FnMut(usize, N)>(&self, f: F);
    fn arc_visitor<G: FnMut(usize, usize, N)>(&self, g: G);
    fn node_count(&self) -> usize;
    fn arc_count(&self) -> usize;
    fn total_entries(&self) -> usize {
        self.arc_count() + self.node_count()
    }
}
