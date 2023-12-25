use anyhow::anyhow;
use std::collections::HashSet;

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

fn process_line(line: &str) -> u32 {
    let (winning, rem) = parse_winning_numbers(line).unwrap();
    rem
        .split_whitespace()
        .filter(|n| n.parse::<u32>().map_or(false, |it| winning.contains(&it)))
        .count() as u32
}

fn part2(input: &str) -> u32 {
    let mut mults = vec![1; input.lines().count()];
    for (i, l) in input.lines().enumerate() {
        let k = process_line(l);
        if k > 0 {
            for j in i + 1 .. (i + 1 + (k as usize)) {
                mults[j] += mults[i];
            }

        }
    }
    mults.iter().sum()
}

fn main() {
    let input = include_str!("./input2.txt");
    println!("{}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("./sample2.txt");
        assert_eq!(30, part2(input));
    }
}
