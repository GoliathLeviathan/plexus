//! This module handles the in-game-time for the game. Since the user can speed up or slow down game time, there must be something to keep track of time.




//=============================================================================
// Crates


use chrono::Duration;
use chrono::naive::{NaiveTime, NaiveDateTime};




//=============================================================================
// Events


pub struct TimeStepEvent;




//=============================================================================
// Components


/// The clock holding the actual in-game time.
pub struct Clock{
	/// The in-game date and time.
	pub datetime: NaiveDateTime,

	/// The speed of the clock. This is a foctor between the real-world time and the in game time. For each second of real-world time, **speed** seconds pass in-game.
	pub speed: f32,
}

impl Clock {
	/// Advancing the in-game-time by a certain Duration.
	pub fn advance( &mut self, dur: Duration ) {
		self.datetime += dur;
	}
}


/// The usage schedule of the computer.
#[derive( Debug )]
pub struct ComputerSchedule {
// 	template: String,
	pub start: NaiveTime,
	pub stop: NaiveTime,
	pub load: f32,
}

impl ComputerSchedule {
	/// Create a new computer schedule from the template.
	pub fn from_template( template: &str ) -> Self {
		ComputerSchedule {
// 			template: template,
			start: NaiveTime::from_hms( 14, 33, 30 ),
			stop: NaiveTime::from_hms( 14, 34, 40 ),
			load: 0.5,
		}
	}

	/// Returns the current load (by the user) of the computer at the specified time.
	pub fn load( &self, time: NaiveTime ) -> f32 {
		if time >= self.start && time <= self.stop {
			return self.load;
		} else {
			return 0.0;
		}
	}
}
