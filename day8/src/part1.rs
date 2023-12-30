use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn create_node(line: &str) -> Node {
    let (name, sides) = line.split_at(line.find("=").unwrap());
    let (left, right) = sides.split_at(sides.find(",").unwrap());
    let (_, left) = left.split_at(left.find("(").unwrap() + 1);
    let (_, right) = right.split_at(right.find(",").unwrap() + 1);
    let (right, _) = right.split_at(right.find(")").unwrap());

    let result = Node {
        name: name.trim(),
        left: left.trim(),
        right: right.trim(),
    };
    eprintln!("{:?}", result);
    return result;
}

fn steps_to_traverse(
    sequence: &str,
    start: &str,
    at_end: fn(&str) -> bool,
    map: &HashMap<&str, Node>,
) -> u64 {
    let mut i = 0;
    let mut name = start;
    for c in sequence.chars().cycle() {
        if at_end(name) {
            break;
        }
        match c {
            'L' => name = map[name].left,
            'R' => name = map[name].right,
            _ => panic!("invalid sequence char: {}", c),
        }
        i += 1;
    }
    return i;
}

pub fn func(path: &Path) -> u64 {
    let file_data = read_to_string(path).unwrap();
    let mut lines = file_data.lines();
    let sequence = lines.next().unwrap();
    let _ = lines.next();
    let nodes: Vec<Node> = lines.map(create_node).collect();
    let mut map = HashMap::<&str, Node>::new();
    for node in nodes {
        map.insert(node.name, node);
    }
    return steps_to_traverse(sequence, "AAA", |val: &str| val == "ZZZ", &map);
}
