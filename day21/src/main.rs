use std::collections::HashSet;
use std::io::{self, Read};
use std::mem::swap;

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

fn solve(input: &str) -> (i64, i64) {
	let b = input.as_bytes();
	let w = b.iter().position(|&x| x == b'\n').unwrap();
	let s = b.iter().position(|&x| x == b'S').unwrap();

	let (sr, sc) = ((s / (w + 1)) as i32, (s % (w + 1)) as i32);

	const N: usize = 26501365;
	const DIR: [(i8, i8); 4] = [
		(-1,  0), // n
		( 0, -1), // w
		( 0,  1), // e
		( 1,  0), // s
	];

	let mut bb = HashSet::new();
	let mut fb = HashSet::new();
	fb.insert((sr, sc));

	let mut ps = [0i64; 4];
	let mut i  = 1;
	let mut s  = 0;
	while i != 4 {
		if s == 64 {
			ps[0] = fb.len() as i64;
		}
		if s % w == N % w {
			ps[i] = fb.len() as i64;
			i += 1;
		}
		for (r, c) in fb.drain() {
			for (dr, dc) in DIR {
				let r  = r + dr as i32;
				let c  = c + dc as i32;
				let rm = r.rem_euclid(w as i32) as usize;
				let cm = c.rem_euclid(w as i32) as usize;
				if b[rm * (w + 1) + cm] != b'#' {
					bb.insert((r, c));
				}
			}
		}
		swap(&mut fb, &mut bb);
		s += 1;
	}

	// f(n) is number of spaces after n steps
	// f(n) is quadratic as grid is square and there are no
	// obstacles around the starting point.
	// hence, we can calculate:
	// f(), f(W), f(2W) and interpolate the answer for
	// f(26501365/W) == f(202300)
	let n  = (N / w) as i64;

	// newton poly
	let y0 = ps[1]; // f(0)
	let y1 = ps[2]; // f(1)
	let y2 = ps[3]; // f(2)
	// P(x) = y0
	//      + (y0 / (x0 - x1) + (y1) / (x1 - x0))*(x - x0)
	//      + (
	//          y0 / ((x0 - x1)*(x0 - x2))
	//        + y1 / ((x1 - x0)*(x1 - x2))
	//        + y2 / ((x2 - x0)*(x2 - x1))
	//        ) * ((x - x0)*(x - x1))
	// or simplified as:
	let p2 = y0 + (y1 - y0) * n + (y2 + y0) * n * (n - 1) / 2;

	// lagrange poly
	// let f0 = ps[1] as f32;
	// let f1 = ps[2] as f32;
	// let f2 = ps[3] as f32;
	// let a  = (f0 / 2.0 - f1 + f2 / 2.0).floor() as i64;
	// let b  = (-3.0 * f0 / 2.0 + 2.0 * f1 - f2 / 2.0).floor() as i64;
	// let c  = f0 as i64;
	// let p2 = a*n*n + b*n + c;

	(ps[0], p2)
}
