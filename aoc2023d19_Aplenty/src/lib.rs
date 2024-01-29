// pub mod nomnom;
// extern crate struct_workflow;
use std::collections::HashMap;
mod struct_workflow;

pub fn run() {
    let (workflow, parts) = read_input("in.txt");
    println!(
        "Run! part1: {part1} = [377025] and part2: {part2} = [135506683246673]",
        part1 = part1(workflow.clone(), parts),
        part2 = part2(workflow)
    );
}

fn part2(wf: HashMap<String, Vec<struct_workflow::Step>>) -> u64 {
    let mut pr = PartRange::new();
    part2_rec("in", &mut pr, &wf)
}
impl PartRange {
    fn new() -> PartRange {
        PartRange {
            part_min: Part {
                x: 1,
                m: 1,
                a: 1,
                s: 1,
            },
            part_max: Part {
                x: 4000,
                m: 4000,
                a: 4000,
                s: 4000,
            },
        }
    }
    fn get_total(&self) -> u64 {
        (self.part_max.x + 1 - self.part_min.x)
            * (self.part_max.m + 1 - self.part_min.m)
            * (self.part_max.a + 1 - self.part_min.a)
            * (self.part_max.s + 1 - self.part_min.s)
    }

    fn get_range_min_max(&self, var: &str) -> (u64, u64) {
        match var {
            "x" => (self.part_min.x, self.part_max.x),
            "m" => (self.part_min.m, self.part_max.m),
            "a" => (self.part_min.a, self.part_max.a),
            "s" => (self.part_min.s, self.part_max.s),
            _ => panic!(),
        }
    }
    fn gen_range_update(&self, var: &str, val_min: u64, val_max: u64) -> PartRange {
        let mut new = self.clone();
        match var {
            "x" => {
                new.part_min.x = val_min;
                new.part_max.x = val_max
            }
            "m" => {
                new.part_min.m = val_min;
                new.part_max.m = val_max
            }
            "a" => {
                new.part_min.a = val_min;
                new.part_max.a = val_max
            }
            "s" => {
                new.part_min.s = val_min;
                new.part_max.s = val_max
            }
            _ => panic!(),
        }
        new
    }

    fn update_less_than(
        &self,
        var_name: &str,
        value: u64,
    ) -> (Option<PartRange>, Option<PartRange>) {
        // e.g. x,m,a,s  var_name <1100 =true and <=1100 false
        let (val_min, val_max) = self.get_range_min_max(var_name);
        // true
        let range_true: Option<PartRange>;
        if val_min < value {
            // TRUE we can comply
            let val_max_new = value - 1;
            range_true = Some(self.gen_range_update(var_name, val_min, val_max_new));
        } else {
            range_true = None;
        };
        // false
        let range_false: Option<PartRange>;
        if val_max >= value {
            // FALSE we can comply
            let val_min_new = value;
            range_false = Some(self.gen_range_update(var_name, val_min_new, val_max));
        } else {
            range_false = None;
        };

        (range_true, range_false)
    }

    fn update_greater_than(
        &self,
        var_name: &str,
        value: u64,
    ) -> (Option<PartRange>, Option<PartRange>) {
        // e.g. x,m,a,s  var_name >1100 =true and <=1100 false
        let (val_min, val_max) = self.get_range_min_max(var_name);
        // true
        let range_true: Option<PartRange>;
        if val_max > value {
            // TRUE we can comply
            let val_min_new = value + 1;
            range_true = Some(self.gen_range_update(var_name, val_min_new, val_max));
        } else {
            range_true = None;
        };
        // false
        let range_false: Option<PartRange>;
        if val_min <= value {
            // FALSE we can comply
            let val_max_new = value;
            range_false = Some(self.gen_range_update(var_name, val_min, val_max_new));
        } else {
            range_false = None;
        };

        (range_true, range_false)
    }
}

#[derive(Clone)]
struct PartRange {
    part_min: Part,
    part_max: Part,
}

fn part2_rec(
    rule_name: &str,
    part_range: &mut PartRange,
    wf: &HashMap<String, Vec<struct_workflow::Step>>,
) -> u64 {
    // part2 no care about parts anymore try to find ranges and total options.
    // 1-4000 for each of the part values x m a s
    let mut total = 0;
    match rule_name {
        "A" => {
            // found valid option/part
            total += part_range.get_total();
            return total;
        }
        "R" => {
            // invalid
            return 0;
        }
        _ => (), // run rules for this label
    }
    let rules = wf
        .get(rule_name)
        .expect(&format!("Can't find rule name ? {rule_name}"));
    println!("part2a: rules: {rule_name}: {rules:?}");
    'next_rule: for (i_r, rule) in rules.iter().enumerate() {
        println!("part2b:   {rule_name}: rule:{i_r}: {rule:?}");
        let mut new_name = "";
        match rule {
            struct_workflow::Step::Condition(step_rule) => {
                // for single rule update range if it is true and 2nd range if it is false
                // e.g. a<2006:qkq, m>2090:A
                let (part_r_true, part_r_false) = match step_rule.comparison.as_str() {
                    ">" => part_range.update_greater_than(&step_rule.var, step_rule.value),
                    "<" => part_range.update_less_than(&step_rule.var, step_rule.value),
                    c => panic!("Invalid comparison operation ?{c}"),
                };

                if let Some(mut part_r_t) = part_r_true {
                    // 1/2 condition true , run with new name
                    total += part2_rec(&step_rule.next, &mut part_r_t, wf);
                };
                if let Some(part_r_f) = part_r_false {
                    // 2/2 condition false, next step
                    new_name = ""; // next rule in this set
                    *part_range = part_r_f;
                } else {
                    println!("        '{}' fail return {total}", step_rule.comparison);
                    return total;
                };
            }
            struct_workflow::Step::Final(final_name) => {
                new_name = &final_name;
            }
        }

        if new_name != "" {
            return total + part2_rec(new_name, part_range, wf);
        }
    }
    total
}

fn part1(wf: HashMap<String, Vec<struct_workflow::Step>>, parts: Vec<Part>) -> u64 {
    let mut total_cnt = 0;
    let mut total_part = 0;
    'next_part: for part in parts {
        println!("part1a: ##### part: {part:?}");
        let mut rule_name = "in";
        'next_rule_set: loop {
            let rules = wf.get(rule_name).unwrap();
            println!("part1b:       rules: {rule_name}: {rules:?}");
            'next_rule: for rule in rules {
                println!("part1c:         rule: {rule:?}");
                let mut new_name = "";
                match rule {
                    struct_workflow::Step::Condition(step_rule) => {
                        let part_specific_val = part.get_val(&step_rule.var);
                        match step_rule.comparison.as_str() {
                            ">" => {
                                if part_specific_val > step_rule.value {
                                    new_name = &step_rule.next;
                                }
                            }
                            "<" => {
                                if part_specific_val < step_rule.value {
                                    new_name = &step_rule.next;
                                }
                            }
                            c => panic!("Invalid comparison operation ?{c}"),
                        }
                    }
                    struct_workflow::Step::Final(final_name) => {
                        new_name = &final_name;
                    }
                }

                println!("part1d:        {rule_name} => {new_name} {total_cnt}");
                match new_name {
                    "A" => {
                        // found valid option/part
                        total_cnt += 1;
                        total_part += part.get_total();
                        continue 'next_part;
                    }
                    "R" => {
                        // invalid
                        continue 'next_part;
                    }
                    "" => {
                        // rule did not match, next rule
                        //panic!("Should be invalid, no rule matched above ?");
                        println!("          rule fail, next ...");
                        continue 'next_rule;
                    }
                    _ => {
                        // Set new name and loop for new set of rules
                        rule_name = new_name;
                        continue 'next_rule_set;
                    }
                }
            }
        }
    }
    total_part
}

#[derive(Debug, Clone)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}
impl Part {
    fn get_total(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
    fn get_val(&self, label: &str) -> u64 {
        match label {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("Invalid part type ? {label}"),
        }
    }
    fn parse(s: &str) -> Vec<Part> {
        let mut parts: Vec<Part> = Vec::new();
        for line in s.split("\n") {
            // {x=787,m=2655,a=1222,s=2876}
            //let line = line[1..]; // strip "{"
            let line = line.trim_start_matches("{").trim_end_matches("}");
            let mut part = Part {
                x: 0,
                m: 0,
                a: 0,
                s: 0,
            };
            for kv in line.split(",") {
                let (key, val) = kv.split_once("=").expect("Invalid part kv value ?");
                let val: u64 = val.parse().expect("val not number?");
                match key {
                    "x" => part.x = val,
                    "m" => part.m = val,
                    "a" => part.a = val,
                    "s" => part.s = val,
                    _ => panic!("Invalid part type ?"),
                }
            }
            parts.push(part);
        }
        parts
    }
}

fn read_input(file_name: &str) -> (HashMap<String, Vec<struct_workflow::Step>>, Vec<Part>) {
    let input = std::fs::read_to_string(file_name).expect("Unknown file ?");
    let (rules, parts) = input.split_once("\n\n").unwrap();
    let (_input, workflow_hash) = struct_workflow::Workflow::parse(&rules);
    let parts = Part::parse(parts);

    (workflow_hash, parts)
}
