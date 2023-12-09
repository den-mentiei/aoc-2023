use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let p1 = part1(&input);
	println!("p1 = {p1}");
	let p2 = part2(&input);
	println!("p2 = {p2}");

	Ok(())
}

fn part1(input: &str) -> i32 {
	input
		.lines()
		.map(|s| solve(s.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect()))
		.sum()
}

fn part2(input: &str) -> i32 {
	input
		.lines()
		.map(|s| {
			let mut seq: Vec<_> = s.split_whitespace().filter_map(|s| s.parse::<i32>().ok()).collect();
			seq.reverse();
			solve(seq)
		})
		.sum()
}

fn solve(mut seq: Vec<i32>) -> i32 {
	let mut n = seq.len();
	loop {
		let mut z = true;
		for i in 1..n {
			seq[i - 1] = seq[i] - seq[i - 1];
			z = z && seq[i - 1] == 0;
		}
		n -= 1;
		if z { break }
	}
	while n < seq.len() {
		seq[n] += seq[n - 1];
		n += 1;
	}
	seq[n - 1]
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 114);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT), 2);
	}
}
