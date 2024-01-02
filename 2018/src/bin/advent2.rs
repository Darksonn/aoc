use advent2018::DataReader;
use std::fmt;
use fnv::FnvHashSet;

fn main() {
    let input = DataReader::open(2);

    let ids = advent2018::time("Read input", || {
        let mut ids = Vec::new();
        for line in input {
            ids.push(BoxID::new(line));
        }
        ids
    });

    let checksum = advent2018::time("Checksum", || {
        ChecksumValue::checksum(ids.iter().map(|id| id.checksum()))
    });

    let partial = advent2018::time("Matching ids", || {
        find_matching_partial(&ids).expect("No BoxIDs differ by one")
    });
    println!("Part one: {}", checksum);
    println!("Part two: {}", partial);
}

fn find_matching_partial<'a>(ids: &'a [BoxID]) -> Option<PartialBoxID<'a>> {
    let mut seen = FnvHashSet::with_capacity_and_hasher(ids.len(), Default::default());
    let len = match ids.get(0) {
        Some(id) => id.len(),
        None => return None,
    };
    for i in 0..len {
        for boxid in ids {
            let partial = boxid.partial(i);
            if !seen.insert(partial) {
                return Some(partial);
            }
        }
        seen.clear();
    }
    None
}

pub struct BoxID {
    /// We use a byte array to avoid handling utf-8.
    id: Vec<u8>,
}
impl BoxID {
    pub fn new(id: String) -> Self {
        BoxID {
            id: id.into(),
        }
    }
    pub fn len(&self) -> usize {
        self.id.len()
    }
    /// Count how many times every character occurs.
    fn count_characters(&self) -> [usize; 256] {
        let mut count = [0; 256];
        for &byte in &self.id {
            count[byte as usize] += 1;
        }
        count
    }
    pub fn checksum(&self) -> ChecksumValue {
        let mut res = ChecksumValue::new();
        for &count in self.count_characters().iter() {
            if count == 2 {
                res.has_double = true;
            }
            if count == 3 {
                res.has_triple = true;
            }
        }
        res
    }
    pub fn partial<'a>(&'a self, i: usize) -> PartialBoxID<'a> {
        PartialBoxID::new(self, i)
    }
}

pub struct ChecksumValue {
    pub has_double: bool,
    pub has_triple: bool,
}
impl ChecksumValue {
    pub fn new() -> Self {
        ChecksumValue {
            has_double: false,
            has_triple: false,
        }
    }
    pub fn checksum<I>(iter: I) -> u64
    where
        I: Iterator<Item = ChecksumValue>,
    {
        let mut doubles = 0;
        let mut triples = 0;
        for check in iter {
            if check.has_double {
                doubles += 1;
            }
            if check.has_triple {
                triples += 1;
            }
        }
        doubles * triples
    }
}

/// A box id where one character is hidden.
///
/// If two PartialBoxIDs are equal, their BoxIDs either differ by one character,
/// or are equal.
#[derive(Hash,PartialEq,Eq,Clone,Copy)]
pub struct PartialBoxID<'a> {
    start: &'a [u8],
    end: &'a [u8],
}
impl<'a> PartialBoxID<'a> {
    pub fn new(id: &'a BoxID, i: usize) -> Self {
        let (first, last) = id.id.split_at(i);
        let last = &last[1..];
        PartialBoxID {
            start: first,
            end: last,
        }
    }
}
impl<'a> fmt::Display for PartialBoxID<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::str::from_utf8;
        from_utf8(self.start).unwrap().fmt(f)?;
        from_utf8(self.end).unwrap().fmt(f)
    }
}
