use std::{collections::VecDeque, fmt};

fn flip() -> bool {
	rand::random()
}

const H: bool = true;
const T: bool = false;

fn main() {
	let patterns: &[&[bool]] = &[&[H, H], &[H, T]];
	let rounds = 100;
	let games = 100000;

	let mut overall_scores = vec![0; patterns.len()];
	for _ in 0..games {
		let mut scores = vec![0; patterns.len()];
		let max_pattern_length = patterns.iter().map(|p| p.len()).max().unwrap();
		let mut sequence = VecDeque::<bool>::with_capacity(max_pattern_length);
		for _ in 0..rounds {
			if sequence.len() == max_pattern_length {
				sequence.pop_front();
			}
			sequence.push_back(flip());
			for p in 0..patterns.len() {
				if sequence.len() >= patterns[p].len()
					&& sequence
						.range(sequence.len() - patterns[p].len()..)
						.eq(patterns[p])
				{
					scores[p] += 1;
				}
			}
		}
		let (max_idx, max_elem) = scores
			.iter()
			.enumerate()
			.max_by(|a, b| a.1.cmp(b.1))
			.unwrap();
		if scores.iter().filter(|&s| s == max_elem).count() == 1 {
			overall_scores[max_idx] += 1;
		}
	}
	println!("Results:");
	for p in 0..patterns.len() {
		let percentage = f64::from(overall_scores[p]) / f64::from(games) * 100.0;
		println!(
			"{} ({}) -> {:.2}%",
			Name(p),
			CoinSequence(patterns[p]),
			percentage
		);
	}
	println!(
		"Draws -> {:.2}%",
		f64::from(games - overall_scores.iter().sum::<u32>()) / f64::from(games) * 100.0
	);
}

struct Name(usize);

impl fmt::Display for Name {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let name = match self.0 {
			0 => "Alice",
			1 => "Bob",
			2 => "Carol",
			3 => "Daisy",
			4 => "Emily",
			5 => "Faith",
			6 => "Grace",
			7 => "Heidi",
			8 => "Isla",
			9 => "Judy",
			10 => "Kevin",
			11 => "Lucy",
			12 => "Mallory",
			13 => "Nancy",
			14 => "Olivia",
			15 => "Patricia",
			16 => "Quentin",
			17 => "Rose",
			18 => "Sybil",
			19 => "Thalia",
			20 => "Ursula",
			21 => "Victoria",
			22 => "Willow",
			23 => "Xavier",
			24 => "Yvonne",
			25 => "Zoe",
			_ => return write!(f, "Player {}", self.0 + 1),
		};
		write!(f, "{name}")
	}
}

struct CoinSequence<'a>(&'a [bool]);

impl<'a> fmt::Display for CoinSequence<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for ch in self.0 {
			write!(f, "{}", if *ch { "H" } else { "T" })?;
		}
		Ok(())
	}
}
