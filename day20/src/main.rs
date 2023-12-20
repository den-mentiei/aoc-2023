use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let nodes = parse(&input);
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
	let mut xs = Vec::new();
	for &n in &nodes[0].1 {
		let mut x = 0;
		let mut b = 0;
		let mut i = Some(n);

		// Flip-flops connected to conjunction are ones,
		// everything else is zeros.
		while let Some(n) = i {
			let (_, ns) = &nodes[n];
			if ns.len() == 2 || nodes[ns[0]].0 == 2 {
				x |= 1 << b;
			}
			b += 1;

			i = ns
				.iter()
				.find(|&&n| nodes[n].0 == 1)
				.copied();
		}

		xs.push(x);
	}

	xs.into_iter().reduce(lcm).unwrap_or_default()
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first.max(second);
    let mut min = first.min(second);

    loop {
        let x = max % min;
        if x == 0 {
            return min;
        }
        max = min;
        min = x;
    }
}

fn parse(input: &str) -> Vec<(u8, Vec<usize>)> {
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

	nodes
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
		assert_eq!(part1(&parse(INPUT1)), 32000000);
		assert_eq!(part1(&parse(INPUT2)), 11687500);
	}
}
