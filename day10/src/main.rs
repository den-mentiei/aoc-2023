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
	let m = input.as_bytes();
	let w = m.iter().position(|&b| b == b'\n').unwrap();
	let s = m.iter().position(|&b| b == b'S').unwrap();

	let (mut p, mut d) = if matches!(m[s - w - 1], b'|' | b'F' | b'7') {
		(s - w - 1, 0)
	} else if matches!(m[s + w + 1], b'|' | b'J' | b'L') {
		(s + w + 1, 2)
	} else {
		(s - 1, 3)
	};

	let mut path = Vec::new();
	loop {
		path.push(p);
		match (unsafe { m.get_unchecked(p) }, d) {
			(b'|', 0) => p -= w + 1,
			(b'|', 2) => p += w + 1,
			(b'-', 3) => p -= 1,
			(b'-', 1) => p += 1,
			(b'L', 3) | (b'J', 1) => { p -= w + 1; d = 0; },
			(b'7', 1) | (b'F', 3) => { p += w + 1; d = 2; },
			(b'L', 2) | (b'F', 0) => { p += 1;     d = 1; },
			(b'7', 0) | (b'J', 2) => { p -= 1;     d = 3; },
			(b'S', _) => break,
			_ => unreachable!(),
		}
	};

	let w   = w as i32 + 1;
	let pnt = |p: usize, w: i32| {
		let p = p as i32;
		(p / w, p % w)
	};

	let mut area = 0;
	for i in 0..path.len() {
		let j = (i + 1) % path.len();
		let p0 = pnt(path[i], w);
		let p1 = pnt(path[j], w);
		area += p0.0 * p1.1 - p1.0 * p0.1;
	}

	let area = area.abs() / 2 + 1 - path.len() as i32 / 2;

	(path.len() as i32 / 2, area)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT1: &str = r#".....
.S-7.
.|.|.
.L-J.
....."#;

	const INPUT2: &str = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;

	const INPUT3: &str = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;

	const INPUT4: &str = r#".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."#;

	#[test]
	fn test_part1() {
		assert_eq!(solve(INPUT1).0, 4);
		assert_eq!(solve(INPUT2).0, 8);
	}

	#[test]
	fn test_part2() {
		assert_eq!(solve(INPUT3).1, 4);
		assert_eq!(solve(INPUT4).1, 8);
	}
}
