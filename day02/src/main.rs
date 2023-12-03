use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let (p1, p2) = solve(&input);
	println!("p1 = {p1}");
	println!("p2 = {p2}");

	Ok(())
}

fn solve(input: &str) -> (i32, i32) {
	const LIMITS: [i32; 3] = [12, 13, 14];
	input
		.lines()
		.filter_map(|s| {
			let s         = s.strip_prefix("Game ")?;
			let (id, s)   = s.split_once(": ")?;
			let id        = id.parse::<i32>().ok()?;
			let worst     = s
				.split(';')
				.map(|r| r
						.split(',')
						.filter_map(|s| {
							let (k, c) = s.trim_start().split_once(' ')?;
							let k = k.parse::<i32>().ok()?;
							let c = parse_color(c);
							Some((k, c))
						})
						.fold([0, 0, 0], |mut acc, (k, c)| {
							acc[c] = acc[c].max(k);
							acc
						})
				)
				.fold([0, 0, 0], |mut acc, cubes| {
					acc[0] = acc[0].max(cubes[0]);
					acc[1] = acc[1].max(cubes[1]);
					acc[2] = acc[2].max(cubes[2]);
					acc
				});
			let legit = worst[0] <= LIMITS[0] && worst[1] <= LIMITS[1] && worst[2] <= LIMITS[2];
			let power = worst[0] * worst[1] * worst[2];
			Some((id, legit, power))
		})
		.fold((0, 0), |acc, (id, legit, power)| (if legit { acc.0 + id } else { acc.0 }, acc.1 + power))
}

fn parse_color(s: &str) -> usize {
	match s {
		"red"   => 0,
		"green" => 1,
		"blue"  => 2,
		_ => panic!("Unexpected color"),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

	#[test]
	fn test_part1() {
		assert_eq!(solve(INPUT).0, 8);
	}

	#[test]
	fn test_part2() {
		assert_eq!(solve(INPUT).1, 2286);
	}
}
