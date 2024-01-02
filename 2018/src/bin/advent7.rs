use advent2018::DataReader;

use std::collections::BTreeSet;
use std::collections::BinaryHeap;
use fnv::FnvHashMap;
use text_io::*;

use std::cmp;
use std::iter::FromIterator;

fn main() {
    let input = DataReader::open(7);

    let deps = advent2018::time("Read input", || {
        let mut deps = Vec::new();
        for mut line in input.text_io() {
            let task: char;
            let dep: char;
            scan!(line =>
                  "Step {} must be finished before step {} can begin.", dep, task);
            deps.push((task as u8, dep as u8));
        }
        deps
    });

    let sort = advent2018::time("Topological sort", || {
        let sorter = TopologicalSorter::new(&deps);
        String::from_iter(sorter.map(|byte| byte as char))
    });
    let finish = advent2018::time("Topological timing", || {
        let timer = WorkerTimer::new(&deps, 5);
        timer.get_finish_instant().0
    });

    println!("Part one: {}", sort);
    println!("Part two: {}", finish);
}

struct WorkerTimer {
    sorter: TopologicalSorter,
    /// The times at which a worker finishes a task.
    worker_ready: BinaryHeap<Instant>,
    time: u32,
}
impl WorkerTimer {
    pub fn new(dep_list: &[(u8, u8)], workers: usize) -> WorkerTimer {
        let mut worker_ready = BinaryHeap::new();
        for _ in 0..workers {
            worker_ready.push(Instant(0, None));
        }
        let sorter = TopologicalSorter::new(dep_list);
        WorkerTimer {
            sorter,
            worker_ready,
            time: 0,
        }
    }
    fn next_worker_finish(&mut self) -> Option<Instant> {
        match self.sorter.start_item() {
            Some(next_task) => {
                let mut ready_at = self.worker_ready.pop().unwrap();

                if ready_at.0 < self.time {
                    ready_at.0 = self.time;
                }

                if let Some(prev_task) = ready_at.1 {
                    self.sorter.remove_item(prev_task);
                }
                let finish_at = ready_at.add(next_task);
                println!("Task {} finish at {}.", next_task as char, finish_at.0);
                self.worker_ready.push(finish_at);
                Some(finish_at)
            },
            None => {
                let mut workers_removed = 0;
                loop {
                    workers_removed += 1;
                    match self.worker_ready.pop() {
                        Some(Instant(continue_at, Some(next_task))) => {
                            self.sorter.remove_item(next_task);
                            for _ in 0..workers_removed {
                                self.worker_ready.push(Instant(continue_at, None));
                            }
                            self.time = continue_at;
                            return self.next_worker_finish();
                        },
                        Some(Instant(_, None)) => {},
                        None => return None,
                    }
                }
            },
        }
    }
    fn get_finish_instant(mut self) -> Instant {
        let mut prev = self.next_worker_finish().unwrap();
        while let Some(next) = self.next_worker_finish() {
            prev = next;
        }
        prev
    }
}

#[derive(Copy,Clone,Eq,PartialEq)]
struct Instant(u32, Option<u8>);
impl Instant {
    pub fn add(&self, task: u8) -> Instant {
        assert!(task <= 'Z' as u8);
        assert!(task >= 'A' as u8);
        let time = (task - ('A' as u8)) as u32 + 61;
        Instant(self.0 + time, Some(task))
    }
}
// We are storing Instants in a BinaryHeap, but we want a min-heap, while
// BinaryHeap is a max-heap. Therefore we invert the ordering, such that the
// smallest instant is considered the largest.
impl PartialOrd for Instant {
    fn partial_cmp(&self, other: &Instant) -> Option<cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl Ord for Instant {
    fn cmp(&self, other: &Instant) -> cmp::Ordering {
        other.0.cmp(&self.0)
    }
}

struct TopologicalSorter {
    deps_on_me: FnvHashMap<u8, Vec<u8>>,
    dependency_count: FnvHashMap<u8, usize>,
    available: BTreeSet<u8>,
}
impl TopologicalSorter {
    pub fn new(dep_list: &[(u8, u8)]) -> TopologicalSorter {
        let mut deps_on_me = FnvHashMap::default();
        let mut dependency_count = FnvHashMap::default();
        let mut available = BTreeSet::new();
        for &(task, depends_on) in dep_list {
            let dep_count_ptr = dependency_count.entry(task).or_insert(0usize);
            *dep_count_ptr += 1;

            deps_on_me.entry(depends_on).or_insert(Vec::new()).push(task);

            available.insert(depends_on);
        }
        for &(task, _depends_on) in dep_list {
            available.remove(&task);
        }
        TopologicalSorter {
            deps_on_me: deps_on_me,
            dependency_count,
            available,
        }
    }
    fn start_item(&mut self) -> Option<u8> {
        let min_avail = self.available.iter().cloned().next();
        if let Some(min) = min_avail {
            self.available.remove(&min);
        }
        min_avail
    }
    fn remove_item(&mut self, item: u8) {
        if let Some(list) = self.deps_on_me.get(&item) {
            for &dep in list {
                let count_ref = self.dependency_count.get_mut(&dep).unwrap();
                *count_ref -= 1;
                if *count_ref == 0 {
                    self.available.insert(dep);
                }
            }
        }
    }
}
impl Iterator for TopologicalSorter {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        let min_avail = self.available.iter().cloned().next();
        if let Some(min) = min_avail {
            self.available.remove(&min);
            self.remove_item(min);
        }
        min_avail
    }
}
