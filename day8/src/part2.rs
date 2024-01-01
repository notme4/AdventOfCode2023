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
    //eprintln!("{:?}", result);
    return result;
}

fn steps_to_traverse(
    sequence: &str,
    start: Vec<&str>,
    at_end: fn(&str) -> bool,
    map: &HashMap<&str, Node>,
) -> u64 {
    let mut i: u64 = 0;
    let mut names = start;
    for c in sequence.chars().cycle() {
        if i > (sequence.len() * map.keys().len()).try_into().unwrap() {
            panic!("should have finished by now");
        }
        eprintln!(
            "{:?}",
            names
                .iter()
                .map(|val| if at_end(*val) { *val } else { "_" })
                .collect::<Vec<_>>()
        );

        if names.iter().all(|val| at_end(*val)) {
            break;
        }
        let mut j = 0;
        while j < names.len() {
            let mut name = names[j];
            name = traverse_step(c, name, map);
            names[j] = name;
            j += 1
        }
        i += 1;
    }
    return i;
}

fn traverse_step<'a>(step: char, name: &str, map: &HashMap<&str, Node<'a>>) -> &'a str {
    let new_name;
    match step {
        'L' => new_name = map[name].left,
        'R' => new_name = map[name].right,
        _ => panic!("invalid sequence char: {}", step),
    }
    //eprintln!("{} -> {}", name, new_name);
    return new_name;
}

fn ends_with(s: &str, c: char) -> bool {
    return s.chars().last().unwrap() == c;
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
    return steps_to_traverse(
        sequence,
        map.keys()
            .filter(|val| ends_with(val, 'A'))
            .map(|val| *val)
            .collect(),
        |val| ends_with(val, 'Z'),
        &map,
    );
}
