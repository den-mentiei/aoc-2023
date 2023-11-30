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

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#""#;

	#[test]
	fn test_part1() {
	}

	#[test]
	fn test_part2() {
	}
}
