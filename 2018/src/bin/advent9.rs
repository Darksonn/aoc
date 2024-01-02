use advent2018::DataReader;

use text_io::*;
use std::cmp;

fn main() {
    let input = DataReader::open(9);

    let (players, points) = advent2018::time("Read input", || {
        let mut line = input.text_io().next().unwrap();
        let pl: usize;
        let po: usize;
        scan!(line => "{} players; last marble is worth {} points", pl, po);
        (pl, po)
    });

    let high_score = advent2018::time("Play game", || {
        solve_part_one(players, points)
    });
    // good thing the solution runs in linear time
    let high_score_large = advent2018::time("Play long game", || {
        solve_part_one(players, 100*points)
    });
    println!("Part one: {}", high_score);
    println!("Part one: {}", high_score_large);

}

pub fn solve_part_one(players: usize, marbles: usize) -> usize {
    let mut scores = vec![0; players];
    let mut player = 0;
    let mut game = MarbleGame::new();
    for _ in 0..=marbles {
        let points = game.insert_next_marble();
        scores[player] += points;
        player += 1;
        if player == players {
            player = 0;
        }
    }
    scores.into_iter().max().unwrap()
}

struct MarbleGame {
    /// This is a linked list stored in a Vec!
    /// not all spots are guaranteed to be in use
    marbles: Vec<CircleSpot>,
    current_marble: usize,
    free_spots: Vec<usize>,
    next_marble: usize,
}
impl MarbleGame {
    pub fn new() -> MarbleGame {
        let first_marble = MarbleId(0);
        let spot = CircleSpot {
            left: 0,
            right: 0,
            marble: first_marble,
        };
        MarbleGame {
            marbles: vec![spot],
            current_marble: 0,
            free_spots: Vec::new(),
            next_marble: 1,
        }
    }
    fn empty_index(&mut self) -> usize {
        match self.free_spots.pop() {
            Some(spot) => spot,
            None => {
                let i = self.marbles.len();
                self.marbles.push(CircleSpot {
                    left: i,
                    right: i,
                    marble: MarbleId(std::usize::MAX),
                });
                i
            },
        }
    }
    fn go_left(&self, i: usize) -> usize {
        self.marbles[i].left
    }
    fn go_right(&self, i: usize) -> usize {
        self.marbles[i].right
    }
    /// returns number of points
    fn insert_next_marble(&mut self) -> usize {
        let marble = MarbleId(self.next_marble);
        self.next_marble += 1;
        if marble.is_special() {
            let mut i = self.current_marble;
            for _ in 0..6 {
                i = self.go_left(i);
            }
            let removeright = i;
            let remove = self.go_left(removeright);
            let removeleft = self.go_left(remove);
            self.marbles[removeleft].right = removeright;
            self.marbles[removeright].left = removeleft;
            self.current_marble = removeright;
            marble.0 + self.marbles[remove].marble.0
        } else {
            let spot = self.empty_index();
            let spotleft = self.go_right(self.current_marble);
            let spotright = self.go_right(spotleft);
            self.marbles[spot].left = spotleft;
            self.marbles[spot].right = spotright;
            self.marbles[spotleft].right = spot;
            self.marbles[spotright].left = spot;
            self.marbles[spot].marble = marble;
            self.current_marble = spot;
            0
        }
    }
    #[allow(dead_code)]
    pub fn debug(&self) {
        let mut i = self.current_marble;
        print!("{} ", self.marbles[i].marble.0);
        i = self.go_right(i);
        while i != self.current_marble {
            print!("{} ", self.marbles[i].marble.0);
            i = self.go_right(i);
        }
        println!();
    }
}
// left is counter-clockwise
// right is clockwise
struct CircleSpot {
    left: usize,
    right: usize,
    marble: MarbleId,
}


#[derive(Copy,Clone,Eq,PartialEq)]
struct MarbleId(usize);
impl MarbleId {
    fn is_special(self) -> bool {
        self.0 % 23 == 0
    }
}
// reversed ordering
impl PartialOrd for MarbleId {
    fn partial_cmp(&self, other: &MarbleId) -> Option<cmp::Ordering> {
        other.0.partial_cmp(&self.0)
    }
}
impl Ord for MarbleId {
    fn cmp(&self, other: &MarbleId) -> cmp::Ordering {
        other.0.cmp(&self.0)
    }
}
