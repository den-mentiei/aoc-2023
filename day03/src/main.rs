#![feature(byte_slice_trim_ascii)]
#![feature(vec_push_within_capacity)]

use std::collections::HashMap;
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
	const DIR: [(i8, i8); 8] = [
		(-1, -1), // nw
		(-1,  0), // n
		(-1,  1), // ne
		( 0, -1), // w
		( 0,  1), // e
		( 1, -1), // sw
		( 1,  0), // s
		( 1,  1), // se
	];

	let w = input.find('\n').unwrap_or(input.len()) as i32;
	let h = input.trim_end().bytes().filter(|&x| x == b'\n').count() as i32 + 1;

	let mut m = Vec::with_capacity((w * h) as usize);
	for x in input.bytes().filter(|&x| x != b'\n') {
		_ = m.push_within_capacity(x);
	}

	let mut sum   = 0;
	let mut gears = HashMap::new();

	for r in 0..h {
		let mut c = 0;
		while c < h {
			let mut num  = 0;
			let mut good = false;
			let mut star = None;

			loop {
				let x = m[(r * w) as usize + c as usize];
				if !x.is_ascii_digit() {
					c += 1;
					break;
				}

				num = num * 10 + (x - b'0') as i32;

				fn is_symbol(b: u8) -> bool {
					b != b'.' && !b.is_ascii_digit()
				}

				for (dr, dc) in DIR {
					let r = r + dr as i32;
					let c = c + dc as i32;
					if r < 0 || r == h || c < 0 || c > w {
						continue;
					}
					let x = m[(r * w) as usize + c as usize];
					if !good {
						good = is_symbol(x);
						if x == b'*' {
							star = Some((r, c));
						}
					}
				}

				c += 1;
				if c == w {
					break;
				}
			}

			if let Some((r, c)) = star {
				let (k, m) = gears.entry((r, c)).or_insert((0, 1));
				*k += 1;
				if *k <= 2 {
					*m *= num;
				}
			}

			if good {
				sum += num;
			}
		}
	}

	let gears = gears
		.into_values()
		.filter(|&(k, _)| k == 2)
		.map(|(_, x)| x)
		.sum();

	(sum, gears)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

	#[test]
	fn test_part1() {
		assert_eq!(solve(INPUT).0, 4361);
	}

	#[test]
	fn test_part2() {
		assert_eq!(solve(INPUT).1, 467835);
	}
}
