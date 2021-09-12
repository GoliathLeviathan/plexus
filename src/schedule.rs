//! This module handles the in-game-time for the game. Since the user can speed up or slow down game time, there must be something to keep track of time.




//=============================================================================
// Crates


use chrono::Duration;
use chrono::naive::{NaiveTime, NaiveDateTime};

use crate::computer::Consumer;




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
		// TODO: Ensure that the computer is on for at least 2 minutes to allow for enough time for booting and shutting down.
		ComputerSchedule {
// 			template: template,
			start: NaiveTime::from_hms( 14, 32, 30 ),
			stop: NaiveTime::from_hms( 14, 36, 40 ),
			load: 0.5,
		}
	}

	/// Returns the current load (by the user) of the computer at the specified time.
	pub fn load( &self, consumer: &Consumer, time: NaiveTime ) -> Result<f32, &str> {
		match consumer {
			Consumer::System => {
				if time >= self.start && time <= self.stop {
					// The system needs some time to boot up. During this time the system load is high and gets lower at the end.
					if time < self.start + Duration::seconds( 45 ) {
						// First part of the booting process.
						return Ok( 0.9 );
					} else if time < self.start + Duration::seconds( 90 ) {
						// Second part of the booting process.
						return Ok( 0.75 );
					} else if time > self.stop - Duration::seconds( 30 ) {
						// Shutting down.
						return Ok( 0.75 );
					} else {
						// Normal work.
						return Ok( 0.1 );
					}
				} else {
					return Ok( 0.0 );
				}
			},
			Consumer::User => {
				// Only after the boot time is done, the user is taking its load. Near the end of the usage time, the user has almost no load.
				if time >= self.start + Duration::seconds( 90 ) && time <= self.stop - Duration::seconds( 30 ) {
					return Ok( self.load );
				} else {
					return Ok( 0.0 );
				}
			},
			_ => return Err( "Consumer not legal" ),
		}
	}
}
