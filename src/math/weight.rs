pub fn w_sum(a: [f64; 3], b: [f64; 3]) -> f64 {
	assert!(a.len() == b.len());

	let mut output = 0.0;
	for i in 0..a.len() {
		output += a[i] * b[i];
	}

	output
}

pub fn vec_mat_mul(vec: [f64; 3], matrix: [[f64; 3]; 3]) -> [f64; 3] {
	assert!(vec.len() == matrix.len());

	let mut res = [0.0; 3];

	for i in 0..vec.len() {
		res[i] = w_sum(vec, matrix[i]);
	}

	res
}