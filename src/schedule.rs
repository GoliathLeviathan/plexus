//! This module handles the in-game-time for the game. Since the user can speed up or slow down game time, there must be something to keep track of time.




//=============================================================================
// Crates


use bevy::prelude::*;
use bevy::core::Timer;




//=============================================================================
// Structs


#[derive( Bundle )]
pub struct Tracker {
	pub speed: f32,
	pub timer: Timer,
}

impl Tracker {
	/// Create a new instance of the tracker.
	pub fn new() -> Tracker {
		return Tracker{
			speed: 1.0,

			// Creating a repeating timer that shoots once per second (normal time flow).
			timer: Timer::from_seconds( 1.0, true ),
		};
	}
}
