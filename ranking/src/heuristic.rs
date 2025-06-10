use crate::genetic::*;


pub fn heuristic(inst : &Instance) -> Indiv {
	let mut score = vec![0.; inst.candidates];

	for i in 0..inst.votes.len() {
		let w = inst.votes[i].len();
		let h = 1./(w as f64 +1.);
		for j in 0..w {
			score[inst.votes[i][j]] += h*(j as f64);
		}
	}

	let mut ranking = (0..inst.candidates).collect::<Vec<usize>>();
	ranking.sort_by(|a, b| score[*a].partial_cmp(&score[*b]).unwrap());
	Indiv::new(ranking, inst)
}