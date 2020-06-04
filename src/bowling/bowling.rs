
pub struct Frame {
	pub score_1:i32,
	pub score_2:Option<i32>
}

pub struct Game {
	pub frames : Vec<Frame>,
}

pub fn calculate_score(game:&Game) -> i32{
	game.frames.iter().fold(0, |sum, val| sum + val.score_1)
}

pub fn frame_points(frame:&Frame) -> i32{
	match frame.score_2 {
		Some(v) => frame.score_1 + v,
		None => frame.score_1
	}
}

#[cfg(test)]
mod tests{
	use crate::bowling::bowling::{calculate_score,Frame,Game, frame_points};

	#[test]
	fn test_calculate_score() {
		let testvec : Vec<Frame> = vec![
			Frame {
				score_1:10,
				score_2:None
			},
			Frame {
				score_1: 5,
				score_2:None
			}
		];
		let game = Game{
			frames: testvec,
		};
		assert_eq!(15, calculate_score(&game));
		assert_eq!(true, game.frames.iter().all(|x| frame_points(&x) <= 10 ));
	}
}