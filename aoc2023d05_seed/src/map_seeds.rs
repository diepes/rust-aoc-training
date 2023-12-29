use crate::nom_parse;

pub fn map(
    seed_num: u64,
    from_orig: &str,
    to_final: &str,
    almanac: &nom_parse::Almanac,
    debug: bool,
) -> (u64, String) {
    let mut from = from_orig;
    let mut to = "";
    let mut num = seed_num;
    let mut cnt = 0;

    while to != to_final {
        cnt += 1;
        let map = find_map_from(from, almanac);
        if debug {
            println!(" found map to use {}", map.name);
        }
        to = &map.to;
        let new_num = convert_map_num(num, &map, debug);
        if debug {
            println! {"{cnt}_{seed_num}: {num}[{from}] > {new_num}[{to}] using map:{name}\n",from=map.from,to=map.to,name=map.name};
        };
        from = to; //This is current type
        num = new_num
    }
    (num, to.to_string())
}

fn find_map_from<'a>(from: &str, almanac: &'a nom_parse::Almanac) -> &'a nom_parse::Map {
    for map in &almanac.maps {
        if map.from == from {
            return &map;
        }
    }
    panic!(" find_map called to lookd for from: {from} that does not exist?");
}

fn convert_map_num(num: u64, map: &nom_parse::Map, debug: bool) -> u64 {
    // look for matching mapping
    for entry in &map.entries {
        if num >= entry.src && num <= entry.src_max {
            if debug {
                println!("convert_map entry used {:#?}", entry);
            };
            return entry.dst + (num - entry.src);
        }
    }
    if debug {
        println!("convert_map entry 1:1");
    };
    num
}
