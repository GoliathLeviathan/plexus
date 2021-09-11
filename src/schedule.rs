//! This module handles the in-game-time for the game. Since the user can speed up or slow down game time, there must be something to keep track of time.




//=============================================================================
// Crates


use chrono::naive::NaiveTime;
use bevy::prelude::*;
use bevy::core::Timer;




//=============================================================================
// Structs


pub struct TimeStepEvent;


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
