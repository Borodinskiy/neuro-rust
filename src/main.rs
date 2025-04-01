mod vectormath;
mod neuro;

use neuro::{matchstat, multistat, winchance};

fn main() {
	let weights_wc = [ 0.1, 0.2, 0.0 ];

	let weights_am = [ 0.3, 0.2, 0.9 ];

	let weights_ml = [
		//# games % win # cheerleaders
		[  0.1,    0.1, -0.3 ], // traumas?
		[  0.1,    0.2,  0.0 ], // win?
		[  0.0,    1.3,  0.1 ]  // sadness?
	];

	let toes    = [ 8.5, 9.5, 9.9, 9.0 ];  // Num games
	let wlrec   = [ 0.65, 0.8, 0.8, 0.9 ]; // % Win
	let nfans   = [ 1.2, 1.3, 0.5, 1.0 ];  // Cool dudes count

	let wc = winchance([ toes[0], wlrec[0], nfans[0] ], weights_wc);
	let am = matchstat(wlrec[0], weights_am);
	let ml = multistat([ toes[0], wlrec[0], nfans[0] ], weights_ml);

	println!("Prediction for winchance is {}!", wc);
	println!("Prediction for aftermatch statistic:\nTraumas: {}\nWin chance: {}\nSadness chance: {}", am[0], am[1], am[2]);
	println!("Prediction for multistat is:\nTraumas: {}\nWin chance: {}\nSadness chance: {}", ml[0], ml[1], ml[2]);
}
