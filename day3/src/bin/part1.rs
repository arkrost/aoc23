#[derive(Debug)]
struct Number {
    value: u32,
    x1: usize,
    x2: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Debug)]
struct Symbol {
    x: usize,
    y: usize,
}

fn collect_parts(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = Vec::<Number>::new();
    let mut symbols = Vec::<Symbol>::new();

    for (y, l) in input.lines().enumerate() {
        let mut acc: Option<Number> = None;

        for (x, c) in l.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                acc = Some(match &acc {
                    None => Number {
                        value: d,
                        x1: x,
                        x2: x,
                        y,
                    },
                    Some(ref n) => Number {
                        value: n.value * 10 + d,
                        x2: x,
                        x1: n.x1,
                        y,
                    },
                })
            } else {
                if let Some(n) = acc {
                    numbers.push(n);
                    acc = None;
                }
                if c != '.' {
                    symbols.push(Symbol { x, y });
                }
            }
        }

        if let Some(n) = acc {
            numbers.push(n);
        }
    }

    (numbers, symbols)
}

fn neighbours(n: &Number, s: &Symbol) -> bool {
    n.y.abs_diff(s.y) < 2 && (n.x1 <= s.x + 1 && s.x <= n.x2 + 1)
}

fn part1(input: &str) -> u32 {
    let (numbers, symbols) = collect_parts(input);
    numbers
        .iter()
        .filter(|n| symbols.iter().any(|s| neighbours(n, s)))
        .map(|n| n.value)
        .sum()
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
        assert_eq!(4361, part1(input));
    }
}
