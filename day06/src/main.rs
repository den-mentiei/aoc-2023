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

fn part1(input: &str) -> i64 {
	let (time, dist) = input.split_once('\n').expect("Bad input");

	let time = time.split_whitespace().filter_map(|s| s.parse::<i64>().ok());
	let dist = dist.split_whitespace().filter_map(|s| s.parse::<i64>().ok());

	time.zip(dist).map(|(t, d)| solve(t as f64, d as f64)).product()
}

fn part2(input: &str) -> i64 {
	let (time, dist) = input.split_once('\n').expect("Bad input");

	fn read_number(s: &str) -> i64 {
		s
			.bytes()
			.filter(|b| b.is_ascii_digit())
			.fold(0, |a, x| a * 10 + (x - b'0') as i64)
	}

	let t = read_number(time);
	let d = read_number(dist);
	solve(t as f64, d as f64)
}

fn solve(t: f64, d: f64) -> i64 {
	let x = ((t - (t * t - 4.0 * (d + 1.0)).sqrt()) * 0.5).ceil() as i64;
	(t as i64) - 2 * x + 1
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 288);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT), 71503);
	}
}
