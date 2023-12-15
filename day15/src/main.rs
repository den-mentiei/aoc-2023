#![feature(vec_push_within_capacity)]
use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let p1 = part1(&input);
	println!("p1 = {p1}");
	let p2 = part2(&input);
	println!("p2 = {p2}");

	Ok(())
}

fn part1(input: &str) -> usize {
	input
		.trim()
		.split(',')
		.map(hash)
		.sum()
}

fn part2(input: &str) -> usize {
	let mut boxes: [Vec<(u64, u8)>; 256] = std::array::from_fn(|_| Vec::new());
	input
		.trim()
		.split(',')
		.for_each(|s| {
			let (l, n) = s.split_once(['=', '-']).unwrap();
			let (k, h) = key_hash(l);
			let b      = &mut boxes[h];
			let i      = b.iter().position(|x| x.0 == k);

			if let Ok(n) = n.parse() {
				if let Some(i) = i {
					b[i].1 = n;
				} else {
					b.push((k, n));
				}
			} else if let Some(i) = i {
				b.remove(i);
			}
		});

	boxes
		.into_iter()
		.zip(1..)
		.filter(|(b, _)| !b.is_empty())
		.flat_map(|(b, i)|
			b
				.into_iter()
				.zip(1..)
				.map(move |((_, n), j)| i * j * n as usize)
		)
		.sum()
}

fn key_hash(s: &str) -> (u64, usize) {
	let (k, h) =s
		.bytes()
		.fold((0, 0u8), |(k, h), b| {
			let h = h.wrapping_add(b).wrapping_mul(17);
			let k = (k << 8) | ((b - b'a') as u64);
			(k, h)
		});
	(k, h as usize)
}

fn hash(s: &str) -> usize {
	s.bytes().fold(0u8, |h, b| h.wrapping_add(b).wrapping_mul(17)) as usize
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(INPUT), 1320);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(INPUT), 145);
	}
}
