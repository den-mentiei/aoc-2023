use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::cmp::Reverse as Rev;
use std::io::{self, Read};

use rand::Rng;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let p1 = part1(&input);
	println!("p1 = {p1}");

	Ok(())
}

fn part1(input: &str) -> usize {
	let mut ids   = Vec::new();
	let mut adj   = Vec::new();
	let mut edges = 0;

	let mut get_or_add_id = |id, adj: &mut Vec<_>| {
		if let Some(i) = ids.iter().position(|&s| s == id) {
			i
		} else {
			let i = ids.len();
			ids.push(id);
			adj.push(Vec::new());
			i
		}
	};

	for s in input.lines() {
		let u = get_or_add_id(&s[..3], &mut adj);
		for v in s[5..].split(' ') {
			let v = get_or_add_id(v, &mut adj);
			adj[u].push((v, edges));
			adj[v].push((u, edges));
			edges += 1;
		}
	}

	let n = ids.len();

	let mut freq = HashMap::<u32, u32>::new();

	let mut rng  = rand::thread_rng();

	let mut cost = vec![u32::MAX; n];
	let mut q    = BinaryHeap::new();

	let mut seen = vec![false; n];
	let mut dfsq = VecDeque::new();

	let c0 = loop {
		for _ in 0..80 {
			let src = rng.gen_range(0..n);
			let dst = loop {
				let i = rng.gen_range(0..n);
				if i != src {
					break i;
				}
			};

			cost.fill(u32::MAX);

			q.clear();
			q.push((Rev(0), src));
			cost[src] = 0;

			while let Some((Rev(c), u)) = q.pop() {
				if u == dst {
					break;
				}

				for &(v, e) in &adj[u] {
					let nc = c + 1;
					if nc < cost[v] {
						cost[v] = nc;
						*freq.entry(e).or_default() += 1;
						q.push((Rev(nc), v));
					}
				}
			}
		}

		let mut cut = [(0, 0); 3];
		for (&k, &v) in freq.iter() {
			if v > cut[0].1 {
				cut[2] = cut[1];
				cut[1] = cut[0];
				cut[0] = (k, v);
			} else if v > cut[1].1 {
				cut[2] = cut[1];
				cut[1] = (k, v);
			} else if v > cut[2].1 {
				cut[2] = (k, v);
			}
		}
		let (e0, e1, e2) = (cut[0].0, cut[1].0, cut[2].0);

		let mut c0 = 0;
		seen.fill(false);

		dfsq.clear();
		dfsq.push_back(0);
		while let Some(u) = dfsq.pop_front() {
			for &(v, e) in &adj[u] {
				if e != e0 && e != e1 && e != e2 {
					if seen[v] { continue }
					seen[v] = true;
					c0 += 1;
					dfsq.push_back(v);
				}
			}
		}

		if c0 != n {
			break c0;
		}
		// println!("doing more sampling");
	};

	c0 * (n - c0)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 54);
	}
}
