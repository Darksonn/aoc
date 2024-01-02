struct Number {
    x: usize,
    y: usize,
    value: u64,
    is_symbol: bool,
}

fn is_symbol(c: u8) -> bool {
    !(c == b'.' || (c as char).is_digit(10))
}

fn lookup(input: &Vec<Vec<Option<usize>>>, x: isize, y: isize) -> Option<usize> {
    if y < 0 || y >= input.len() as isize { return None; }
    let y = y as usize;
    if x < 0 || x >= input[y].len() as isize { return None; }
    let x = x as usize;
    input[y][x]
}

fn main() {
    let input = std::fs::read("inputs/day03.txt").unwrap();

    let input: Vec<Vec<u8>> = input
        .split(|c| *c == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.to_vec())
        .collect();

    let height = input.len();
    let width = input[0].len();

    let mut numbers = Vec::new();
    let mut coord_to_number = vec![vec![None; width]; height];

    // Find numbers.
    for y in 0..height {
        let mut curr_number = None;
        for x in 0..width {
            if (input[y][x] as char).is_digit(10) {
                if curr_number.is_none() {
                    curr_number = Some(numbers.len());
                    numbers.push(Number {
                        x,
                        y,
                        value: 0,
                        is_symbol: false,
                    });
                }
            } else {
                curr_number = None;
            }
            coord_to_number[y][x] = curr_number;
        }
    }

    // Compute value of each number.
    for number in &mut numbers {
        let mut value = 0;
        for i in number.x .. width {
            if let Some(digit) = (input[number.y][i] as char).to_digit(10) {
                value = 10 * value + (digit as u64);
            } else {
                break;
            }
        }
        number.value = value;
    }

    // Mark symbols.
    for y in 0..height as isize {
        for x in 0..width as isize {
            if is_symbol(input[y as usize][x as usize]) {
                for (y2,x2) in [(y-1,x-1), (y,x-1), (y+1,x-1), (y-1,x), (y,x), (y+1,x), (y-1,x+1), (y,x+1), (y+1,x+1)] {
                    if let Some(num) = lookup(&coord_to_number, x2, y2) {
                        numbers[num].is_symbol = true;
                    }
                }
            }
        }
    }

    let part1: u64 = numbers.iter()
        .filter(|num| num.is_symbol)
        .map(|num| num.value)
        .sum();
    println!("First part: {}", part1);

    let mut ratios = Vec::new();
    for y in 0..height as isize {
        for x in 0..width as isize {
            if is_symbol(input[y as usize][x as usize]) {
                let mut adjacent_numbers = Vec::new();
                for (y2,x2) in [(y-1,x-1), (y,x-1), (y+1,x-1), (y-1,x), (y,x), (y+1,x), (y-1,x+1), (y,x+1), (y+1,x+1)] {
                    if let Some(num) = lookup(&coord_to_number, x2, y2) {
                        adjacent_numbers.push(num);
                    }
                }

                // We might find the same number twice. Remove duplicates.
                adjacent_numbers.sort_unstable();
                adjacent_numbers.dedup();

                if let &[n1, n2] = adjacent_numbers.as_slice() {
                    ratios.push(numbers[n1].value * numbers[n2].value);
                }
            }
        }
    }

    let part2: u64 = ratios.iter().copied().sum();
    println!("Second part: {}", part2);
}
