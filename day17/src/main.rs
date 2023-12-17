use std::collections::BinaryHeap;
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
	let b = input.as_bytes();
	let w = b.iter().position(|&x| x == b'\n').unwrap();
	let h = input.trim_end().len() / w;

	let p1 = dijkstra(b, w, h, 1, 3);
	let p2 = dijkstra(b, w, h, 4, 10);

	(p1, p2)
}

fn dijkstra(map: &[u8], w: usize, h: usize, min: i8, max: i8) -> i32 {
	const DIR: [(i8, i8); 4] = [(-1, 0), (0, 1), (0, -1), (1, 0)];

	let mut dists = vec![i32::MAX; w * h * 4];
	let mut front = BinaryHeap::from_iter([(0, (0, 0, 0))]);

	while let Some((cost, (r, c, d))) = front.pop() {
		if (r, c) == ((h - 1) as i16, (w - 1) as i16) {
			return -cost;
		}

		let didx = r as usize * w * 4 + c as usize * 4 + d as usize;
		if -dists[didx] > c as i32 {
			continue;
		}

		for (nd, &(dr, dc)) in DIR.iter().enumerate() {
			let nd = nd as i8;
			// don't go backward or further than max steps
			if d == nd || d == (4 - nd - 1) { continue; }

			let mut ncost = -cost;
			for dist in 1..=max {
				let nr = (r + (dr * dist) as i16) as usize;
				let nc = (c + (dc * dist) as i16) as usize;
				if nr >= h || nc >= w { continue; }

				ncost += (map[nr * (w + 1) + nc] - b'0') as i32;
				if dist < min {
					continue;
				}

				let k = (nr as i16, nc as i16, nd);
				let d = nr as usize * w * 4 + nc as usize * 4 + nd as usize;
				if ncost < dists[d] {
					dists[d] = ncost;
					front.push((-ncost, k));
				}
			}
		}
	}

	unreachable!()
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
