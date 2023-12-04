const DIGITS: &[&str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn to_value(line: &str) -> u32 {
    let mut idx = 0;
    let mut iter = std::iter::from_fn(|| loop {
        let tail = &line[idx..];
        idx += 1;

        let first_char = tail.chars().next()?;

        let parse_digit = first_char.to_digit(10).or_else(|| {
            DIGITS.iter().enumerate().find_map(|(v, p)| {
                if tail.starts_with(p) {
                    Some(v as u32)
                } else {
                    None
                }
            })
        });
        if parse_digit.is_some() {
            return parse_digit;
        }
    });
    let first_digit = iter.next().unwrap();
    let last_digit = iter.last().unwrap_or(first_digit);
    10 * first_digit + last_digit
}

fn part2(input: &str) -> u32 {
    input.lines().map(|l| to_value(l)).sum()
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
        assert_eq!(80, to_value("jcb82eightwzerond"));
    }
}
