fn part1(input: &'static str) -> u32 {
    input
        .lines()
        .map(|l| {
            let mut iter = l.chars().filter_map(|c| c.to_digit(10));
            let first_digit = iter.next().unwrap();
            let last_digit = iter.last().unwrap_or(first_digit);
            10 * first_digit + last_digit
        })
        .sum()
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    println!("{}", output);
}
