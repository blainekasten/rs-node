use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

struct State<'a> {
    current: Graph<'a>,
}

struct Graph<'a> {
    id: &'a str,
    nodes: Rc<HashMap<&'a str, Graph<'a>>>,
    parent: Option<&'a Graph<'a>>,
}

fn main() {
    let g = Graph {
        id: "123",
        nodes: Rc::new(HashMap::new()),
        parent: None,
    };
    let state = State { current: g };

    g.nodes.insert(
        "456",
        Graph {
            id: "456",
            nodes: Rc::new(HashMap::new()),
            parent: Some(&g),
        },
    );

    println!("{} {}", state.current.id, g.nodes.len());
}
