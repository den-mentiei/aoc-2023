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
	let (p1, p2) = input
		.lines()
		.map(|s| s.split_whitespace().filter_map(|s| s.parse().ok()).collect())
		.map(|mut s: Vec<i32>| {
			s.insert(0, 0);
			s.push(0);

			let mut n = s.len();
			while n > 2 {
				for i in 1..n { s[i - 1] = s[i] - s[i - 1]; }
				n -= 1;
			}

			(-s[1], -s[0])
		})
		.fold((0, 0), |(s1, s2), (p1, p2)| (s1 + p1, s2 + p2));
	(p1, p2.abs())
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

	#[test]
	fn test_part1() {
		assert_eq!(solve(INPUT).0, 114);
	}

	#[test]
	fn test_part2() {
		assert_eq!(solve(INPUT).1, 2);
	}
}
