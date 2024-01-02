use aoc2023::parse_iter;
use aoc2023::range_map::*;

fn main() {
    let input = aoc2023::load_input(5);

    let mut lines = input.lines();
    let seeds = lines.next().unwrap().strip_prefix("seeds: ").unwrap();
    let seeds: Vec<u64> = parse_iter(seeds.split_ascii_whitespace());

    assert!(lines.next().unwrap().is_empty());

    let mut maps = Vec::<RangeMap>::new();
    let mut curr_builder = RangeMapBuilder::new();
    for line in lines {
        if line.ends_with(" map:") { continue; }
        if line.is_empty() {
            maps.push(curr_builder.build());
            curr_builder = RangeMapBuilder::new();
            continue;
        }

        let line: Vec<u64> = parse_iter(line.split_ascii_whitespace());
        curr_builder.add_range(line[1], line[0], line[2]);
    }
    if !curr_builder.is_empty() {
        maps.push(curr_builder.build());
    }

    let locations: Vec<_> = seeds.iter()
        .map(|seed| {
            let mut curr = *seed;
            for map in &maps {
                curr = map.get(curr).unwrap_or(curr);
            }
            curr
        })
        .collect();

    println!("part 1: {}", locations.iter().copied().min().unwrap());

    let mut ranges: Vec<Range> = (0 .. seeds.len() / 2)
        .map(|i| Range { from: seeds[2*i], len: seeds[2*i+1] })
        .collect();

    for map in &maps {
        ranges = ranges.into_iter()
            .flat_map(|r| map.map_range(r)
                      .into_iter()
                      .map(|(r, mr)| mr.unwrap_or(r)))
            .collect();
    }

    let min_loc = ranges.iter().map(|r| r.from).min().unwrap();
    println!("part 2: {}", min_loc);
}
