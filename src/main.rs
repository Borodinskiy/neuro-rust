mod math;
mod neuro;

use math::base::pow;
use neuro::{matchstat, multistat, winchance};

fn main() {
	let mut weight = 0.5;
	let input = 0.5;
	let goal_prediction = 0.8;

	let step_amount = 0.001;

	for _ in 0..1101 {
		let prediction = input * weight;
		let error = pow(prediction - goal_prediction, 2);

		println!("Error: {} Prediction: {}", error, prediction);

		let up_prediction = input * (weight + step_amount);
		let up_error = pow(goal_prediction - up_prediction, 2);

		let down_prediction = input * (weight + step_amount);
		let down_error = pow(goal_prediction - down_prediction, 2);

		if down_error < up_error {
			weight = weight - step_amount;
		} else {
			weight = weight + step_amount;
		}
	}
}
