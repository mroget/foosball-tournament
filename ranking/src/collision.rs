use std::collections::VecDeque;
use std::collections::HashMap;

fn rename(mut l1 : VecDeque<usize>, mut l2 : VecDeque<usize>) -> VecDeque<usize> {
	let mut dic = HashMap::<usize, usize>::new();
	let mut l = VecDeque::with_capacity(l2.len());

	let mut i = 0;
	while l1.len() > 0 {
		dic.insert(l1.pop_front().unwrap(), i);
		i += 1;
	}

	while l2.len() > 0 {
		let k = l2.pop_front().unwrap();
		if dic.contains_key(&k) {
			l.push_back(*dic.get(&k).unwrap());
		}
	}

	l
}

fn merge(mut l1 : VecDeque<usize>, mut l2 : VecDeque<usize>) -> (VecDeque<usize>, u64) {
	let mut k = 0;
	let mut l = VecDeque::with_capacity(l1.len() + l2.len());

	while l1.len() > 0 && l2.len() > 0 {
		if l1.front().unwrap() <= l2.front().unwrap() {
			l.push_back(l1.pop_front().unwrap());
		}
		else {
			l.push_back(l2.pop_front().unwrap());
			k += l1.len();
		}
	}

	while l1.len() > 0 {
		l.push_back(l1.pop_front().unwrap());
	}

	while l2.len() > 0 {
		l.push_back(l2.pop_front().unwrap());
	}

	(l,k as u64)
}

fn modified_merge_sort(mut l2 : VecDeque<usize>) -> (VecDeque<usize>, u64) {
	match l2.len() {
		0 => {(l2, 0)},
		1 => {(l2, 0)},
		n => {
			let mut l1 = VecDeque::with_capacity(n/2);
			for _i in 0..n/2 {
				l1.push_back(l2.pop_front().unwrap());
			}
			let (l1, k1) = modified_merge_sort(l1);
			let (l2, k2) = modified_merge_sort(l2);
			let (l3, k3) = merge(l1,l2);
			(l3, k1+k2+k3)
		}
	}
}

fn modified_merge_sort_lin(mut l : VecDeque<usize>) -> (VecDeque<usize>, u64) {
	let mut tab = VecDeque::with_capacity(l.len());
	let mut count = 0;

	while l.len() > 0 {
		let mut tmp = VecDeque::new();
		tmp.push_back(l.pop_front().unwrap());
		tab.push_back(tmp);
	}

	while tab.len() > 1 {
		let mut tmp = VecDeque::with_capacity(tab.len());
		while tab.len() >= 2 {
			let l1 = tab.pop_front().unwrap();
			let l2 = tab.pop_front().unwrap();
			let (q, k) = merge(l1, l2);
			tmp.push_back(q);
			count += k
		}
		tab = tmp;
	}

	(tab[0].clone(), count)
}

pub fn collision(l1 : &Vec<usize>, l2 : &Vec<usize>, opt : bool) -> u64 {
	let q1 = VecDeque::from(l1.clone());
	let q2 = VecDeque::from(l2.clone());
	let l = if q1.len() >= q2.len() {rename(q1,q2)} else {rename(q2,q1)};
	if l.len() == 0 {
		return 0;
	}

	let (_, k) =  
		if opt {
			modified_merge_sort_lin(l)
		}
		else {
			modified_merge_sort(l)
		};
	
	k
}
