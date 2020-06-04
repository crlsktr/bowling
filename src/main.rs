pub mod bowling;
use crate::bowling::bowling::Frame;
use crate::bowling::bowling::Game;
use bowling::calculate_score;



fn main() {
	let game = Game {
		frames: vec![
			Frame {
				score_1:10,
				score_2:Some(0),
			}
		]
	};
	println!("score {}", calculate_score(&game));
}