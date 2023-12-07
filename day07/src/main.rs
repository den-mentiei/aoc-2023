use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let p1 = solve::<false>(&input);
	println!("p1 = {p1}");
	let p2 = solve::<true>(&input);
	println!("p2 = {p2}");

	Ok(())
}

fn solve<const P2: bool>(input: &str) -> i64 {
	let mut top: Vec<_> = input
		.lines()
		.filter_map(|s| {
			let (hand, bid) = s.split_once(' ')?;
			let mut h = [0; 5];
			for (i, b) in hand.bytes().enumerate() {
				h[i] = card_value::<P2>(b) as u8;
			}
			let r = rank::<P2>(hand);
			let b = bid.parse::<i64>().ok()?;

			Some((hand, h, r, b))
		})
		.collect();

	top.sort_by_cached_key(|&(_, h, r, _)| (r, h));

	top
		.into_iter()
		.enumerate()
		.map(|(i, x)| (i as i64 + 1) * x.3)
		.sum()
}

fn rank<const P2: bool>(hand: &str) -> [u8; 2] {
	let mut table = [0; 13];

	let mut js = 0;
	for b in hand.bytes() {
		if P2 && b == b'J' {
			js += 1;
		} else {
			table[card_value::<P2>(b)] += 1;
		}
	}

	table.sort_unstable();
	let mut rank: [u8; 2] = table[11..].try_into().unwrap();
	rank.reverse();
	rank[0] += js;
	rank
}

fn card_value<const P2: bool>(b: u8) -> usize {
	let cards = if !P2 { b"23456789TJQKA" } else { b"J23456789TQKA" };
	cards
		.iter()
		.position(|&c| c == b)
		.unwrap()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

	#[test]
	fn test_part1() {
		assert_eq!(solve::<false>(INPUT), 6440);
	}

	#[test]
	fn test_part2() {
		assert_eq!(solve::<true>(INPUT), 5905);
	}
}
