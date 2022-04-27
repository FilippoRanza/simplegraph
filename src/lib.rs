
macro_rules! enum_mut {
    ($list:expr) => {
        $list.iter_mut().enumerate()
    };
}

pub mod adjacency_list_graph;
pub mod matrix_graph;

pub enum GraphType {
    Direct,
    Undirect,
}

#[cfg(test)]
mod tests {}

fn empty_list_of_lists<T>(count: usize) -> Vec<Vec<T>> {
    (0..count).map(|_| vec![]).collect()
}

pub struct Node<N>(N);
impl<N> Default for Node<N>
where
    N: num_traits::Num + Default,
{
    fn default() -> Self {
        Self(Default::default())
    }
}
