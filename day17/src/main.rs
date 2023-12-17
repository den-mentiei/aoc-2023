use std::collections::BinaryHeap;
use std::cmp::Reverse as Rev;
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

fn solve(input: &str) -> (u32, u32) {
	let b = input.as_bytes();
	let w = b.iter().position(|&x| x == b'\n').unwrap();

	let p1 = dijkstra(b, w, 1, 3);
	let p2 = dijkstra(b, w, 4, 10);

	(p1, p2)
}

fn dijkstra(map: &[u8], w: usize, min: u8, max: u8) -> u32 {
	let n = map.len();

	let mut seen = vec![0u8; n];
	let mut cost = vec![u32::MAX; 2 * n]; // vertical | horizontal

	let mut q = BinaryHeap::new();
	q.push((Rev(0), 0, 0));
	q.push((Rev(0), 0, 1));

	let next = |p: usize, d: u8| Some(match d {
		0 if p > w                  => p - w - 1, // n
		1 if (p + 1) % (w + 1) != w => p + 1,     // e
		2 if p < n - (w + 1)        => p + w + 1, // s
		3 if p % (w + 1) != 0       => p - 1,     // w
		_ => return None,
	});

	while let Some((Rev(c), p, d)) = q.pop() {
		// bottom-right corner
		if p == n - 2 { return c; }

		if seen[p] & (1 << d) != 0 { continue; }
		seen[p] |= 1 << d;

		// n    e    w
		// 0 -> 1 -> 3
		// e    n    s
		// 1 -> 0 -> 2
		// s    w    e
		// 2 -> 3 -> 1
		// w    s    n
		// 3 -> 2 -> 0
		let od = d ^ 1; // orientation: vertical or horizontal
		for nd in [od, od ^ 2] {
			let mut sum = 0;
			let mut np  = p;
			for dist in 1..=max {
				if let Some(op) = next(np, nd) {
					sum += (map[op] - b'0') as u32;
					if dist >= min {
						let nc = c + sum;
						let i  = (op << 1) | od as usize;
						if cost[i] > nc {
							cost[i] = nc;
							q.push((Rev(nc), op, od));
						}
					}
					np = op;
				}
			}
		}
	}

	0
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT1: &str = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#;

	const INPUT2: &str = r#"111111111111
999999999991
999999999991
999999999991
999999999991
"#;

	#[test]
	fn test_part1() {
		assert_eq!(solve(INPUT1).0, 102);
	}

	#[test]
	fn test_part2() {
		assert_eq!(solve(INPUT1).1, 94);
		assert_eq!(solve(INPUT2).1, 71);
	}
}
