type Mapping = Vec<(String, u32)>;

fn first_digit(mut line: &str, mapping: &Mapping) -> u32 {
    loop {
        for (pattern, value) in mapping {
            if line.starts_with(pattern) {
                return *value;
            }
        }
        line = &line[1..];
    }
}


fn main() {
    let file = std::fs::read("inputs/day01.txt").unwrap();
    let lines: Vec<&str> = std::str::from_utf8(&file).unwrap().lines().collect();

    let mut values = Vec::new();
    for line in &lines {
        let first = line.chars().filter_map(|c| c.to_digit(10)).next().unwrap();
        let last = line.chars().rev().filter_map(|c| c.to_digit(10)).next().unwrap();
        values.push(first * 10 + last);
    }

    println!("First part: {}", values.iter().copied().sum::<u32>());

    let mapping: Mapping = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ].into_iter().map(|(pat, val)| (pat.to_string(), val)).collect();

    let rev_mapping: Mapping = mapping.iter().map(|(pat, val)| (pat.chars().rev().collect(), *val)).collect();

    let mut values2 = Vec::new();
    for line in &lines {
        let first = first_digit(line, &mapping);
        let last = first_digit(line.chars().rev().collect::<String>().as_str(), &rev_mapping);
        values2.push(first * 10 + last);
    }

    println!("Second part: {}", values2.iter().copied().sum::<u32>());
}
