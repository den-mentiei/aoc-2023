#![feature(iter_array_chunks)]

use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let (rs, ps, s) = parse(&input);
	let p1 = part1(&rs, ps, s);
	println!("p1 = {p1}");
	let p2 = part2(&rs, s);
	println!("p2 = {p2}");

	Ok(())
}

fn part1(rs: &[Rules], ps: Vec<[u16; 4]>, in_id: usize) -> u64 {
	let mut sum = 0;

	for xmas in ps {
		let mut w = in_id;
		while w >= 2 {
			w = rs[w - 2]
				.iter()
				.find(|&&(r, _)| {
					match r {
						Some((i, b'<', n)) => xmas[i as usize] < n,
						Some((i, b'>', n)) => xmas[i as usize] > n,
						_ => true,
					}
				})
				.unwrap()
				.1;
		}
		if w == 1 {
			for x in xmas { sum += x as u64 }
		}
	}

	sum
}

fn part2(rs: &[Rules], in_id: usize) -> u64 {
	let mut sum = 0;

	let mut q = vec![(in_id, [(1, 4000); 4])];
	while let Some((w, mut xmas)) = q.pop() {
		match w {
			0 => (),
			1 => sum += xmas.into_iter().fold(1, |a, x| a * (x.1 - x.0 + 1) as u64),
			_ => {
				for &(r, nw) in rs[w - 2].iter() {
					match r {
						Some((i, b'<', n)) => {
							let i = i as usize;
							if xmas[i].0 < n {
								let mut nx = xmas;
								nx[i].1 = nx[i].1.min(n - 1);
								q.push((nw, nx));
							}
							xmas[i].0 = n;
						},
						Some((i, b'>', n)) => {
							let i = i as usize;
							if xmas[i].1 > n {
								let mut nx = xmas;
								nx[i].0 = nx[i].0.max(n + 1);
								q.push((nw, nx));
							}
							xmas[i].1 = n;
						},
						_ => q.push((nw, xmas)),
					}
				}
			},
		}
	}

	sum
}

type Rules = Vec<(Option<(u8, u8, u16)>, usize)>;

fn parse(input: &str) -> (Vec<Rules>, Vec<[u16; 4]>, usize) {
	fn get_or_add_id<'s>(id: &'s str, ids: &mut Vec<&'s str>) -> usize {
		if let Some(i) = ids.iter().position(|&s| s == id) {
			i
		} else {
			let i = ids.len();
			ids.push(id);
			i
		}
	}

	let (rs, ps) = input.split_once("\n\n").unwrap();
	let (ws, mut rs) = rs
		.lines()
		.fold((vec!["R", "A"], Vec::new()), |(mut ws, mut rs), s| {
			let (w, s) = s.split_once('{').unwrap();
			let w      = get_or_add_id(w, &mut ws);
			let cs     = s[..s.len() - 1]
				.split(',')
				.map(|r| {
					if let Some((c, d)) = r.split_once(':') {
						let n = c[2..].parse().unwrap();
						let b = c.as_bytes();
						let i = match b[0] {
							b'x' => 0,
							b'm' => 1,
							b'a' => 2,
							b's' => 3,
							_    => unreachable!(),
						};

						(Some((i, b[1], n)), get_or_add_id(d, &mut ws))
					} else {
						(None, get_or_add_id(r, &mut ws))
					}
				})
				.collect();
			rs.push((w, cs));
			(ws, rs)
		});
	rs.sort_unstable_by_key(|r| r.0);
	let rs = rs.into_iter().map(|r| r.1).collect();

	let ps = ps
		.lines()
		.flat_map(|s| {
			s[1..s.len() - 1]
				.split(',')
				.filter_map(|x| x[2..].parse().ok())
		})
		.array_chunks()
		.collect();

	let in_id = ws.into_iter().position(|x| x == "in").unwrap();

	(rs, ps, in_id)
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#;

	#[test]
	fn test_part1() {
		let (rs, ps, s) = parse(INPUT);
		assert_eq!(part1(&rs, ps, s), 19114);
	}

	#[test]
	fn test_part2() {
		let (rs, _, s) = parse(INPUT);
		assert_eq!(part2(&rs, s), 167409079868000);
	}
}
