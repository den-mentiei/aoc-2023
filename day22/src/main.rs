#![feature(iter_array_chunks)]

use std::collections::{HashSet, VecDeque};
use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let s = std::time::Instant::now();
	let (p1, p2) = solve(&input);
	let d = s.elapsed();
	println!("both parts with parsing took {d:?}");
	println!("p1 = {p1}");
	println!("p2 = {p2}");

	Ok(())
}

fn solve(input: &str) -> (usize, usize) {
	let mut bricks = input
		.lines()
		.flat_map(|s| {
			s
				.split(['~', ','])
				.map(|s| s.parse::<i16>().unwrap())
				.array_chunks::<6>()
		})
		.collect::<Vec<_>>();

	bricks.sort_unstable_by_key(|&[_, _, z, _, _, _]| z);

	let mut hm = [[0; 10]; 10];
	let mut im = [[-1i16; 10]; 10];

	let mut adj  = vec![Vec::new(); bricks.len()];
	let mut cant = HashSet::new();
	let mut temp = HashSet::new();

	for (i, &[x0, y0, z0, x1, y1, z1]) in bricks.iter().enumerate() {
		let mut top  = 0;

		for x in x0..=x1 {
			for y in y0..=y1 {
				top = top.max(hm[x as usize][y as usize]);
			}
		}

		let h = z1 - z0 + 1;
		for x in x0..=x1 {
			for y in y0..=y1 {
				let x = x as usize;
				let y = y as usize;
				let j = im[x][y];
				if hm[x][y] == top && j != -1 {
					temp.insert(j);
				}

				hm[x][y] = top + h;
				im[x][y] = i as i16;
			}
		}

		for &j in &temp {
			adj[j as usize].push(i);
		}

		if temp.len() == 1 {
			cant.extend(temp.drain());
		} else {
			temp.clear();
		}
	}

	let p1 = bricks.len() - cant.len();

	let mut p2 = 0;

	let mut ind = vec![0; bricks.len()];
	for &j in adj.iter().flat_map(|v| v.iter()) {
		ind[j] += 1;
	}
	let mut indw = ind.clone();

	let mut q   = VecDeque::new();
	for i in 0..bricks.len() {
		q.push_back(i);
		while let Some(x) = q.pop_front() {
			if x != i {
				p2 += 1;
			}
			for &y in &adj[x] {
				let d = indw[y] - 1;
				indw[y] = d;
				if d == 0 {
					q.push_back(y);
				}
			}
		}

		indw.copy_from_slice(&ind);
	}

	(p1, p2)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"#;

	#[test]
	fn test_part1() {
		assert_eq!(solve(INPUT).0, 5);
	}

	#[test]
	fn test_part2() {
		assert_eq!(solve(INPUT).1, 7);
	}
}
