use crate::{Conjunction, Node, NodeType};
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
                (
                    NodeType::Conjunction(Conjunction {
                        in_state_hash: HashMap::new(),
                        last_button_press_high: 0,
                        last_cycle: 0,
                        inverter: false,
                    }),
                    name,
                )
            }

            _ => panic!("Unkown module type '{type_name}'"),
        };
        // now we got name , t(type) -> destinations
        // populate rev_data to update Conjuntion with nodes pointing to it.
        for dst in &destinations {
            if let Some(hm_inner) = rev_data.get_mut(dst) {
                hm_inner.insert(rev_name.to_string(), false); //remember initial _low
            } else {
                rev_data.insert(dst, HashMap::from([(rev_name.to_string(), false)]));
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
        //if let NodeType::Conjunction(ref mut hs, last_button_low) = n.t {
        if let NodeType::Conjunction(ref mut conj) = n.t {
            conj.in_state_hash = rev_data.remove(node_name).expect("Unknown key ?");
            if conj.in_state_hash.len() == 1 {
                conj.inverter = true; // single input conj behaves as inverter
            } else {
                //panic!("Fail {node_name} {} {:?}",conj.in_state_hash.len(),conj.in_state_hash)
            }
        }
    }

    data
}
