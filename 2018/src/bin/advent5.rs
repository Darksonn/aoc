use advent2018::DataReader;
use std::collections::VecDeque;

fn main() {
    let mut input = DataReader::open(5);
    let mut polymer = advent2018::time("Read input", || {
        Polymer::new(input.next().unwrap())
    });

    let alive = advent2018::time("Fully react", || {
        fully_react(&mut polymer)
    });

    let best_length = advent2018::time("Fully react letter", || {
        let mut min = std::usize::MAX;
        for letter in ('a' as u8) ..= ('z' as u8) {
            polymer.revive_all_but(letter);
            let len = fully_react(&mut polymer);
            if len < min {
                min = len;
            }
        }
        min
    });

    println!("Part one: {}", alive);
    println!("Part two: {}", best_length);
}


fn fully_react(polymer: &mut Polymer) -> usize {
    let mut stack = VecDeque::new();
    let mut last = None;
    for i in 0..polymer.len() {
        if !polymer.destroyed[i] {
            if let Some(j) = last {
                stack.push_back((j, i));
            }
            last = Some(i);
        }
    }

    while let Some((from, to)) = stack.pop_front() {
        if polymer.destroyed[from] { continue; }
        if polymer.destroyed[to] { continue; }
        if reacts(polymer.codes[from], polymer.codes[to]) {
            polymer.destroyed[from] = true;
            polymer.destroyed[to] = true;
            if let Some(left) = polymer.scan_left(from) {
                if let Some(right) = polymer.scan_right(to) {
                    stack.push_back((left, right));
                }
            }
        }
    }
    let mut count = 0;
    for &is_des in &polymer.destroyed {
        if !is_des {
            count += 1;
        }
    }
    count
}

#[inline]
fn reacts(a: u8, b: u8) -> bool {
    let space = ' ' as u8;
    (a ^ space) == b
}

#[derive(Clone,Debug)]
struct Polymer {
    codes: Vec<u8>,
    destroyed: Vec<bool>,
}
impl Polymer {
    pub fn new(polymer: String) -> Self {
        let len = polymer.len();
        Polymer {
            codes: polymer.into(),
            destroyed: vec![false; len],
        }
    }
    pub fn len(&self) -> usize {
        self.codes.len()
    }
    pub fn revive_all_but(&mut self, dead: u8) {
        for (ptr, code) in self.destroyed.iter_mut().zip(self.codes.iter().cloned()) {
            *ptr = code == dead || reacts(code, dead);
        }
    }
    pub fn scan_left(&self, mut i: usize) -> Option<usize> {
        while i > 0 {
            i -= 1;
            if !self.destroyed[i] {
                return Some(i);
            }
        }
        None
    }
    pub fn scan_right(&self, mut i: usize) -> Option<usize> {
        while i < self.len() {
            if !self.destroyed[i] {
                return Some(i);
            }
            i += 1;
        }
        None
    }
}
