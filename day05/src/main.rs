#![feature(iter_array_chunks)]

use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let (seeds, layers) = parse(&input).expect("Bad input");
	let p1 = part1(&seeds, &layers);
	println!("p1 = {p1}");
	let p2 = part2(&seeds, &layers);
	println!("p2 = {p2}");

	Ok(())
}

fn parse(input: &str) -> Option<(Vec<i64>, Vec<Vec<(i64, i64, i64)>>)> {
	let mut sections = input.split("\n\n");

	let seeds = sections
		.next()?
		.strip_prefix("seeds: ")?
		.split(' ')
		.filter_map(|s| s.parse().ok())
		.collect();

	let layers = sections
		.map(|s| s
			 .lines()
			 .skip(1)
			 .filter_map(|s| {
				 let [d, s, n] = s
					 .split(' ')
					 .filter_map(|x| x.parse().ok())
					 .array_chunks()
					 .next()?;
				 Some((d, s, n))
			 })
			 .collect()
		)
		.collect();

	Some((seeds, layers))
}

fn part1(seeds: &[i64], layers: &[Vec<(i64, i64, i64)>]) -> i64 {
	seeds
		.iter()
		.map(|&s|
			 layers
			 .iter()
			 .fold(s, |s, layer| {
				 layer
					 .iter()
					 .find(|&&(_, src, n)| (src..src + n).contains(&s))
					 .map(|&(dst, src, _)| dst + s - src)
					 .unwrap_or(s)
			 })
		)
		.min()
		.unwrap_or_default()
}

fn part2(seeds: &[i64], layers: &[Vec<(i64, i64, i64)>]) -> i64 {
	let seeds = seeds
		.chunks_exact(2)
		.map(|c| (c[0], c[0] +  c[1]))
		.collect::<Vec<_>>();
	layers
		.iter()
		.fold(seeds, |seeds, layer|
			  seeds
			  	.iter()
			  	.flat_map(|&(start, end)| {
					let mut mapped   = Vec::new();
					let mut unmapped = vec![(start, end)];
					for &(dst, src, n) in layer {
						let mut temp = Vec::new();
						for (start, end) in unmapped {
							let p = (start, end.min(src));
							let f = (start.max(src), (src + n).min(end));
							let s = ((src + n).max(start), end);
							if p.0 < p.1 { temp.push(p); } // prefix is unmapped
							if f.0 < f.1 { mapped.push((f.0 - src + dst, f.1 - src + dst)); } // fully mapped
							if s.0 < s.1 { temp.push(s); } // suffix is unmapped
						}
						unmapped = temp;
					}
					mapped.extend(unmapped);
					mapped
				})
			  .collect()
		)
		.into_iter()
		.map(|r| r.0)
		.min()
		.unwrap_or_default()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

	#[test]
	fn test_part1() {
		let (seeds, layers) = parse(INPUT).unwrap();
		assert_eq!(part1(&seeds, &layers), 35);
	}

	#[test]
	fn test_part2() {
		let (seeds, layers) = parse(INPUT).unwrap();
		assert_eq!(part2(&seeds, &layers), 46);
	}
}
