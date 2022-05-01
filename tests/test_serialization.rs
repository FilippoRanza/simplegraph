use simplegraph;
use simplegraph::Graph;

#[test]
fn test_adjlist_serialization() {
    run_serialiation(simplegraph::AdjList::new_direct);
    run_serialiation(simplegraph::AdjList::new_undirect);
}

fn run_serialiation<F>(f: F)
where
    F: Fn(usize) -> simplegraph::AdjList<usize>,
{
    let orig_graph = make_undirect_graph(f);
    let graph_json = serde_json::to_string(&orig_graph).unwrap();
    let tmp_graph: simplegraph::MatrixGraph<usize> = serde_json::from_str(&graph_json).unwrap();
    let graph_json = serde_json::to_string(&tmp_graph).unwrap();
    let new_graph = serde_json::from_str(&graph_json).unwrap();
    assert_eq!(orig_graph, new_graph);
}

fn make_undirect_graph<F>(f: F) -> simplegraph::AdjList<usize>
where
    F: Fn(usize) -> simplegraph::AdjList<usize>,
{
    let mut graph = f(4);
    graph.update_all_nodes_weight(|i, _| i);
    graph.add_new_arc(0, 1, 1);
    graph.add_new_arc(1, 2, 2);
    graph.add_new_arc(2, 3, 3);

    graph
}
