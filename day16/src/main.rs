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

fn part1(input: &str) -> usize {
	let w = input.bytes().position(|b| b == b'\n').unwrap();
	traverse(input.as_bytes(), w as i16, (0, 0), 1)
}

fn part2(input: &str) -> usize {
	let w        = input.bytes().position(|b| b == b'\n').unwrap() as i16;
	let map      = input.as_bytes();
	let mut best = 0;
	for x in 0..=w {
		best = best.max(traverse(map, w, (0,     x), 3));
		best = best.max(traverse(map, w, (w - 1, x), 0));
		best = best.max(traverse(map, w, (x,     0), 1));
		best = best.max(traverse(map, w, (x, w - 1), 2));
	}

	best
}

fn traverse(map: &[u8], dim: i16, p: (i16, i16), d: u8) -> usize {
	const DIR: [(i8, i8); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];
	fn next((x, y): (i16, i16), d: u8) -> ((i16, i16), u8) {
		let dp = unsafe { *DIR.get_unchecked(d as usize) };
		((x + dp.0 as i16, y + dp.1 as i16), d)
	}

	let mut q = Vec::with_capacity(map.len());
	q.push((p, d));

	let mut seen = vec![0u16; map.len()];

	while let Some((p, d)) = q.pop() {
		if p.0 < 0 || p.0 == dim || p.1 < 0 || p.1 == dim {
			continue
		}

		let i = (p.0 * (dim + 1) + p.1) as usize;
		let s = unsafe { seen.get_unchecked_mut(i) };

		*s += 1 << 4;
		if (*s & 0b1111) & (1 << d) != 0 {
			continue
		}
		*s |= 1 << d;

		let mut nd = [-1i8; 2];

		match unsafe { map.get_unchecked(i) } {
			b'|' if d == 1 || d == 2 => { nd[0] = 0; nd[1] = 3; },
			b'-' if d == 0 || d == 3 => { nd[0] = 1; nd[1] = 2; },
			b'/'  => nd[0] = (d ^ 0b01) as i8,
			b'\\' => nd[0] = (d ^ 0b10) as i8,
			// covers empty cells & matching |-
			_ => nd[0] = d as i8,
		}

		q.push(next(p, nd[0] as u8));
		if nd[1] >= 0 { q.push(next(p, nd[1] as u8)) }
	}

	seen.into_iter().filter(|&x| x > 0).count()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 46);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT), 51);
	}
}
