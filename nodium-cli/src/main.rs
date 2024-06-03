use std::collections::HashMap;

struct Nodium {
    configs: HashMap<String, Config>,
    nodes: HashMap<String, Node>,
    connections: Vec<Connection>,
}

struct Config {
    name: String,
    description: String,
    keywords: Vec<String>,
}

struct Node {
    name: String,
    description: String,
    keywords: Vec<String>,
    inputs: Vec<Data>,
    outputs: Vec<Data>,
}

struct Data {
    name: String,
    data_type: String,
    time: u64,
    value: String,
}

struct Connection {
    from: String,
    to: String,
}

fn main() {
    let mut node = Node {
        name: "My Node".to_string(),
        description: "This is my node".to_string(),
        keywords: Vec::new(),
        inputs: Vec::new(),
        outputs: Vec::new(),
    };

    node.keywords.push("keyword1".to_string());
    node.keywords.push("keyword2".to_string());
}
