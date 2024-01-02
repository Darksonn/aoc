use advent2018::{DataReader, TextIOLine};
use text_io::*;

use prefix_sum::sum2d::{PrefixSum2D, Rect, Buf2D};

fn main() {
    let input = DataReader::open(3);

    let claims = advent2018::time("Read input", move || {
        let mut claims = Vec::new();
        for line in input.text_io() {
            claims.push(Claim::new(line));
        }
        claims
    });

    let (overlaps, counts) = advent2018::time("Count overlaps", || {
        count_overlap_at_least_two(&claims)
    });

    let lonely = advent2018::time("Find lonely", || {
        find_lonely_claim(&counts, &claims)
    });
    println!("Part one: {}", overlaps);
    println!("Part two: {}", lonely);
}

fn count_overlap_at_least_two(claims: &[Claim]) -> (usize, Buf2D<u32>) {
    let (w, h) = max_size(claims);

    // I am the author of the prefix_sum crate.
    let mut sum: PrefixSum2D<u32> = PrefixSum2D::new(w, h);
    for claim in claims {
        sum.add_rectangle(claim.rect, 1);
    }
    let mut count = 0;
    let counts = sum.build();
    for row in &counts {
        for value in row {
            if *value > 1 {
                count += 1;
            }
        }
    }
    (count, counts)
}

fn find_lonely_claim(counts: &Buf2D<u32>, claims: &[Claim]) -> usize {
    // This is not super efficient, but oh well.
    'next_claim: for claim in claims {
        for x in claim.rect.x ..= claim.rect.x2() {
            for y in claim.rect.y ..= claim.rect.y2() {
                if counts[(x,y)] > 1 {
                    continue 'next_claim;
                }
            }
        }
        return claim.claim_id;
    }
    panic!("Lonely claim not found.");
}

fn max_size(claims: &[Claim]) -> (usize, usize) {
    let mut width = 0;
    let mut height = 0;
    for claim in claims {
        if width < claim.rect.x2() {
            width = claim.rect.x2();
        }
        if height < claim.rect.y2() {
            height = claim.rect.y2();
        }
    }
    (width+1, height+1)
}

pub struct Claim {
    pub claim_id: usize,
    pub rect: Rect,
}
impl Claim {
    pub fn new(mut s: TextIOLine) -> Self {
        let (claim_id, x, y, width, height);
        scan!(s => "#{} @ {},{}: {}x{}", claim_id, x, y, width, height);
        Claim {
            claim_id,
            rect: Rect::new(x, y, width, height),
        }
    }
}

