use advent2018::DataReader;

use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use fnv::FnvHashMap;
use text_io::*;

fn main() {
    let mut input = DataReader::open(8);

    let input = advent2018::time("Read input", || {
        let line = input.next().unwrap();
        let nums: Vec<u32> = line.split(' ').map(|n| n.parse().unwrap()).collect();
        let (node, remain) = Node::parse(&nums);
        assert_eq!(remain.len(), 0);
        node
    });

    let sum = advent2018::time("Sum metadata", || {
        input.metadata_sum()
    });
    let sum2 = advent2018::time("Sum metadata ref", || {
        input.reference_sum()
    });
    println!("Part one: {}", sum);
    println!("Part two: {}", sum2);
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<u32>,
}
impl Node {
    pub fn parse(slice: &[u32]) -> (Node, &[u32]) {
        let child_len = slice[0] as usize;
        let metadata_len = slice[1] as usize;
        let mut slice = &slice[2..];

        let mut children = Vec::with_capacity(child_len);
        for _ in 0..child_len {
            let res = Node::parse(slice);
            children.push(res.0);
            slice = res.1;
        }
        let metadata = Vec::from(&slice[0..metadata_len]);
        (Node {
            children,
            metadata,
        }, &slice[metadata_len..])
    }
    fn metadata_sum(&self) -> u32 {
        let selfdata: u32 = self.metadata.iter().cloned().sum();
        let childdata: u32 = self.children.iter().map(|child| child.metadata_sum()).sum();
        selfdata + childdata
    }
    fn reference_sum(&self) -> u32 {
        if self.children.len() == 0 {
            self.metadata.iter().cloned().sum()
        } else {
            let sums: Vec<u32> = self.children.iter()
                .map(|child| child.reference_sum()).collect();
            let mut sum = 0;
            for meta in &self.metadata {
                if *meta == 0 { continue; }
                if let Some(v) = sums.get((meta-1) as usize) {
                    sum += v;
                }
            }
            sum
        }
    }
}
