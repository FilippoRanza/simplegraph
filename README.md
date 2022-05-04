# simplegraph
![Test](https://github.com/FilippoRanza/simplegraph/workflows/Rust/badge.svg) ![crates.io](https://img.shields.io/crates/v/simplegraph.svg)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=round-square)](http://makeapullrequest.com)

**simplegraph** provides a simple graph implementation 
in Rust. 

## Features
**simplegraph** aims to be as simple as possible while providing 
all the features that I need. 
You can consider using **simplegraph** if you need:
- support for direct and undirected graphs;
- [Adjacency List](https://en.wikipedia.org/wiki/Adjacency_list) or [Adjacency Matrix](https://en.wikipedia.org/wiki/Adjacency_matrix) based graphs;
- Graph to [Graphviz](https://en.wikipedia.org/wiki/Adjacency_list) (dot) source conversion;
- Serialization and Deserialization support with [Serde](https://serde.rs/);
- dynamic arc insertion;
- update arc's and nodes' weights. 

On the other side some intentional restriction are set on the graphs:
- the number of nodes is set at creation time 
- nodes' and arcs' weights *must* implement [num_traits::Num](https://docs.rs/num-traits/latest/num_traits/trait.Num.html)
- weights are always present: it is not possible to create a simple unweighted graph.

**simplegraph** does not provide any check on the operation performed on it. It is 
caller's responsibility to ensure operations soundness.

## Why?
To my best knowledge [petgrah](https://github.com/petgraph/petgraph) 
is the most used general purpose graph library for Rust. 
It is a very complete and complex library that allows to implement complex 
graphs for various purposes. 
If you choose/need to use a graph library you should check it out too. 

**simplegraph** aims to be a simple *wrap* around adjacency list or matrix. 
I've implemented this library mainly for my own use and avoid 
some - from my point of view - needless complexities. So it may 
lack some features that I'll add just when, and if, I'll need them. 
