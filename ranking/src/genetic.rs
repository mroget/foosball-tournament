use crate::heuristic;
use std::time::Duration;
use std::time::Instant;
use std::fmt;
use crate::io;
use rand::Rng;
use crate::collision;

use rand::thread_rng;
use rand::seq::SliceRandom;

pub struct Instance {
	pub votes : Vec<Vec<usize>>,
	pub candidates : usize,
}
impl Instance {
	pub fn read() -> Instance {
		let mut buffer = String::new();
    	io::stdin().read_line(&mut buffer).expect("Failed to read line");
    	let c : usize = buffer.trim().parse().expect("parsing failed");
    	buffer = String::new();
    	io::stdin().read_line(&mut buffer).expect("Failed to read line");
    	let nb : usize = buffer.trim().parse().expect("parsing failed");
    	let mut votes = Vec::with_capacity(nb);
    	for _i in 0..nb {
    		buffer = String::new();
		    io::stdin().read_line(&mut buffer).expect("Failed to read line");
		    votes.push(buffer.trim().split(" ")
		    .map(|x| x.parse().expect("Not an integer!"))
		    .collect());
    	}
    	Instance {votes : votes, candidates : c}
	}
}

pub struct Indiv {
	pub ranking : Vec<usize>,
	pub score : u64,
}
impl fmt::Display for Indiv {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	let mut s = String::new();
    	s.push_str(&format!("{} :", self.score));
    	for i in 0..self.ranking.len() {
    		s.push_str(&format!(" {}", self.ranking[i]));
    	}
    	write!(f, "{}", s)
    }
}
impl Clone for Indiv {
	fn clone(&self) -> Self {
		Indiv {ranking : self.ranking.clone(), score : self.score}
	}
}
impl Indiv {
	pub fn new(ranking : Vec<usize>, inst : &Instance) -> Indiv {
		let score = eval(&ranking, inst);
		Indiv {ranking : ranking, score : score}
	}
}

fn eval(ranking : &Vec<usize>, inst : &Instance) -> u64 {
	let mut score = 0;
	for i in 0..inst.votes.len() {
		score += collision(ranking, &inst.votes[i], true);
	}
	score
}

fn random_indiv(inst : &Instance) -> Indiv {
	let mut vec: Vec<usize> = (0..inst.candidates).collect();
	vec.shuffle(&mut thread_rng());
	Indiv::new(vec, inst)
}

fn mutation(indiv : &Indiv, inst : &Instance) -> Indiv {
	let mut ret = indiv.ranking.clone();
	let mut rng = rand::thread_rng();
	let r1 = rng.gen_range(0..indiv.ranking.len());
	let r2 = rng.gen_range(0..indiv.ranking.len());	
	ret.swap(r1,r2);
	Indiv::new(ret, inst)
}

fn mutation_bis(indiv : &Indiv, inst : &Instance) -> Indiv {
	let mut ret = indiv.ranking.clone();
	let mut rng = rand::thread_rng();
	let r1 = rng.gen_range(0..indiv.ranking.len()-1);
	ret.swap(r1,r1+1);
	Indiv::new(ret, inst)
}

fn fusion(indiv1 : &Indiv, indiv2 : &Indiv, inst : &Instance) -> Indiv {
	let n = indiv1.ranking.len();
	let mut used = vec![false; n];
	let mut ret = Vec::with_capacity(n);
	let mut active = 0;
	let mut pos = vec![0,0];
	for _i in 0..n {
		if active==0 {
			while used[indiv1.ranking[pos[active]]] {
				pos[active] +=1;
			}	
			ret.push(indiv1.ranking[pos[active]]);
			used[indiv1.ranking[pos[active]]] = true;
		}
		else {
			while used[indiv2.ranking[pos[active]]] {
				pos[active] +=1;
			}	
			ret.push(indiv2.ranking[pos[active]]);
			used[indiv2.ranking[pos[active]]] = true;
		}
		pos[active] +=1;
		active = (active+1) % 2;
	}
	Indiv::new(ret, inst)
}

fn nextgen(population : &Vec<Indiv>, inst : &Instance, mutation_size : usize, fusion_size : usize, mut_bis:bool) -> Vec<Indiv> {
	let mut ret = Vec::with_capacity(mutation_size+fusion_size);
	let mut rng = rand::thread_rng();
	for _i in 0..mutation_size {
		let r = rng.gen_range(0..population.len());
		if mut_bis {
			ret.push(mutation_bis(&population[r], inst));
		}
		else {
			ret.push(mutation(&population[r], inst));
		}
		
	}

	for _i in 0..fusion_size {
		let r1 = rng.gen_range(0..population.len());
		let r2 = rng.gen_range(0..population.len());
		ret.push(fusion(&population[r1], &population[r2], inst));
	}

	ret
}

pub fn genetic(inst : &Instance, mutation_size : usize, fusion_size : usize, population_size : usize, time_limit : Duration, debug : bool, mutation_bis : bool) -> Indiv {
	let start = Instant::now();
	let mut population : Vec<Indiv> = (0..population_size).map(|_x| random_indiv(inst)).collect();

	if debug {
			println!("(1) {}", population[0]);
			println!("(2) {}", population[population.len()-1]);
		}

	let mut i = 0;
	while start.elapsed() < time_limit{
		let next = nextgen(&population, inst, mutation_size, fusion_size, mutation_bis);
		population.extend(next);
		population.sort_by(|a, b| a.score.partial_cmp(&b.score).unwrap());
		population.truncate(population_size);
		if debug {
			println!("{} => {}", i, population[0]);
		}
		i+=1;
	}

	population[0].clone()
}