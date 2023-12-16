#![feature(slice_swap_unchecked)]

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
	let (mut map, dim) = parse(input);
	tilt(&mut map, dim);
	weight(&map, dim)
}

fn part2(input: &str) -> i32 {
	let map = parse(input);

	let (mut map, i, n) = brent(map, |(mut m, dim)| {
		spin(&mut m, dim);
		(m, dim)
	});

	let left = (1_000_000_000 - i) % n;
	for _ in 0..left {
		spin(&mut map.0, map.1);
	}
	weight(&map.0, map.1)
}

fn tilt(map: &mut [u8], dim: usize) {
	for c in map.chunks_exact_mut(dim) {
		let mut pre = 0;
		for i in 0..dim {
			match c[i] {
				0 => pre += 1,
				1 => pre  = 0,
				2 => unsafe { c.swap_unchecked(i, i - pre) },
				_ => (),
			}
		}
	}
}

fn weight(map: &[u8], dim: usize) -> i32 {
	map
		.chunks_exact(dim)
		.flat_map(|c| c
			 .iter()
			 .rev()
			 .enumerate()
			 .filter(|(_, &c)| c == 2)
			 .map(|(i, _)| (i + 1) as i32))
		.sum::<i32>()
}

fn spin(map: &mut [u8], dim: usize) {
	for _ in 0..4 {
		tilt(map, dim);
		rot_cw(map, dim);
	}
}

fn rot_cw(m: &mut [u8], dim: usize) {
	for r in 0..dim / 2 {
		for c in 0..(dim + 1) / 2 {
			let i1 = r * dim + c;
			let i2 = c * dim + dim - 1 - r;
			let i3 = (dim - 1 - r) * dim + dim - 1 - c;
			let i4 = (dim - 1 - c) * dim + r;
			unsafe {
				m.swap_unchecked(i1, i2);
				m.swap_unchecked(i1, i4);
				m.swap_unchecked(i3, i4);
			};
		}
	}
}

fn brent<T, F>(x0: T, f: F) -> (T, usize, usize)
where
	T: Clone + PartialEq,
	F: Fn(T) -> T,
{	// Main phase: search successive powers of two.
	let mut power    = 1;
	let mut lam      = 1;
	let mut tortoise = x0.clone();
	let mut hare     = f(x0.clone());
	while tortoise != hare {
		if power == lam { // Time to start a new power of two?
			tortoise = hare.clone();
			power  <<= 1;
			lam      = 0;
		}
		hare = f(hare);
		lam += 1;
	}
	// Find the position of the first repitition of 'lam' length.
	tortoise = x0.clone();
	hare     = (0..lam).fold(x0, |x, _| f(x));
	// The distance between the hare and tortoise is now 'lam'.
	let mut mu = 0;
	while tortoise != hare {
		tortoise = f(tortoise);
		hare     = f(hare);
		mu      += 1;
	}
	(hare, mu, lam)
}

fn parse(input: &str) -> (Vec<u8>, usize) {
	let w = input.bytes().position(|b| b == b'\n').unwrap();
	let h = input.trim_end().len() / w;
	debug_assert_eq!(w, h);

	let mut map = vec![0u8; w * w];

	for (r, s) in input.lines().map(|s| s.as_bytes()).enumerate() {
		for c in 0..s.len() {
			map[((h - c - 1) * w) + r] = match s[c] {
				b'#' => 1,
				b'O' => 2,
				_    => 0,
			};
		}
	}

	(map, w)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 136);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT), 64);
	}
}
