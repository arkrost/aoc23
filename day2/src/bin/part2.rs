use anyhow::{anyhow, bail, Context, Result};
use std::str::FromStr;

fn to_game_power(s: &str) -> Result<u32> {
    let (_, records) = s
        .split_once(':')
        .ok_or(anyhow!("expected colon separator {:?}", s))?;

    let mut red = 0u32;
    let mut green = 0u32;
    let mut blue = 0u32;

    for rec_str in records.split(';') {
        let rec = rec_str.parse::<Record>()?;
        red = red.max(rec.red);
        green = green.max(rec.green);
        blue = blue.max(rec.blue);
    }

    Ok(red * green * blue)
}

#[derive(Debug, Default, Eq, PartialEq)]
struct Record {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Record {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for p in s.trim().split(", ") {
            let (count_str, color) = p
                .split_once(' ')
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

fn part2(input: &str) -> u32 {
    input.lines().flat_map(to_game_power).sum()
}

fn main() {
    let input = include_str!("./input2.txt");
    println!("{}", part2(input));
}
