use anyhow::anyhow;
use std::{collections::HashSet, ops::Shl};

fn parse_winning_numbers(input: &str) -> anyhow::Result<(HashSet<u32>, &str)> {
    let (_, rest) = input
        .split_once(": ")
        .ok_or(anyhow!("expected 'Card d+: ' prefix"))?;
    let (winning_numbers, rest) = rest
        .split_once(" | ")
        .ok_or(anyhow!("expected ' | ' separator"))?;
    let parsed_winning_numbers = winning_numbers
        .split_whitespace()
        .map(|n| {
            n.parse::<u32>()
                .map_err(|e| anyhow!("failed to parse winning number: {}", e))
        })
        .collect::<anyhow::Result<HashSet<u32>>>()?;
    Ok((parsed_winning_numbers, rest))
}

fn process_line(line: &str) -> anyhow::Result<u32> {
    let (winning, rem) = parse_winning_numbers(line)?;
    let k = rem
        .split_whitespace()
        .filter(|n| n.parse::<u32>().map_or(false, |it| winning.contains(&it)))
        .count() as u32;
    let res = if k > 0 { 1u32.shl(k - 1) } else { 0 };
    Ok(res)
}

fn part1(input: &str) -> u32 {
    input.lines().flat_map(process_line).sum()
}

fn main() {
    let input = include_str!("./input1.txt");
    println!("{}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("./sample1.txt");
        assert_eq!(13, part1(input));
    }
}
