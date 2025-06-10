use crate::genetic::*;
use itertools::Itertools;
use rayon::prelude::*;


pub fn naif(inst : &Instance, par : bool) -> Indiv {
	if par {
		foo_par(inst)
	}
	else {
		foo(inst)
	}
}

fn foo(inst : &Instance) -> Indiv {
	Itertools::permutations(0..inst.candidates, inst.candidates)
		.map(|ranking| Indiv::new(ranking, inst))
		.min_by_key(|x| x.score)
		.unwrap()
}

fn foo_par(inst : &Instance) -> Indiv {
	Itertools::permutations(0..inst.candidates, inst.candidates)
		.map(|ranking| Indiv::new(ranking, inst))
		.min_by_key(|x| x.score)
		.unwrap()
}