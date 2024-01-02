use advent2018::DataReader;
use std::collections::VecDeque;

use fnv::FnvHashMap;
use text_io::*;

fn main() {
    let input = DataReader::open(6);

    let points = advent2018::time("Read input", || {
        let mut points = Vec::new();
        for mut line in input.text_io() {
            let (x, y);
            scan!(line => "{}, {}", x, y);
            points.push(OrigPoint(x, y, points.len()));
        }
        points
    });

    let max_size = advent2018::time("Dijkstra", || {
        let sizes = find_sizes(&points);
        let mut max_size = 0;
        for size in &sizes {
            if let Some(size) = *size {
                if max_size < size {
                    max_size = size;
                }
            }
        }
        max_size
    });
    let shared = advent2018::time("Region and", || {
        count_shared_region(&points, 10000)
    });

    println!("Part one: {}", max_size);
    println!("Part two: {}", shared);
}

fn count_shared_region(points: &[OrigPoint], max_dist: i32) -> usize {
    let max_dist = max_dist-1;
    if points.len() == 0 {
        return 0;
    }
    let mut region = Vec::new();
    let point1 = points[0];
    let points = &points[1..];
    for x in point1.0 - max_dist ..= point1.0 + max_dist {
        let y_max = max_dist - (point1.0 - x).abs();
        for y in point1.1 - y_max ..= point1.1 + y_max {
            let p = Point(x, y);
            region.push((p, p.dist(point1)));
        }
    }

    for point in points {
        for i in (0..region.len()).rev() {
            let (p, sum_dist) = region[i];
            let dist = p.dist(point);
            let sum_dist = sum_dist + dist;
            if max_dist < sum_dist {
                region.swap_remove(i);
            } else {
                region[i] = (p, sum_dist);
            }
        }
    }

    region.len()
}

fn find_sizes(points: &[OrigPoint]) -> Vec<Option<usize>> {
    let bounding_box = max_xy(points);
    enum Visited {
        SeenOnce(OrigPoint, usize),
        SeenMultiple,
    };
    let mut is_inf = vec![false; points.len()];
    let mut visited: FnvHashMap<Point, Visited> = FnvHashMap::default();
    let mut stack = VecDeque::new();
    for &point in points {
        stack.push_back((0, point, point.into()));
    }
    // dijkstra
    while let Some((dist, orig, point)) = stack.pop_front() {
        match visited.get(&point) {
            Some(Visited::SeenOnce(orig2, dist2)) => {
                if *dist2 == dist {
                    if *orig2 != orig {
                        visited.insert(point, Visited::SeenMultiple);
                    }
                }
            },
            Some(Visited::SeenMultiple) => {},
            None => {
                visited.insert(point, Visited::SeenOnce(orig, dist));
                if point.is_inf(bounding_box) {
                    is_inf[orig.2] = true;
                } else {
                    stack.push_back((dist+1, orig, point.left()));
                    stack.push_back((dist+1, orig, point.right()));
                    stack.push_back((dist+1, orig, point.up()));
                    stack.push_back((dist+1, orig, point.down()));
                }
            },
        }
    }
    let mut counts = Vec::new();
    for point in points {
        if is_inf[point.2] {
            counts.push(None);
        } else {
            counts.push(Some(0));
        }
    }
    // count how many are in each area
    for (_point, visited) in visited {
        match visited {
            Visited::SeenOnce(orig, _dist) => {
                if let Some(count) = counts[orig.2] {
                    counts[orig.2] = Some(count + 1);
                }
            },
            Visited::SeenMultiple => {},
        }
    }
    counts
}





fn max_xy(points: &[OrigPoint]) -> (i32, i32) {
    let mut maxx = 0;
    let mut maxy = 0;
    for point in points {
        if maxx < point.0 {
            maxx = point.0;
        }
        if maxy < point.1 {
            maxy = point.1;
        }
    }
    (maxx, maxy)
}

#[derive(Copy,Clone,PartialEq,Eq,Debug,Hash,PartialOrd,Ord)]
struct Point(i32, i32);
#[derive(Copy,Clone,PartialEq,Eq,Debug,Hash,PartialOrd,Ord)]
struct OrigPoint(i32, i32, usize);

impl Point {
    pub fn dist(self, other: impl Into<Point>) -> i32 {
        let other = other.into();
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
    pub fn is_inf(self, bounding_box: (i32, i32)) -> bool {
        self.0 < -bounding_box.0 || self.1 < -bounding_box.1
            || self.0 > 2*bounding_box.0 || self.1 > 2*bounding_box.1
    }
    pub fn left(self) -> Point {
        Point(self.0 - 1, self.1)
    }
    pub fn right(self) -> Point {
        Point(self.0 + 1, self.1)
    }
    pub fn up(self) -> Point {
        Point(self.0, self.1 - 1)
    }
    pub fn down(self) -> Point {
        Point(self.0, self.1 + 1)
    }
}
impl From<OrigPoint> for Point {
    fn from(o: OrigPoint) -> Point {
        Point(o.0, o.1)
    }
}
impl From<&OrigPoint> for Point {
    fn from(o: &OrigPoint) -> Point {
        Point(o.0, o.1)
    }
}
