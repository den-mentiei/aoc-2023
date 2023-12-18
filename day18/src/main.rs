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
	solve(input, parse1)
}

fn part2(input: &str) -> i64 {
	solve(input, parse2)
}

fn solve(input: &str, parse: fn(&str) -> (u8, i64)) -> i64 {
	let (mut r, mut c) = (0, 0);
	let mut area = 0;

	for l in input.lines() {
		let (d, n) = parse(l);
		let (pr, pc) = (r, c);
		match d {
			0 => r -= n,
			1 => c += n,
			2 => r += n,
			3 => c -= n,
			_    => unreachable!(),
		};
		area += (c + pc) * (r - pr) + n;
	}

	area / 2 + 1
}

fn parse1(s: &str) -> (u8, i64) {
	let b = s.as_bytes();
	let d = match b[0] {
		b'R' => 0,
		b'D' => 1,
		b'L' => 2,
		b'U' => 3,
		_    => unreachable!(),
	};
	let n = match b[2] {
		b'1' if b[3].is_ascii_digit() => (b[2] - b'0') * 10 + b[3] - b'0',
		b'1'..=b'9'                   => b[2] - b'0',
		_ => unreachable!(),
	} as i64;

	(d, n)
}

fn parse2(s: &str) -> (u8, i64) {
	let (_, b) = s.split_once('#').unwrap();
	let h = u32::from_str_radix(&b[..6], 16).unwrap();
	((h & 3) as u8, (h >> 4) as i64)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 62);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT), 952408144115);
	}
}
