use super::visitor;
use super::{GetGraphType, GraphType};
use std::fmt;

pub fn to_dot_source<G, N>(g: G) -> String
where
    G: visitor::GraphVisitor<N> + GetGraphType + Copy,
    N: num_traits::Num + Default + Clone + Copy + std::fmt::Display,
{
    let body = build_body(g);
    let gtype = get_graph_type(g);
    format!("{gtype} {{\n{body}\n}}")
}

struct BuildBody {
    str_buff: Vec<String>,
    arrow: &'static str,
}

impl BuildBody {
    fn new(size: usize, arrow: &'static str) -> Self {
        let str_buff = Vec::with_capacity(size);
        Self { str_buff, arrow }
    }

    fn add_node<N: fmt::Display>(&mut self, i: usize, n: N) {
        let node_stmt = format!("\tn{i} [label=\"{n}\"];");
        self.str_buff.push(node_stmt);
    }

    fn add_arc<N: fmt::Display>(&mut self, i: usize, j: usize, n: N) {
        let node_stmt = format!("\tn{} {} n{} [label=\"{}\"];", i, self.arrow, j, n);
        self.str_buff.push(node_stmt);
    }

    fn build_str(self) -> String {
        self.str_buff.join("\n")
    }
}

fn build_body<G, N>(g: G) -> String
where
    G: visitor::GraphVisitor<N> + GetGraphType + Copy,
    N: num_traits::Num + Default + Clone + Copy + std::fmt::Display,
{
    let arrow = get_arrow(g);
    let mut str_builder = BuildBody::new(0, arrow);
    g.node_visitor(|i, n| str_builder.add_node(i, n));
    g.arc_visitor(|i, j, n| str_builder.add_arc(i, j, n));
    str_builder.build_str()
}

fn get_arrow<G: GetGraphType>(g: G) -> &'static str {
    select_str(g, "->", "--")
}

fn get_graph_type<G: GetGraphType>(g: G) -> &'static str {
    select_str(g, "digraph", "graph")
}

fn select_str<G: GetGraphType>(g: G, direct: &'static str, undirect: &'static str) -> &'static str {
    match g.graph_type() {
        GraphType::Direct => direct,
        GraphType::Undirect => undirect,
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{adjacency_list_graph, matrix_graph};

    #[test]
    fn test_dot_build_adj_list() {
        let mut graph = adjacency_list_graph::AdjList::new_direct(4);
        graph.add_new_arc(0, 1, 1.5);
        graph.add_new_arc(1, 2, 2.5);
        graph.add_new_arc(3, 2, 11.5);
        graph.add_new_arc(1, 0, -1.5);

        let dot_code = to_dot_source(&graph);
        let expect = "digraph {\n\tn0 [label=\"0\"];\n\tn1 [label=\"0\"];\n\tn2 [label=\"0\"];\n\tn3 [label=\"0\"];\n\tn0 -> n1 [label=\"1.5\"];\n\tn1 -> n2 [label=\"2.5\"];\n\tn1 -> n0 [label=\"-1.5\"];\n\tn3 -> n2 [label=\"11.5\"];\n}";
        assert_eq!(dot_code, expect)
    }
}
