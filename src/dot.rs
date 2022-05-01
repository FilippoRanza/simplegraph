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
    add_arc_check: &'static dyn Fn(usize, usize) -> bool,
    str_buff: Vec<String>,
    arrow: &'static str,
}

impl BuildBody {
    fn new(
        size: usize,
        arrow: &'static str,
        add_arc_check: &'static dyn Fn(usize, usize) -> bool,
    ) -> Self {
        let str_buff = Vec::with_capacity(size);
        Self {
            str_buff,
            arrow,
            add_arc_check,
        }
    }

    fn add_node<N: fmt::Display>(&mut self, i: usize, n: N) {
        let node_stmt = format!("\tn{i} [label=\"{n}\"];");
        self.str_buff.push(node_stmt);
    }

    fn add_arc<N: fmt::Display>(&mut self, i: usize, j: usize, n: N) {
        if (self.add_arc_check)(i, j) {
            let node_stmt = format!("\tn{} {} n{} [label=\"{}\"];", i, self.arrow, j, n);
            self.str_buff.push(node_stmt);
        }
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
    let f = get_arc_insert_logic(g);
    let count = g.total_entries();
    let mut str_builder = BuildBody::new(count, arrow, f);
    g.node_visitor(|i, n| str_builder.add_node(i, n));
    g.arc_visitor(|i, j, n| str_builder.add_arc(i, j, n));
    str_builder.build_str()
}

fn get_arc_insert_logic<G: GetGraphType>(g: G) -> &'static dyn Fn(usize, usize) -> bool {
    select(g, &|_, _| true, &|i, j| i <= j)
}

fn get_arrow<G: GetGraphType>(g: G) -> &'static str {
    select(g, "->", "--")
}

fn get_graph_type<G: GetGraphType>(g: G) -> &'static str {
    select(g, "digraph", "graph")
}

fn select<G: GetGraphType, T>(g: G, direct: T, undirect: T) -> T {
    match g.graph_type() {
        GraphType::Direct => direct,
        GraphType::Undirect => undirect,
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::Graph;
    use crate::adjacency_list_graph;

    #[test]
    fn test_dot_build_direct_graph() {
        let mut graph = adjacency_list_graph::AdjList::new_direct(4);
        graph.add_new_arc(0, 1, 1.5);
        graph.add_new_arc(1, 2, 2.5);
        graph.add_new_arc(3, 2, 11.5);
        graph.add_new_arc(1, 0, -1.5);

        let dot_code = to_dot_source(&graph);
        let expect = "digraph {\n\tn0 [label=\"0\"];\n\tn1 [label=\"0\"];\n\tn2 [label=\"0\"];\n\tn3 [label=\"0\"];\n\tn0 -> n1 [label=\"1.5\"];\n\tn1 -> n2 [label=\"2.5\"];\n\tn1 -> n0 [label=\"-1.5\"];\n\tn3 -> n2 [label=\"11.5\"];\n}";
        assert_eq!(dot_code, expect)
    }

    #[test]
    fn test_dot_build_undirect_graph() {
        let mut graph = adjacency_list_graph::AdjList::new_undirect(4);
        graph.add_new_arc(0, 1, 1.5);
        graph.add_new_arc(1, 2, 2.5);
        graph.add_new_arc(3, 2, 11.5);

        let dot_code = to_dot_source(&graph);
        let expect = "graph {\n\tn0 [label=\"0\"];\n\tn1 [label=\"0\"];\n\tn2 [label=\"0\"];\n\tn3 [label=\"0\"];\n\tn0 -- n1 [label=\"1.5\"];\n\tn1 -- n2 [label=\"2.5\"];\n\tn2 -- n3 [label=\"11.5\"];\n}";
        assert_eq!(dot_code, expect)
    }
}
