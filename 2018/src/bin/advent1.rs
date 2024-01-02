use advent2018::DataReader;
use fnv::FnvHashSet;

fn main() {
    let input = DataReader::open(1);

    let numbers = advent2018::time("Read input", || {
        let mut numbers: Vec<i64> = Vec::new();
        for line in input {
            numbers.push(line.parse().unwrap());
        }
        numbers
    });

    let sum: i64 = advent2018::time("Sum", || {
        numbers.iter().cloned().sum()
    });
    let first_dup = advent2018::time("First duplicate", || {
        first_duplicate(&numbers[..])
    });
    println!("Part one: {}", sum);
    println!("Part two: {}", first_dup);
}

fn first_duplicate(numbers: &[i64]) -> i64 {
    let mut seen = FnvHashSet::default();
    let mut iter = LoopingIterator::new(numbers);
    let mut sum = 0;
    loop {
        let value = iter.get();
        sum += value;
        if seen.contains(&sum) {
            return sum;
        }
        seen.insert(sum);
    }
}

/// An iterator that repeats an array infinitely.
pub struct LoopingIterator<'a, T: Clone> {
    values: &'a [T],
    i: usize,
}
impl<'a, T: Clone> LoopingIterator<'a, T> {
    pub fn new(values: &'a [T]) -> Self {
        if values.len() == 0 {
            panic!("Can't loop empty array.");
        }
        LoopingIterator {
            values,
            i: 0,
        }
    }
    /// This method allows not having to unwrap Options.
    pub fn get(&mut self) -> T {
        match self.values.get(self.i) {
            Some(v) => {
                self.i += 1;
                v.clone()
            },
            None => {
                self.i = 1;
                self.values[0].clone()
            },
        }
    }
}
impl<'a, T: Clone> Iterator for LoopingIterator<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        Some(self.get())
    }
}
