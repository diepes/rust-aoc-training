use std::collections::HashMap;
use std::collections::VecDeque; //fifo
mod lcm;
mod parse;
pub fn run() {
    println!("lib.rs run()");
    let input = std::fs::read_to_string("in.txt").expect("Cant find file ?");
    let mut net = parse::load_net(&input);
    //println!("net:{net:?}");
    let mut total_low = 0;
    let mut total_high = 0;
    for button in 1..1_000_0 {
        let (low, high) = press_button_low(&mut net, button);
        // println!("low:{low} high:{high}");
        total_low += low;
        total_high += high;
        // println!();
    }
    println!(
        "total_low:{total_low} x total_high: {total_high} ={}  == #1[32000000]c#2[11687500]",
        total_low * total_high
    );

    // todo!() no good way to figure out the loops, grab them from output above and calc lcm. 212986464842911 ✅
    let mut  p: u64 = 1;
    for n in [3761, 3793, 3847, 3881] {
        let lcm = lcm::lcm(p,n);
        println!("LCM {p}:{n} >> {lcm}");
        p = lcm;
    }
}
#[derive(Debug)]
struct Conjunction {
    in_state_hash: HashMap<String, bool>,
    last_button_press_high: u64,
    last_cycle: u64,
    inverter: bool, // true if only one feed in
}
#[derive(Debug)]
pub enum NodeType {
    Button,         // start lowpulse on press
    Broadcaster,    //repeat in to all out
    FlipFlop(bool), //% initially off, high ->ignore, _low -> flips, sends on->high, off->_low
    //& remembers input for each src, default _low, if all in high->_low else ->high
    Conjunction(Conjunction),
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
        //println! {"q: {q:?}"};
        q
    }
}
fn press_button_low(net: &mut HashMap<&str, Node>, button: u64) -> (u64, u64) {
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
                // NodeType::Conjunction(hashm,previous_button_count) => {
                NodeType::Conjunction(conj) => {
                    // if all in high->_low else ->high
                    conj.in_state_hash.insert(name_src.clone(), pulse); // update specific source pulse lvl
                                                                        //
                    let mut all_high = true;
                    let last_button_count = conj.last_button_press_high.clone();
                    let mut msg: String = Default::default();
                    for (src, state) in conj.in_state_hash.iter() {
                        msg += &format!(" src:{src}={state},");
                        if state == &false {
                            all_high = false;
                        };
                    }

                    let send_pulse = if all_high { false } else { true };
                    if all_high {
                        conj.last_button_press_high = button;
                    };

                    // println!(
                    //     "DEBUG Conjuntion[{n}] got pulse:{pulse} -all_high:{all_high}-> {send_pulse} msg:{msg}",n=node.name
                    // );

                    for dst in &node.destinations {
                        pulseq.push_front(node.name, send_pulse, dst);
                        //if all_high && (dst == &"ql" || dst == &"rx") {
                    }
                    let this_cycle = button - last_button_count;
                    if all_high && this_cycle > 1 && (this_cycle == conj.last_cycle) {
                        //if this_cycle > 1 {
                        println!(
                                "Conj: {name_dst} {nn}->{nd:?} inv:{inv} all_high fire _low at button press cycle: this[{this_cycle}] last[{last_cycle}]",
                                nn = node.name,
                                nd = node.destinations,
                                inv = conj.inverter,
                                last_cycle = conj.last_cycle,
                            );
                    } else if all_high && this_cycle > 1 {
                        println!("Cycle changed for {name_dst} {this_cycle}")
                    }
                    if all_high {
                        conj.last_button_press_high = button;
                        conj.last_cycle = this_cycle;
                    }
                }
            }
        } else {
            //println!("Unknown dst {name_dst} pulse:{pulse} button press: {button}");
            if !pulse {
                panic!("Stop low pulse! {button}");
            }
        };
    }
}
