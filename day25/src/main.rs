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
	let mut edges = Vec::new();
	let mut adj   = Vec::new();

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
		let id = get_or_add_id(&s[..3], &mut adj);
		for n in s[5..].split(' ') {
			let nid = get_or_add_id(n, &mut adj);
			let e = edges.len();
			edges.push((id, nid));
			adj[id].push((nid, e));
			adj[nid].push((id, e));
		}
	}

	let (c0, c1) = karger(&edges, ids.len());

	c0 * c1
}

fn karger(edges: &[(usize, usize)], n: usize) -> (usize, usize) {
	let mut rng = rand::thread_rng();

	loop {
		let mut ds  = DisjointSet::new(n);
		let mut v   = Vec::new();
		let m       = edges.len();
		for _ in 2..n {
			v.clear();
			v.extend((0..m).filter(|&i| ds.distinct(edges[i].0, edges[i].1)));
			let e = v[rng.gen_range(0..v.len())];
			let (s, t) = edges[e];
			ds.union(s, t);
		}
		let cut = (0..m)
			.filter(|&i| ds.distinct(edges[i].0, edges[i].1))
			.collect::<Vec<_>>();
		if cut.len() == 3 {
			let p0 = ds.find(edges[cut[0]].0);
			let c0 = ds.count[p0];
			let p1 = ds.find(edges[cut[0]].1);
			let c1 = ds.count[p1];
			return (c0, c1)
		}
	}
}

#[derive(Debug)]
struct DisjointSet {
	parent: Vec<usize>,
	count:  Vec<usize>,
}

impl DisjointSet {
	fn new(n: usize) -> Self {
		Self {
			parent: (0..n).collect(),
			count: vec![1; n],
		}
	}

	fn distinct(&mut self, x: usize, y: usize) -> bool {
		self.find(x) != self.find(y)
	}

	fn find(&mut self, i: usize) -> usize {
		let mut i = i;
		while self.parent[i] != i {
			let p = self.parent[self.parent[i]];
			self.parent[i] = p;
			i = p;
		}
		i
	}

	fn union(&mut self, x: usize, y: usize) {
		let px = self.find(x);
		let py = self.find(y);
		if px == py { return }

		if self.count[px] <= self.count[py] {
			self.parent[px] = py;
			self.count[py] += self.count[px];
		} else {
			self.parent[py] = px;
			self.count[px] += self.count[py];
		}
	}
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
