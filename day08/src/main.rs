use std::collections::HashMap;
use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let (guide, network) = parse(&input);
	let p1 = part1(guide, &network);
	println!("p1 = {p1}");
	let p2 = part2(guide, &network);
	println!("p2 = {p2}");

	Ok(())
}

fn parse(input: &str) -> (&str, HashMap<u64, u64>) {
	let (guide, network) = input.split_once("\n\n").unwrap();

	let network = network
		.lines()
		.map(|s| {
			let bytes = s.as_bytes();
			let node  = encode(&bytes[0..3]);
			let lr    = encode(&bytes[7..10]) | encode(&bytes[12..15]) << 32;
			(node, lr)
		})
		.collect();

	(guide, network)
}

#[inline(always)]
fn encode(b: &[u8]) -> u64 {
	((b[0] - b'0') as u64) << 12 | ((b[1] - b'0') as u64) << 6 | (b[2] - b'0') as u64
}

fn part1(guide: &str, network: &HashMap<u64, u64>) -> i64 {
	trace::<false>(encode(b"AAA"), guide, network)
}

fn part2(guide: &str, network: &HashMap<u64, u64>) -> i64 {
	network
		.keys()
		.filter(|&&k| k & 0b111111 == (b'A' - b'0') as u64)
		.map(|&n| trace::<true>(n, guide, network))
		.reduce(lcm)
		.unwrap_or_default()
}

fn trace<const P2: bool>(start: u64, guide: &str, network: &HashMap<u64, u64>) -> i64 {
	guide
		.bytes()
		.cycle()
		.scan(start, |at, s| {
			let next = network[at];
			*at = if s == b'L' {
				next & 0xFFFF_FFFF
			} else {
				next >> 32
			};
			Some(*at & 0b111111 == (b'Z' - b'0') as u64)
		})
		.position(|n| n)
		.unwrap() as i64
		+ 1
}

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
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

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT1: &str = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;

	const INPUT2: &str = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;

	const INPUT3: &str = r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;

	#[test]
	fn test_part1() {
		let (guide, network) = parse(INPUT1);
		assert_eq!(part1(guide, &network), 2);
		let (guide, network) = parse(INPUT2);
		assert_eq!(part1(guide, &network), 6);
	}

	#[test]
	fn test_part2() {
		let (guide, network) = parse(INPUT3);
		assert_eq!(part2(guide, &network), 6);
	}
}
