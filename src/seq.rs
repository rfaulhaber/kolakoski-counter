pub struct KolSeq {
	prev_run: Run,
	seq: Vec<u8>,
	step: usize,
}

impl KolSeq {
	pub fn new() -> KolSeq {
		KolSeq {
			prev_run: Run::Single(0),
			step: 0,
			seq: Vec::new(),
		}
	}

	pub fn is_step_even(&self) -> bool {
		self.step % 2 == 0
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Run {
	Single(u8),
	Double(u8, u8),
}

impl Run {
	fn first(&self) -> u8 {
		match self {
			Run::Single(first) => *first,
			Run::Double(first, _) => *first,
		}
	}

	fn append_to(&self, vec: &mut Vec<u8>) {
		match self {
			Run::Single(first) => vec.push(*first),
			Run::Double(first, second) => {
				vec.push(*first);
				vec.push(*second);
			}
		};
	}
}

impl Iterator for KolSeq {
	type Item = Run;

	fn next(&mut self) -> Option<Self::Item> {
		let next_step = match self.step {
			0 => Run::Single(1),
			1 => Run::Double(2, 2),
			_ => {
				let prev_val = self.seq[self.step];

				if !self.is_step_even() {
					if prev_val == 1 {
						Run::Single(2)
					} else {
						Run::Double(2, 2)
					}
				} else {
					if prev_val == 1 {
						Run::Single(1)
					} else {
						Run::Double(1, 1)
					}
				}
			}
		};

		next_step.append_to(&mut self.seq);

		self.step += 1;

		Some(next_step.clone())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn kol_seq_generates_n_steps() {
		let seq = KolSeq::new();

		let mut steps = seq.take(10);

		assert_eq!(steps.next(), Some(Run::Single(1)));
		assert_eq!(steps.next(), Some(Run::Double(2, 2)));
		assert_eq!(steps.next(), Some(Run::Double(1, 1)));
		assert_eq!(steps.next(), Some(Run::Single(2)));
		assert_eq!(steps.next(), Some(Run::Single(1)));
		assert_eq!(steps.next(), Some(Run::Double(2, 2)));
		assert_eq!(steps.next(), Some(Run::Single(1)));
		assert_eq!(steps.next(), Some(Run::Double(2, 2)));
		assert_eq!(steps.next(), Some(Run::Double(1, 1)));
		assert_eq!(steps.next(), Some(Run::Single(2)));
	}

	// #[test]
	// fn benchmark() {
	// 	let seq = KolSeq::new();

	// 	let mut steps = seq.take(100000000000000);

	// 	let mut ones = 0;
	// 	let mut twos = 0;

	// 	for step in steps {
	// 		match step {
	// 			Run::Single(val) => {
	// 				if val == 1 {
	// 					ones += 1;
	// 				} else {
	// 					twos += 1;
	// 				}
	// 			}
	// 			Run::Double(first, second) => {
	// 				if first == 1 {
	// 					ones += 1;
	// 				} else {
	// 					twos += 1;
	// 				}

	// 				if second == 1 {
	// 					ones += 1;
	// 				} else {
	// 					twos += 1;
	// 				}
	// 			}
	// 		}
	// 	}
	// }
}
