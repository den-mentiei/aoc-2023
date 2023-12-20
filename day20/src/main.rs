use std::fs;
use std::env;
use std::fmt::Write;
use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let (ids, nodes) = parse(&input);

	if env::args().nth(1).is_some() {
		let dot = generate_dot(&ids, &nodes);
		fs::write("graph.dot", dot)?;
		println!("Saved an input visualization, use it like this:");
		println!("dot -Tsvg graph.dot -o graph.svg");
	}

	let p1 = part1(&nodes);
	println!("p1 = {p1}");
	let p2 = part2(&nodes);
	println!("p2 = {p2}");

	Ok(())
}

fn part1(nodes: &[(u8, Vec<usize>)]) -> u64 {
	let mut st  = vec![0u8;  nodes.len()];
	let mut inp = vec![0u64; nodes.len()];
	let mut ind = vec![0u8;  nodes.len()];
	for (_, ns) in nodes {
		for n in ns {
			if let Some(ty) = nodes.get(*n) {
				if ty.0 == 2 {
					ind[*n] += 1;
				}
			}
		}
	}

	let mut sum = [0u64; 2];

	let mut q = VecDeque::new();
	for _ in 0..1000 {
		q.push_back((0, 0, 0u8));

		while let Some((i, s, p)) = q.pop_front() {
			sum[p as usize] += 1;

			let Some((ty, ns)) = nodes.get(i) else { continue };

			match (ty, p) {
				(0, _) => for n in ns { q.push_back((*n, i, p)) },
				(1, 0) => {
					st[i] ^= 1;
					for n in ns { q.push_back((*n, i, st[i] as u8)) }
				},
				(1, _) => (),
				(2, p) => {
					let b = (p as u64) << s;
					let m = 1u64 << s;
					inp[i] = (inp[i] & !m) | b;
					st[i]  = if inp[i].count_ones() == (ind[i] as u32) {
						0
					} else {
						1
					};
					for n in ns { q.push_back((*n, i, st[i])) }
				},
				_ => unreachable!(),
			}
		}
	}

	sum[0] * sum[1]
}

fn part2(nodes: &[(u8, Vec<usize>)]) -> u64 {
	let mut prod = 1;
	for &n in &nodes[0].1 {
		let mut x = 0;
		let mut b = 0;
		let mut i = Some(n);

		// Flip-flops connected to conjunction are ones,
		// everything else is zeros.
		while let Some(n) = i {
			let (_, ns) = &nodes[n];
			if ns.iter().any(|&n| nodes[n].0 == 2) {
				x |= 1 << b;
			}
			b += 1;

			i = ns
				.iter()
				.find(|&&n| nodes[n].0 == 1)
				.copied();
		}

		prod *= x;
	}
	prod
}

fn parse(input: &str) -> (Vec<&str>, Vec<(u8, Vec<usize>)>) {
	fn get_or_add_id<'s>(id: &'s str, ids: &mut Vec<&'s str>) -> usize {
		if let Some(i) = ids.iter().position(|&s| s == id) {
			i
		} else {
			let i = ids.len();
			ids.push(id);
			i
		}
	}

	let mut ids   = vec!["broadcaster"];
	let mut wires = input
		.lines()
		.filter_map(|s| {
			let (id, s)  = s.split_once(" -> ")?;
			let (id, ty) = match id.as_bytes()[0] {
				b'%' => (&id[1..], 1u8),
				b'&' => (&id[1..], 2u8),
				_    => (id,       0u8),
			};
			let id = get_or_add_id(id, &mut ids);
			let ns = s
				.split(", ")
				.map(|id| get_or_add_id(id, &mut ids))
				.collect();
			Some((id, (ty, ns)))
		})
		.collect::<HashMap<_, _>>();

	let mut nodes = Vec::with_capacity(ids.len());
	for i in 0..ids.len() {
		if let Some(n) = wires.remove(&i) {
			nodes.push(n);
		} else {
			nodes.push((0, Vec::new()));
		}
	}

	(ids, nodes)
}

fn generate_dot(ids: &[&str], nodes: &[(u8, Vec<usize>)]) -> String {
	let mut buf = String::with_capacity(ids.len() * 16);

	_ = writeln!(&mut buf, "digraph {{");
	_ = writeln!(&mut buf, "  {{");
	_ = writeln!(&mut buf, "    broadcaster [shape=oval]");

	let mut write_shapes = |t: u8, s: &str| {
		let nodes = nodes
			.iter()
			.enumerate()
			.filter(|&(_, &(ty, _))| ty == t)
			.map(|(i, _)| i);
		_ = write!(&mut buf, "    ");
		for (i, id) in nodes.enumerate() {
			if i != 0 {
				_ = write!(&mut buf, ", ");
			}
			_ = write!(&mut buf, "{}", ids[id]);
		}
		_ = writeln!(&mut buf, " [shape={}]", s);
	};
	write_shapes(1, "diamond");
	write_shapes(2, "invhouse");
	_ = writeln!(&mut buf, "    rx [shape=oval]");

	for (i, (_, ns)) in nodes.iter().enumerate() {
		for &n in ns {
			_ = writeln!(&mut buf, "    {} -> {}", ids[i], ids[n]);
		}
	}

	_ = writeln!(&mut buf, "  }}");

	_ = writeln!(&mut buf, "}}");

	buf
}


#[cfg(test)]
mod tests {
	use super::*;

	const INPUT1: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#;

	const INPUT2: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(&parse(INPUT1).1), 32000000);
		assert_eq!(part1(&parse(INPUT2).1), 11687500);
	}
}
