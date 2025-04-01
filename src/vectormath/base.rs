pub fn ele_mul(num: f64, vec: [f64; 3]) -> [f64; 3] {
	let mut res = [0.0; 3];

	for i in 0..vec.len() {
		res[i] = num * vec[i];
	}

	res
}

pub fn elementwise_addition(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
	assert!(a.len() == b.len());

	let mut outvec = a;
	for i in 0..b.len() {
		outvec[i] += b[i];
	}

	outvec
}

pub fn vec_sum(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
	assert!(a.len() == b.len());

	let mut outvec = a;
	for i in 0..b.len() {
		outvec[i] += b[i];
	}

	outvec
}

pub fn vec_avg(a: [f64; 4], b: [f64; 4]) -> [f64; 4] {
	assert!(a.len() == b.len());

	let mut outvec = a;
	for i in 0..b.len() {
		outvec[i] += b[i];
	}

	outvec
}