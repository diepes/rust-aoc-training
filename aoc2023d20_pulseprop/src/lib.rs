use std::collections::HashMap;
use std::collections::VecDeque; //fifo
mod parse;
pub fn run() {
    println!("lib.rs run()");
    let input = std::fs::read_to_string("in.txt").expect("Cant find file ?");
    let mut net = parse::load_net(&input);
    println!("net:{net:?}");
    let mut total_low = 0;
    let mut total_high = 0;
    for _button in 0..1000 {
        let (low, high) = press_button_low(&mut net);
        println!("low:{low} high:{high}");
        total_low += low;
        total_high += high;
        println!();
    }
    println!(
        "total_low:{total_low} x total_high: {total_high} ={}  == #1[32000000]c#2[11687500]",
        total_low * total_high
    );
}
#[derive(Debug)]
pub enum NodeType {
    Button,                             // start lowpulse on press
    Broadcaster,                        //repeat in to all out
    FlipFlop(bool), //% initially off, high ->ignore, _low -> flips, sends on->high, off->_low
    Conjunction(HashMap<String, bool>), //& remembers input for each src, default _low, if all in high->_low else ->high
    Output,
}
#[derive(Debug)]
pub struct Node<'a> {
    name: &'a str,
    t: NodeType,
    destinations: Vec<&'a str>,
}
struct PulseQ {
    q: VecDeque<(String, bool, String)>,
    cnt_pulse_low: u64,
    cnt_pulse_high: u64,
}
impl PulseQ {
    fn new() -> PulseQ {
        PulseQ {
            q: VecDeque::new(),
            cnt_pulse_low: 0,
            cnt_pulse_high: 0,
        }
    }
    fn push_front(&mut self, src: &str, pulse: bool, dst: &str) {
        self.q.push_front((src.to_string(), pulse, dst.to_string()));
        if pulse {
            self.cnt_pulse_high += 1;
        } else {
            self.cnt_pulse_low += 1;
        }
    }
    fn pop_back(&mut self) -> Option<(String, bool, String)> {
        let q = self.q.pop_back();
        //let level = if pulse { "high" } else { "low" };
        //println!("{src} -{level}-> {dst}");
        println! {"q: {q:?}"};
        q
    }
}
fn press_button_low(net: &mut HashMap<&str, Node>) -> (u64, u64) {
    // a single low pulse is sent directly to the broadcaster module
    let mut pulseq = PulseQ::new();
    pulseq.push_front("button", false, "broadcaster"); //_low pulse sent
    let mut _cnt = 0;
    loop {
        _cnt += 1;
        // run until pulseq empty/no changes
        let q = pulseq.pop_back();
        let (name_src, pulse, name_dst) =
            q.unwrap_or_else(|| ("empty".to_string(), true, "empty".to_string()));
        if name_dst == "empty".to_string() {
            return (pulseq.cnt_pulse_low, pulseq.cnt_pulse_high);
        };
        if let Some(node) = net.get_mut(name_dst.as_str()) {
            //.expect(&format!("Missing module {name_dst}"));
            // node pulse
            match &mut node.t {
                NodeType::Button | NodeType::Output => panic!("Unexpected"),
                NodeType::Broadcaster => {
                    for dst in &node.destinations {
                        pulseq.push_front("broadcaster", pulse, dst);
                    }
                }
                NodeType::FlipFlop(ref mut onoff) => {
                    if pulse == true {
                        // ignore high pulse
                    } else {
                        // flip on off then send
                        *onoff ^= true;
                        for dst in &node.destinations {
                            pulseq.push_front(node.name, *onoff, dst);
                        }
                    };
                }
                NodeType::Conjunction(hashm) => {
                    // . ?????
                    //let _prev_state = hashm.get(name_src.as_str()).unwrap_or_else(|| &false);
                    hashm.insert(name_src.clone(), pulse); // remember src
                                                           // calc pulse to send
                                                           // if all in high->_low else ->high
                    let mut all_high = true;
                    let mut msg: String = Default::default();
                    for (dst, state) in hashm.iter() {
                        msg += &format!(" dst:{dst}={state},");
                        if state == &false {
                            all_high = false;
                        };
                    }
                    let send_pulse = if all_high { false } else { true };
                    println!(
                        "DEBUG Conjuntion[{n}] got pulse:{pulse} -all_high:{all_high}-> {send_pulse} msg:{msg}",n=node.name
                    );

                    for dst in &node.destinations {
                        pulseq.push_front(node.name, send_pulse, dst);
                    }
                }
            }
        }
        //  else {
        //     return (pulseq.cnt_pulse_low, pulseq.cnt_pulse_high);
        // };
    }
}
