use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let p1 = solve(&input, 2);
	println!("p1 = {p1}");
	let p2 = solve(&input, 1_000_000);
	println!("p2 = {p2}");

	Ok(())
}

fn solve(input: &str, expand: usize) -> i64 {
	let m = input.as_bytes();
	let w = m.iter().position(|&b| b == b'\n').unwrap();
	let h = m.len() / w;

	let mut empty = vec![expand - 1; h + w];
	let mut gs    = Vec::new();

	let mut r = 0;
	let mut c = 0;
	for &b in m {
		if b == b'\n' {
			r  = 0;
			c += 1;
			continue;
		}
		if b == b'#' {
			gs.push((r, c));
			empty[c]     = 0;
			empty[w + r] = 0;
		}
		r += 1;
	}

	fn prefix_sum(xs: &mut [usize]) {
		for i in 1..xs.len() {
			xs[i] += xs[i - 1];
		}
	}
	prefix_sum(&mut empty[..w]);
	prefix_sum(&mut empty[w..]);

	for g in gs.iter_mut() {
		g.0 += empty[w + g.0];
		g.1 += empty[g.1];
	}

	let mut sum = 0;
	let n = gs.len();
	for i in 0..n - 1 {
		let (x0, y0) = (gs[i].0 as i64, gs[i].1 as i64);
		for j in i + 1..n {
			let (x1, y1) = (gs[j].0 as i64, gs[j].1 as i64);
			let d  = (x0 - x1).abs() + (y0 - y1).abs(); // manhattan distance
			sum += d;
		}
	}

	sum
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;

	#[test]
	fn test_part1() {
		assert_eq!(solve(INPUT, 2), 374);
	}

	#[test]
	fn test_part2() {
		assert_eq!(solve(INPUT, 10),  1030);
		assert_eq!(solve(INPUT, 100), 8410);
	}
}
