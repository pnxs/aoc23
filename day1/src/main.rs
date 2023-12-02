
use std::fs::read_to_string;

fn get_first_last(l: &str) -> Option<u32> {
    let first = l.find(char::is_numeric);
    let last = l.rfind(char::is_numeric);

    if first.is_some() && last.is_some() {
        let first_digit = l.chars().nth(first?)?.to_digit(10)?;
        let last_digit = l.chars().nth(last?)?.to_digit(10)?;
        return Some(first_digit*10 + last_digit)
    }
    return None;
}

fn part1() -> Option<u32> {
    let binding = read_to_string("input.txt").unwrap();
    let values = binding.lines();

    let mut sum: u32 = 0;
    for l in values {
        sum += get_first_last(l)?;
    }

    return Some(sum)
}

fn replace_digits(line: String) -> Vec<u8> {
    let mut digits: Vec<u8> = vec![];

    let digit_map = [
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for (s,c) in line.chars().enumerate() {
        let ss = &line[s..];
        for (name, val) in digit_map {
            match ss.starts_with(name) {
                true => digits.push(val),
                _ => ()
            }
        }
    }

    digits
}


fn part2() -> Option<u32> {
    let binding = read_to_string("input.txt").unwrap();

    let values : Vec<Vec<u8>> = binding.lines()
        .into_iter()
        .map(|x| replace_digits(x.to_string()))
        .collect();

    let mut sum: u32 = 0;
    for v in values {
        let first_digit = v.first()?.clone() as u32;
        let last_digit = v.last()?.clone() as u32;
        let value: u32 = first_digit *10 + last_digit;
        sum += value;

    }

    return Some(sum)
}

fn main()  {
    let result = part2().unwrap();
    println!("Result: {result}");

}