#![feature(let_chains)]

use std::collections::{VecDeque, HashSet, HashMap};
use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let p1 = solve::<true>(&input);
	println!("p1 = {p1}");
	let p2 = solve::<false>(&input);
	println!("p2 = {p2}");

	Ok(())
}

fn solve<const P1: bool>(input: &str) -> i32 {
	let edges = make_graph::<P1>(input);

	assert!(edges.len() <= 64);

	let mut max = 0;
	let mut q   = Vec::new();

	q.push((0, 0, 0u64));
	while let Some((i, d, mut seen)) = q.pop() {
		seen |= 1 << i;

		if i == 1 {
			max = max.max(d);
		}

		for &(ni, nd) in &edges[i] {
			if seen & (1 << ni) == 0 {
				q.push((ni, d + nd, seen));
			}
		}
	}

	max
}

fn make_graph<const P1: bool>(input: &str) -> Vec<Vec<(usize, i32)>> {
	let b = input.as_bytes();
	let n = b.len();
	let w = b.iter().position(|&x| x == b'\n').unwrap();

	let next = |p: usize, d: u8| Some(match d {
		0 if p > w                  => p - (w + 1),   // n
		1 if (p + 1) % (w + 1) != w => p + 1,         // e
		2 if p < n - (w + 1)            => p + w + 1, // s
		3 if p % (w + 1) != 0           => p - 1,     // w
		_ => return None,
	});

	let mut lookup = HashMap::new();
	let mut edges  = Vec::new();

	let start  = 1;
	let finish = (w - 1) * (w + 1) + (w - 2);
	lookup.insert(start, 0);
	edges.push(Vec::new());
	lookup.insert(finish, 1);
	edges.push(Vec::new());

	let mut p = 0;
	for _ in 0..w {
		for _ in 0..w {
			if b[p] == b'#' {
				p += 1;
				continue;
			}

			let mut neighbours = 0;
			for d in [0, 1, 2, 3] {
				if let Some(np) = next(p, d) && b[np] != b'#' {
					neighbours += 1;
				}
			}
			if neighbours > 2 {
				let id = edges.len();
				edges.push(Vec::new());
				lookup.insert(p, id);
			}
			p += 1;
		}
		p += 1;
	}

	let mut seen = HashSet::new();
	let mut q    = VecDeque::new();
	for (&pv, &idv) in lookup.iter() {
		q.push_back((pv, 0));

		seen.clear();

		while let Some((p, d)) = q.pop_front() {
			if !seen.insert(p) { continue }

			if p != pv {
				if let Some(&id) = lookup.get(&p) {
					edges[idv].push((id, d));
					continue;
				}
			}

			let nope = [b'^', b'>', b'v', b'<'];
			for dir in [0, 1, 2, 3] {
				if P1 && b[p] != b'.' && b[p] != nope[dir as usize] { continue }
				if let Some(np) = next(p, dir) && b[np] != b'#' {
					q.push_back((np, d + 1));
				}
			}
		}
	}

	// println!("digraph {{");
	// for (v, e) in edges.iter().enumerate() {
	// 	for (nv, d) in e.iter() {
	// 		println!("  {v} -> {nv} [label=\"{d}\"]");
	// 	}
	// }
	// println!("}}");

	edges
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
"#;

	#[test]
	fn test_part1() {
		assert_eq!(solve::<true>(INPUT), 94);
	}

	#[test]
	fn test_part2() {
		assert_eq!(solve::<false>(INPUT), 154);
	}
}
