#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]

use std::io::{self, Read};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input)?;

	let rays = parse(&input);
	let p1 = part1(&rays, 200_000_000_000_000.0, 400_000_000_000_000.0);
	println!("p1 = {p1}");
	let p2 = part2(&rays);
	println!("p2 = {p2}");

	Ok(())
}

fn part1(hails: &[[[f64; 3]; 2]], min: f64, max: f64) -> i32 {
	let mut count = 0;
	for i in 0..hails.len() {
		let [[x0, y0, _], [dx0, dy0, _]] = hails[i];
		for &[[x1, y1, _], [dx1, dy1, _]] in hails.iter().skip(i + 1) {
			let den = dx1 * dy0 - dy1 * dx0;
			if den == 0.0 { continue }

            let u = ((y1 - y0) * dx1 - (x1 - x0) * dy1) / den;
            let v = ((y1 - y0) * dx0 - (x1 - x0) * dy0) / den;

			if u < 0.0 || v < 0.0 { continue }

			let xi = x1 + dx1 * v;
			let yi = y1 + dy1 * v;

			if xi >= min && xi <= max && yi >= min && yi <= max {
				count += 1;
			}
		}
	}
	count
}

fn part2(hails: &[[[f64; 3]; 2]]) -> i64 {
	// rock is at P having velocity V
	// hail is H_i(p_i, v_i)
	//
	// we need to find an integer values for P & V,
	// such that there are integer positive t_i:
	// p1 + t1*v1 = P + t1*V
	// p2 + t2*v2 = P + t2*V
	// ...
	// p_n + t_n*v_n = P + t_n*V
	//
	// P + t_i * V = p_i + t_i * v_i
	// (P - p_i) = t_i * (v_i - V)
	// cross-product of both sides with (V - v_i):
	// (P - p_i) x (V - v_i) == t_i * (v_i - V) x (V - v_i) == 0
	// equals to 0 as (P - p_i) and (V - v_i) are parallel
	//
	// (P - p_i) x (v_i - V) = 0 (1)
	// this is a bilinear system in P and V.
	// but as P x V is common for every i, those can be equated
	// for 2 pairs of different indices giving 6 linear equations
	// for P and V.

	let h0 = hails[0];
	let h1 = hails[1];
	let h2 = hails[2];

	let v = [
        (h0[0][1] * h0[1][0] - h1[0][1] * h1[1][0]) - (h0[0][0] * h0[1][1] - h1[0][0] * h1[1][1]),
        (h0[0][1] * h0[1][0] - h2[0][1] * h2[1][0]) - (h0[0][0] * h0[1][1] - h2[0][0] * h2[1][1]),
        (h0[0][2] * h0[1][0] - h1[0][2] * h1[1][0]) - (h0[0][0] * h0[1][2] - h1[0][0] * h1[1][2]),
        (h0[0][2] * h0[1][0] - h2[0][2] * h2[1][0]) - (h0[0][0] * h0[1][2] - h2[0][0] * h2[1][2]),
        (h0[0][2] * h0[1][1] - h1[0][2] * h1[1][1]) - (h0[0][1] * h0[1][2] - h1[0][1] * h1[1][2]),
        (h0[0][2] * h0[1][1] - h2[0][2] * h2[1][1]) - (h0[0][1] * h0[1][2] - h2[0][1] * h2[1][2]),
	];

	let mut m = [
		[h1[1][1] - h0[1][1], h0[1][0] - h1[1][0], 0.0, h0[0][1] - h1[0][1], h1[0][0] - h0[0][0], 0.0, v[0]],
		[h2[1][1] - h0[1][1], h0[1][0] - h2[1][0], 0.0, h0[0][1] - h2[0][1], h2[0][0] - h0[0][0], 0.0, v[1]],
		[h1[1][2] - h0[1][2], 0.0, h0[1][0] - h1[1][0], h0[0][2] - h1[0][2], 0.0, h1[0][0] - h0[0][0], v[2]],
		[h2[1][2] - h0[1][2], 0.0, h0[1][0] - h2[1][0], h0[0][2] - h2[0][2], 0.0, h2[0][0] - h0[0][0], v[3]],
		[0.0, h1[1][2] - h0[1][2], h0[1][1] - h1[1][1], 0.0, h0[0][2] - h1[0][2], h1[0][1] - h0[0][1], v[4]],
		[0.0, h2[1][2] - h0[1][2], h0[1][1] - h2[1][1], 0.0, h0[0][2] - h2[0][2], h2[0][1] - h0[0][1], v[5]],
	];

	gauss_elimination(&mut m);

	let rp = [m[0][6] as i64, m[1][6] as i64, m[2][6] as i64];
	let rv = [m[3][6] as i64, m[4][6] as i64, m[5][6] as i64];

	let hp = [h0[0][0] as i64, h0[0][1] as i64, h0[0][2] as i64];
	let hv = [h0[1][0] as i64, h0[1][1] as i64, h0[1][2] as i64];

	// Gauss elimination has division, which results in
	// precision loss. As we know that solution is integer,
	// we just adjust the solution, while checking for a
	// collision.
	for dx in [-1, 0, 1] {
		for dy in [-1, 0, 1] {
			for dz in [-1, 0, 1] {
				let d  = [dx, dy, dz];
				let pd = sub(sub(hp, rp), d);
				let vd = sub(hv, rv);
				let cm = cross_matrix(pd);
				let p  = matrix_vec_mul(&cm, vd);

				if p[0] == 0 && p[1] == 0 && p[2] == 0 {
					return rp[0] + rp[1] + rp[2] + dx + dy + dz;
				}
			}
		}
	}

	0
}

fn sub(a: [i64; 3], b: [i64; 3]) -> [i64; 3] {
	[a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}

fn cross_matrix(v: [i64; 3]) -> [[i64; 3]; 3] {
	[[    0, -v[2],  v[1]],
	 [ v[2],    0,  -v[0]],
	 [-v[1],  v[0],     0]]
}

fn matrix_vec_mul(m: &[[i64; 3]; 3], v: [i64; 3]) -> [i64; 3] {
	[
		m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
		m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
		m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
	]
}

fn gauss_elimination(m: &mut [[f64; 7]; 6]) {
    let n = m.len();
    for i in 0..n {
        let mut max_row = i;
        for j in i + 1..n {
            if m[j][i].abs() > m[max_row][i].abs() {
                max_row = j;
            }
        }

        for k in i..n + 1 {
            let tmp = m[max_row][k];
            m[max_row][k] = m[i][k];
            m[i][k] = tmp;
        }

        for j in i + 1..n {
            let c = m[j][i] / m[i][i];
            for k in i..n + 1 {
                if i == k {
                    m[j][k] = 0.0;
                } else {
                    m[j][k] -= c * m[i][k];
                }
            }
        }
    }

    for i in (0..n).rev() {
        m[i][n] /= m[i][i];
        m[i][i] = 1.0;
        for j in 0..i {
            m[j][n] -= m[j][i] * m[i][n];
            m[j][i] = 0.0;
        }
    }
}

fn parse(input: &str) -> Vec<[[f64; 3]; 2]> {
	input
		.lines()
		.flat_map(|s| {
			s
				.split([',', '@'])
				.filter_map(|s| s.trim().parse::<f64>().ok())
				.array_chunks::<3>()
				.next_chunk::<2>()
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	const INPUT: &str = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"#;

	#[test]
	fn test_part1() {
		assert_eq!(part1(&parse(INPUT), 7.0, 27.0), 2);
	}

	#[test]
	fn test_part2() {
		assert_eq!(part2(&parse(INPUT)), 47);
	}
}
