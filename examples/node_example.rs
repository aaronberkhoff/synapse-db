use synapse_db::node::Node;

fn main() {
    println!("Synapse DB - Node Example");

    let node = Node::new(1, None, 0, "test", None);

    println!("Node initialized: {:?}", std::any::type_name_of_val(&node));
}
