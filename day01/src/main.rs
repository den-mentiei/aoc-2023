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
	input.lines().map(digit_sum::<false>).sum()
}

fn part2(input: &str) -> i32 {
	input.lines().map(digit_sum::<true>).sum()
}

fn digit_sum<const P2: bool>(s: &str) -> i32 {
	let s = s.as_bytes();
	let mut digits = (0..s.len())
		.filter_map(|i| match s[i] {
			b'1'..=b'9' => Some((s[i] - b'0') as i32),
			_ if P2 => eat_wordy_digit(&s[i..]),
			_ => None,
		});
	let d1 = digits.next().expect("Bad inputs");
	let d0 = digits.last().unwrap_or(d1);
	d1 * 10 + d0
}

fn eat_wordy_digit(s: &[u8]) -> Option<i32> {
	match s {
		[b'o', b'n', b'e', ..]             => Some(1),
		[b't', b'w', b'o', ..]             => Some(2),
		[b't', b'h', b'r', b'e', b'e', ..] => Some(3),
		[b'f', b'o', b'u', b'r', ..]       => Some(4),
		[b'f', b'i', b'v', b'e', ..]       => Some(5),
		[b's', b'i', b'x', ..]             => Some(6),
		[b's', b'e', b'v', b'e', b'n', ..] => Some(7),
		[b'e', b'i', b'g', b'h', b't', ..] => Some(8),
		[b'n', b'i', b'n', b'e', ..]       => Some(9),
		_ => None,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT1), 142);
	}

	const INPUT2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT2), 281);
	}
}
