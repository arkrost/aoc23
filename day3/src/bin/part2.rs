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
                if c == '*' {
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

fn part2(input: &str) -> u32 {
    let (numbers, symbols) = collect_parts(input);
    symbols
        .into_iter()
        .map(|s| {
            let three_neighbours: Vec<u32> = numbers
                .iter()
                .filter(|n| neighbours(n, &s))
                .map(|n| n.value)
                .take(3)
                .collect();
            match three_neighbours.as_slice() {
                [a, b] => a * b,
                _ => 0,
            }
        })
        .sum()
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
        assert_eq!(467835, part2(input));
    }
}
