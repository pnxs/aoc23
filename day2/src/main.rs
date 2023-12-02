use std::panic::panic_any;

struct Set {
    red: u32,
    green: u32,
    blue: u32
}

struct Game {
    id: u32,
    sets: Vec<Set>
}

fn parse_game(line: &str) -> Game
{
    let (game_id, notes) = line.split_once(':').unwrap();
    let game_id: u32 = game_id.strip_prefix("Game ").unwrap().parse().unwrap();
    let sets: Vec<Vec<&str>> = notes.split(";")
        .map(|n| n.split(',').collect() ).collect();

    let mut result = Game{
        id: game_id,
        sets: vec![]
    };

    for set_item in sets {
        let mut set = Set { red: 0, green: 0, blue: 0 };

        for item in set_item {
            let (nr, color) = item.trim().split_once(char::is_whitespace).unwrap();
            match color {
                "blue" => set.blue = nr.parse().unwrap(),
                "red" => set.red = nr.parse().unwrap(),
                "green" => set.green = nr.parse().unwrap(),
                _ => panic_any(format!("Invalid color: {color}, line: {line}"))
            }
        }
        result.sets.push(set);
    }

    return result
}

fn part1() -> u32 {
    let lines = include_str!("../input.txt").lines();

    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut gameid_sum = 0;

    'game: for l in lines {
        let game = parse_game(l);

        for s in game.sets {
            if s.red > max_red { continue 'game }
            if s.green > max_green { continue 'game }
            if s.blue > max_blue { continue 'game }
        }

        gameid_sum += game.id;
    }

    return gameid_sum
}

fn part2() -> u32 {
    let lines = include_str!("../input.txt").lines();

    let mut power_sum = 0;

    for l in lines {
        let game = parse_game(l);

        let max_red = game.sets.iter().map(|s| s.red).max().unwrap();
        let max_green = game.sets.iter().map(|s| s.green).max().unwrap();
        let max_blue = game.sets.iter().map(|s| s.blue).max().unwrap();

        power_sum += max_red * max_green * max_blue;
    }

    return power_sum
}

fn main() {
    let result_part1 = part1();
    println!("Result p1: {result_part1}");

    let result_part2 = part2();
    println!("Result p2: {result_part2}");
}
