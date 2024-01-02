use aoc2023::parse_iter;

struct Card {
    num: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn new(s: &str) -> Card {
        let (card, numbers) = s.split_once(':').unwrap();
        let card = card.strip_prefix("Card").unwrap().trim().parse().unwrap();
        let (winning, numbers) = numbers.split_once('|').unwrap();

        Card {
            num: card,
            winning: parse_iter(winning.split_ascii_whitespace()),
            numbers: parse_iter(numbers.split_ascii_whitespace()),
        }
    }

    fn num_winning(&self) -> usize {
        self.numbers.iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }

    fn value(&self) -> u64 {
        let n = self.num_winning() as u64;
        (1 << n) / 2
    }
}

fn main() {
    let input = aoc2023::load_input(4);

    let cards: Vec<Card> = input.lines().map(|line| Card::new(line)).collect();

    let problem1: u64 = cards.iter().map(|card| card.value()).sum();
    println!("Part 1: {problem1}");

    let mut number_of_cards = vec![1u64; cards.len()];

    for i in 0..cards.len() {
        for j in (i+1) .. (i+1 + cards[i].num_winning()) {
            number_of_cards[j] += number_of_cards[i];
        }
    }

    let problem2: u64 = number_of_cards.into_iter().sum();
    println!("Part 2: {problem2}");
}
