//! This module handles the in-game-time for the game. Since the user can speed up or slow down game time, there must be something to keep track of time.




//=============================================================================
// Crates


use chrono::Duration;
use chrono::naive::{NaiveTime, NaiveDateTime};
use bevy::prelude::Component;

use crate::computer::Consumer;




//=============================================================================
// Events


// pub struct TimeStepEvent;




//=============================================================================
// Components


/// The clock holding the actual in-game time.
#[derive( Component )]
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
#[derive( Debug, Component )]
pub struct ComputerSchedule {
// 	template: String,
	pub start: Vec<NaiveTime>,
	pub duration: Duration,
	pub load: u32,
}

impl ComputerSchedule {
	/// Create a new computer schedule from the template.
	pub fn from_template( template: &str ) -> Self {
		// TODO: Ensure that the computer is on for at least 2 minutes to allow for enough time for booting and shutting down.
		ComputerSchedule {
			start: vec![
				NaiveTime::from_hms( 14, 32, 30 ),
				NaiveTime::from_hms( 14, 42, 40 ),
				NaiveTime::from_hms( 14, 52, 50 ),
				NaiveTime::from_hms( 15, 03, 00 ),
				NaiveTime::from_hms( 15, 13, 10 ),
				NaiveTime::from_hms( 15, 23, 20 ),
			],
			duration: Duration::minutes( 5 ),
			load: 500,
		}
	}

	/// If the computer is currently on, return the time it was started and will stop. Otherwise return `None`.
	fn start_stop( &self, time: NaiveTime ) -> Option<( NaiveTime, NaiveTime )> {
		for start in &self.start {
			let stop = *start + self.duration;
			if time >= *start && time <= stop {
				return Some( ( *start, stop ) );
			}
		}
		return None;
	}

	/// If the computer is on at the time provided, this returns `true` otherwise `false`.
	pub fn is_on( &self, time: NaiveTime ) -> bool {
		return self.start_stop( time ).is_some();
	}

	/// Returns the current discrete load of the computer at the specified time.
	fn load_discrete( &self, consumer: &Consumer, time: NaiveTime ) -> Result<u32, &str> {
		let ( start, stop ) = match self.start_stop( time ) {
			Some( x ) => x,
			None => return Ok( 0 )
		};

		match consumer {
			Consumer::System => {
				// The system needs some time to boot up. During this time the system load is high and gets lower at the end.
				if time < start + Duration::seconds( 45 ) {
					// First part of the booting process.
					return Ok( 900 );
				} else if time < start + Duration::seconds( 90 ) {
					// Second part of the booting process.
					return Ok( 750 );
				} else if time > stop - Duration::seconds( 30 ) {
					// Shutting down.
					return Ok( 750 );
				} else {
					// Normal work.
					return Ok( 100 );
				}
			},
			Consumer::User => {
				// Only after the boot time is done, the user is taking its load. Near the end of the usage time, the user has almost no load.
				if time >= start + Duration::seconds( 90 ) && time <= stop - Duration::seconds( 30 ) {
					return Ok( self.load );
				} else {
					return Ok( 0 );
				}
			},
			_ => return Err( "Consumer not legal" ),
		}
	}

	/// Returns the current load of the computer at the specified time. This value takes into account, that the load change is not infinitessimal fast but changes over time.
	pub fn load( &self, consumer: &Consumer, time: NaiveTime ) -> Result<u32, &str> {
		let current = self.load_discrete( consumer, time )?;

		match self.start_stop( time ) {
			Some( x ) => {
				let start = x.0;
				let prev = i64::from( self.load_discrete( consumer, start - Duration::milliseconds( 1 ) )? );
				let curr = i64::from( current );
				let dur = time - start;
				if dur > Duration::seconds( 0 ) && dur < Duration::seconds( 1 ) {
					let factor: f64 = dur.num_milliseconds() as f64 / 1000.0;
					let act: f64 = prev as f64 - ( ( prev - curr ) as f64 * factor );
					return Ok( act as u32 );
				} else {
					return Ok( current );
				}
			},
			None => return Ok( current ),
		}
	}
}
