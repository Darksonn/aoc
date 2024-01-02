struct Game {
    id: u32,
    rounds: Vec<Round>,
}

struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

fn main() {
    let input = std::fs::read("inputs/day02.txt").unwrap();
    let input = std::str::from_utf8(&input).unwrap();
    let mut games = Vec::new();
    for line in input.lines() {
        let (game_id, game) = line.split_once(':').unwrap();

        let game_id = game_id.strip_prefix("Game ").unwrap().parse().unwrap();
        let mut rounds = Vec::new();
        for round in game.split_terminator(';') {
            let mut r = Round {
                red: 0,
                blue: 0,
                green: 0,
            };
            for part in round.split_terminator(',').map(|part| part.trim()) {
                if let Some(num) = part.strip_suffix(" blue") { r.blue = num.parse().unwrap(); continue; }
                if let Some(num) = part.strip_suffix(" red") { r.red = num.parse().unwrap(); continue; }
                if let Some(num) = part.strip_suffix(" green") { r.green = num.parse().unwrap(); continue; }
                panic!();
            }
            rounds.push(r);
        }
        games.push(Game {
            id: game_id,
            rounds,
        });
    }

    let part1: u32 = games.iter()
        .filter(|game| game.rounds.iter().all(|round| round.red <= 12 && round.green <= 13 && round.blue <= 14))
        .map(|game| game.id)
        .sum();
    println!("Part 1: {part1}");

    let part2: u32 = games.iter()
        .map(|game| {
            let min_red = game.rounds.iter().map(|round| round.red).max().unwrap();
            let min_blue = game.rounds.iter().map(|round| round.blue).max().unwrap();
            let min_green = game.rounds.iter().map(|round| round.green).max().unwrap();

            min_red * min_blue * min_green
        })
        .sum();
    println!("Part 2: {part2}");
}
