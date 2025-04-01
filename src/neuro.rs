use crate::vectormath::weight::{w_sum, vec_mat_mul};
use crate::vectormath::base::ele_mul;

pub fn winchance(input: [f64; 3], weights: [f64; 3]) -> f64 {
	w_sum(input, weights)
}

pub fn matchstat(input: f64, weights: [f64; 3]) -> [f64; 3] {
	ele_mul(input, weights)
}

pub fn multistat(input: [f64; 3], weights: [[f64; 3]; 3]) -> [f64; 3] {
	vec_mat_mul(input, weights)
}