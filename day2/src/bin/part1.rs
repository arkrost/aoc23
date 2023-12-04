use anyhow::{anyhow, bail, Context, Result};
use std::str::FromStr;

fn parse_game_id(s: &str) -> Result<u32> {
    let (game_prefix, records) = s
        .split_once(':')
        .ok_or(anyhow!("expected colon separator {:?}", s))?;

    let game_id = game_prefix
        .strip_prefix("Game ")
        .ok_or(anyhow!("expected 'Game ' prefix, got {:?}", game_prefix))?
        .parse::<u32>()
        .context("cannot parse game_id as u32")?;

    let invalid_rec: Option<&str> = records
        .split(';')
        .into_iter()
        .find(|r| !r.parse::<Record>().is_ok_and(|rec| is_valid(&rec)));

    if let Some(rec_str) = invalid_rec {
        bail!("Invalid game record {:?}", rec_str)
    }

    Ok(game_id)
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Record {
    red: u32,
    green: u32,
    blue: u32,
}

fn is_valid(&Record { red, green, blue }: &Record) -> bool {
    red <= 12 && green <= 13 && blue <= 14
}

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for p in s.trim().split(", ") {
            let (count_str, color) = p
                .split_once(" ")
                .context("Expected whitespace separated pair, got '{p}'")?;

            let count = count_str
                .parse::<u32>()
                .with_context(|| format!("Cannot parse cubes count {count_str:?}"))?;

            match color {
                "red" => red += count,
                "green" => green += count,
                "blue" => blue += count,
                _ => bail!("Unknown cube color '{color}'"),
            }
        }

        Ok(Record { red, green, blue })
    }
}

fn part1(input: &str) -> u32 {
    input.lines().flat_map(|l| parse_game_id(l)).sum()
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
        assert_eq!(
            Record {
                red: 1,
                green: 2,
                blue: 3
            },
            "1 red, 2 green, 3 blue".parse().unwrap()
        )
    }

    #[test]
    fn test2() {
        assert_eq!(42, parse_game_id("Game 42: 2 green").unwrap());
    }
}
