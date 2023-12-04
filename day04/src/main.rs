use std::collections::HashSet;
use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let (p1, p2) = solve(&input);
	println!("p1 = {p1}");
	println!("p2 = {p2}");

	Ok(())
}

fn solve(input: &str) -> (i32, i32) {
	let matches = input
		.lines()
		.filter_map(|s| {
			let (_, s)       = s.split_once(':')?;
			let (wins, mine) = s.split_once('|')?;

			fn parse_numbers(s: &str) -> HashSet<i32> {
				s
					.trim()
					.split(' ')
					.filter_map(|w| w.parse().ok())
					.collect()
			}

			let wins = parse_numbers(wins);
			let mine = parse_numbers(mine);
			Some(wins.intersection(&mine).count())
		})
		.collect::<Vec<_>>();

	let mut points = 0;
	let mut copies = vec![1; matches.len()];

	for (c, &m) in matches.iter().enumerate() {
		if m == 0 {
			continue;
		}

		points += 1 << (m - 1);

		let have = copies[c];
		for i in 0..m {
			let won = c + i + 1;
			copies[won] += have;
		}
	}

	let copies = copies.into_iter().sum();

	(points, copies)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

	#[test]
	fn test_part1() {
		assert_eq!(solve(INPUT).0, 13);
	}

	#[test]
	fn test_part2() {
		assert_eq!(solve(INPUT).1, 30);
	}
}
