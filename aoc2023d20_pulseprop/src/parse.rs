use crate::{Node, NodeType};
use std::collections::HashMap;

pub fn load_net(input: &str) -> HashMap<&str, Node> {
    let mut data: HashMap<&str, Node> = HashMap::new();
    let mut rev_data: HashMap<&str, HashMap<String, bool>> = HashMap::new();
    for line in input.lines() {
        let (type_name, destinations) = line.split_once(" -> ").unwrap();
        let destinations: Vec<&str> = destinations.split(", ").collect();
        let rev_name: &str;
        let (t, name) = match (&type_name[0..1], &type_name[1..]) {
            ("%", name) => {
                rev_name = name;
                (NodeType::FlipFlop(false), name)
            }
            ("b", "roadcaster") => {
                rev_name = type_name;
                (NodeType::Broadcaster, type_name)
            }
            ("&", name) => {
                rev_name = name;
                (NodeType::Conjunction(HashMap::new()), name)
            }

            _ => panic!("Unkown module type '{type_name}'"),
        };
        // populate rev_data to update Conjuntion with nodes pointing to it.
        for dst in &destinations {
            if let Some(hm) = rev_data.get_mut(dst) {
                hm.insert(rev_name.to_string(), false);
            } else {
                rev_data.insert(dst, HashMap::new());
            }
        }
        data.insert(
            name,
            Node {
                name,
                t,
                destinations,
            },
        );
    }
    // update Conjunction with rev looking info
    for (node_name, n) in data.iter_mut() {
        //if n.t == N
        if let NodeType::Conjunction(ref mut hs) = n.t {
            *hs = rev_data.remove(node_name).expect("Unknown key ?");
        }
    }

    data
}
