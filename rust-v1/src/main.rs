use rust_v1::Graph;

fn main()
{
    let mut graph: Graph<String, i32> = Graph::new();
    let n_a = graph.add_node("Node A".to_string(), 1);
    let n_b = graph.add_node("Node B".to_string(), 2);

    let node_a = graph.find_node_by_id(n_a).unwrap();
    let node_b = graph.find_node_by_name(&"Node B".to_string()).unwrap();

    println!("Node A: {} {}", node_a.name, node_a.data);
    println!("Node B: {} {}", node_b.name, node_b.data);
}
