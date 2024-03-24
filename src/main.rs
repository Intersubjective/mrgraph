use itertools::Itertools;
use meritrank::MyGraph;
use mrgraph::{GRAPH, GraphSingleton};

fn task() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // create merit rank
    let mut newrank = GraphSingleton::get_rank()?;
    let mut graph0 = GRAPH.lock()?;

    let node1 = graph0.get_node_id("node1");
    let node2 = graph0.get_node_id("node2");
    let node3 = graph0.get_node_id("node3");
    let node4 = graph0.get_node_id("node4");
    let node5 = graph0.get_node_id("node5");

    let mut graph: &mut MyGraph = graph0.borrow_graph_mut();

    // add nodes
    graph.add_node_by_id(node1);
    graph.add_node_by_id(node2);
    graph.add_node_by_id(node3);
    graph.add_node_by_id(node4);

    // graph.add_node(Node::new(5.into()));

    graph.add_edge(node1, node2, 0.98);
    graph.add_edge(node2, node3, 1.0);
    graph.add_edge(node3, node4, 1.0);
    // graph.add_edge(4.into(), 1.into(), 1.0);

    // graph.add_edge(3.into(), 4.into(), 1.0);
    // graph.add_edge(4.into(), 1.into(), 1.0);

    // calculate merit rank
    match newrank.calculate(1.into(), 100) {
        Ok(_) => {
            println!("Calculation successful.");
        }
        Err(e) => {
            eprintln!("Calculation Error: {}", e);
        }
    }

    let node_scores = newrank.get_node_score(node1, node5);

    println!("Node scores: {:?}", node_scores);

    newrank.add_node(node5);

    // add edges
    // newrank.add_edge(1.into(), 2.into(), 0.7);
    // newrank.add_edge(2.into(), 3.into(), 1.0);
    // newrank.add_edge(2.into(), 4.into(), 1.0);
    // newrank.add_edge(3.into(), 4.into(), 1.0);
    // newrank.add_edge(2.into(), 3.into(), -1.0);
    // newrank.add_edge(2.into(), 4.into(), 1.0);
    println!("Adding edges 2 -> 4");
    newrank.add_edge(node2, node4, 1.0);
    println!("Adding edges 3 -> 4");
    newrank.add_edge(node3, node4, -1.0);
    println!("Adding edges 4 -> 5");
    newrank.add_edge(node4, node5, 1.0);
    println!("Adding edges 3 -> 5");
    newrank.add_edge(node3, node5, -1.0);
    // newrank.add_edge(3.into(), 5.into(), 1.0);
    // newrank.add_edge(4.into(), 5.into(), 1.0);
    // newrank.add_edge(4.into(), 5.into(), 1.0);
    // newrank.add_edge(4.into(), 5.into(), 1.0);
    // newrank.add_edge(4.into(), 5.into(), 0.0);

    // calculate merit rank
    let ratings = newrank
        .get_ranks(1.into(), None)
        .unwrap_or_default();

    // print rating
    println!("Rating: {:?}", ratings);

    let (nodes, edges) = graph.all();
    nodes.iter().unique().for_each(|&a| println!("{:?} --> {}", a, graph0.node_id_to_name_unsafe(a).unwrap()));
    edges.iter().for_each(|(|a,b,w)| println!("{:?} -> {:?} : {w}", a, b));

    Ok(())
}

fn main() {
    println!("Hello, world!");

    task().unwrap();
}
