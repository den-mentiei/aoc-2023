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

fn solve(input: &str) -> (usize, usize) {
	input
		.split("\n\n")
		.fold((0, 0), |(p1, p2), s| {
			let (cols, rows) = parse(s);
			let (c1, c2)     = count(&cols, 1);
			let (r1, r2)     = count(&rows, 100);
			(p1 + c1 + r1, p2 + c2 + r2)
		})
}

fn count(xs: &[u32], k: usize) -> (usize, usize) {
	let mut p1 = 0;
	let mut p2 = 0;
	let n = xs.len();
	for i in 0..n - 1 {
		let mut diffs = 0;
		let m = i.min(n - i - 2);
		for j in 0..=m {
			diffs += (xs[i - j] ^ xs[i + 1 + j]).count_ones();
		}
		if diffs == 0 { p1 += (i + 1) * k; }
		if diffs == 1 { p2 += (i + 1) * k; }
	}
	(p1, p2)
}

fn parse(pattern: &str) -> (Vec<u32>, Vec<u32>) {
	let w = pattern.bytes().position(|b| b == b'\n').unwrap();

	let mut cols = vec![0u32; w];
	let mut rows = Vec::new();

	for s in pattern.lines().map(|s| s.as_bytes()) {
		let mut row = 0;
		let r = rows.len();
		for i in 0..s.len() {
			if s[i] == b'#' {
				row |= 1 << i;
				cols[i] |= 1 << r;
			}
		}
		rows.push(row);
	}

	(cols, rows)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;

	#[test]
	fn test_part1() {
		assert_eq!(solve(INPUT).0, 405);
	}

	// #[test]
	// fn test_part2() {
	// 	assert_eq!(solve(INPUT).1, 400);
	// }
}
