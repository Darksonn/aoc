pub struct RangeMapBuilder {
    mappings: Vec<RangeMapPart>,
}

pub struct RangeMap {
    mappings: Vec<RangeMapPart>,
}

struct RangeMapPart {
    from: u64,
    to: u64,
    len: u64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Range {
    pub from: u64,
    pub len: u64,
}

impl RangeMapBuilder {
    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }

    pub fn add_range(&mut self, from: u64, to: u64, len: u64) {
        self.mappings.push(RangeMapPart { from, to, len });
    }

    pub fn is_empty(&self) -> bool {
        self.mappings.is_empty()
    }

    pub fn build(mut self) -> RangeMap {
        self.mappings.sort_unstable_by_key(|range| range.from);

        for i in 1..self.mappings.len() {
            let end = self.mappings[i-1].from + self.mappings[i-1].len;
            if self.mappings[i].from < end {
                panic!("Overlapping ranges.");
            }
        }

        RangeMap {
            mappings: self.mappings
        }
    }
}

impl RangeMap {
    pub fn get(&self, val: u64) -> Option<u64> {
        let idx = match self.mappings.binary_search_by_key(&val, |m| m.from) {
            Ok(idx) => idx,
            Err(0) => return None,
            Err(idx) => idx - 1,
        };

        let range = &self.mappings[idx];
        let off = val - range.from;

        if off < range.len {
            Some(range.to + off)
        } else {
            None
        }
    }

    pub fn map_range(&self, range: Range) -> Vec<(Range, Option<Range>)> {
        let search = self.mappings.binary_search_by_key(&range.from, |m| m.from);
        let mut range_idx = match search {
            Ok(idx) => idx,
            Err(0) => 0,
            Err(idx) => {
                let r = &self.mappings[idx-1];
                let off = range.from - r.from;

                if off < r.len {
                    idx-1
                } else {
                    idx
                }
            },
        };

        let mut remaining_range = range;
        let mut output = Vec::new();
        while remaining_range.len > 0 {
            if range_idx == self.mappings.len() {
                output.push((remaining_range, None));
                break;
            }

            let range = &self.mappings[range_idx];
            if remaining_range.from < range.from {
                let diff = range.from - remaining_range.from;
                output.push((
                    Range {
                        from: remaining_range.from,
                        len: diff,
                    },
                    None
                ));
                remaining_range.from += diff;
                remaining_range.len -= diff;
            } else {
                let end = u64::min(
                    range.from + range.len,
                    remaining_range.from + remaining_range.len,
                );
                let diff = end - remaining_range.from;
                let off = remaining_range.from - range.from;
                output.push((
                    Range {
                        from: remaining_range.from,
                        len: diff,
                    },
                    Some(Range {
                        from: range.to + off,
                        len: diff,
                    }),
                ));
                remaining_range.from += diff;
                remaining_range.len -= diff;
                range_idx += 1;
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_range1() {
        let mut map = RangeMapBuilder::new();
        map.add_range(0, 7, 2);
        map.add_range(5, 2, 2);
        let map = map.build();

        assert_eq!(map.map_range(Range { from: 0, len: 10 }),
            vec![
                (Range { from: 0, len: 2 }, Some(Range { from: 7, len: 2 })),
                (Range { from: 2, len: 3 }, None),
                (Range { from: 5, len: 2 }, Some(Range { from: 2, len: 2 })),
                (Range { from: 7, len: 3 }, None),
            ]
        );
        assert_eq!(map.map_range(Range { from: 0, len: 7 }),
            vec![
                (Range { from: 0, len: 2 }, Some(Range { from: 7, len: 2 })),
                (Range { from: 2, len: 3 }, None),
                (Range { from: 5, len: 2 }, Some(Range { from: 2, len: 2 })),
            ]
        );
        assert_eq!(map.map_range(Range { from: 0, len: 6 }),
            vec![
                (Range { from: 0, len: 2 }, Some(Range { from: 7, len: 2 })),
                (Range { from: 2, len: 3 }, None),
                (Range { from: 5, len: 1 }, Some(Range { from: 2, len: 1 })),
            ]
        );
        assert_eq!(map.map_range(Range { from: 1, len: 6 }),
            vec![
                (Range { from: 1, len: 1 }, Some(Range { from: 8, len: 1 })),
                (Range { from: 2, len: 3 }, None),
                (Range { from: 5, len: 2 }, Some(Range { from: 2, len: 2 })),
            ]
        );
    }
}
