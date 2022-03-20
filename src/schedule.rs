//! This module handles the in-game-time for the game. Since the user can speed up or slow down game time, there must be something to keep track of time.




//=============================================================================
// Crates


use std::collections::HashMap;
use std::fmt;

use chrono::Duration;
use chrono::naive::{NaiveTime, NaiveDateTime};
use bevy::prelude::*;
use bevy::prelude::Component;

use crate::computer::Consumer;




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
pub enum MachineState {
	Off,
	Booting,
	Ready,
	ShuttingDown,
// 	Panic,
// 	Destroyed,
}

impl fmt::Display for MachineState {
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
		match *self {
			MachineState::Off => write!( f, "Off" ),
			MachineState::Booting => write!( f, "Booting" ),
			MachineState::Ready => write!( f, "Ready" ),
			MachineState::ShuttingDown => write!( f, "Shutting down" ),
// 			MachineState::Panic => write!( f, "Unrecoverable" ),
// 			MachineState::Destroyed => write!( f, ")/#(?§»…/($!~" ),
		}
	}
}


/// The hardware capabilities.
#[derive( Debug, Component )]
pub struct Hardware {
	/// The capability of the CPU. The higher the number, the better is the CPU.
	pub cpu: u32,
	pub state: MachineState,
	load: HashMap<Consumer, u32>,
}

impl Hardware {
	/// Return the total load on the hardware.
	fn load_total( &self ) -> u32 {
		let mut total = 0;
		for ( _, load ) in &self.load {
			total += load;
		}
		return total;
	}

	/// Return the load the given consumer is putting on the hardware.
	pub fn get_load( &self, consumer: &Consumer ) -> u32 {
		return *self.load.get( consumer ).unwrap();
	}

	/// Set the load the given consumer is putting on the hardware. Since the total load of all consumer can never exceed the capacity, some consumer's load is reduced.
	pub fn set_load( &mut self, consumer: &Consumer, val: u32 ) {
		self.load.insert( consumer.clone(), val );

		for cons in [ Consumer::Player, Consumer::User, Consumer::Enemy, ] {
			let spill = i64::from( self.load_total() ) - i64::from( self.cpu );
			if spill > 0 {
				let load = i64::from( self.get_load( &cons ) );
				let load_new = if spill < load { load - spill } else { 0 };
				self.load.insert( cons, load_new as u32 );
			} else {
				break;
			}
		}

		// If the system still has more load than capacity, it crashes.
		let spill = i64::from( self.load_total() ) - i64::from( self.cpu );
		if spill > 0 {
			self.crash();
		}
	}

	/// Crash the hardware.
	fn crash( &mut self ) {
		for ( _, load ) in self.load.iter_mut() {
			*load = 0;
		}
		self.state = MachineState::Booting;
	}
}


/// The usage schedule of the computer.
#[derive( Debug, Component )]
pub struct ComputerSchedule {
	pub start: Vec<NaiveTime>,
	pub duration: Duration,
}

impl ComputerSchedule {
	/// Create a new computer schedule from the template.
	pub fn new() -> Self {
		// TODO: Ensure that the computer is on for at least 2 minutes to allow for enough time for booting and shutting down.
		ComputerSchedule {
			start: vec![
				NaiveTime::from_hms( 14, 32, 05 ),
				NaiveTime::from_hms( 14, 42, 00 ),
				NaiveTime::from_hms( 14, 52, 50 ),
				NaiveTime::from_hms( 15, 03, 00 ),
				NaiveTime::from_hms( 15, 13, 10 ),
				NaiveTime::from_hms( 15, 23, 20 ),
			],
			duration: Duration::minutes( 1 ),
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

	/// Provide the scheduled state of the computer.
	pub fn state( &self, time: NaiveTime ) -> MachineState {
		let ( start, stop ) = match self.start_stop( time ) {
			Some( x ) => x,
			None => return MachineState::Off,
		};

		if time < start + Duration::seconds( 10 ) {
			return MachineState::Booting;
		} else if time > stop - Duration::seconds( 10 ) {
			return MachineState::ShuttingDown;
		} else {
			return MachineState::Ready;
		}
	}

	/// Returns the current discrete load of the computer at the specified time.
	pub fn load( &self, consumer: &Consumer, time: NaiveTime ) -> Result<u32, &str> {
		if self.start_stop( time ).is_none() {
			return Ok( 0 );
		};

		match consumer {
			Consumer::System => {
				match self.state( time ) {
					MachineState::Booting => return Ok( 900 ),
					MachineState::ShuttingDown => Ok( 750 ),
					MachineState::Ready => return Ok( 100 ),
					_ => return Ok( 0 ),
				}
			},
			Consumer::User => {
				match self.state( time ) {
					MachineState::Ready => return Ok( 500 ),
					_ => return Ok( 0 ),
				}
			},
			_ => return Err( "Consumer not legal" ),
		}
	}
}




//=============================================================================
// Systems


pub fn spawn_hardware(
	mut commands: Commands,
) {
	commands
		.spawn()
		.insert( Hardware {
			cpu: 1000,
			load: HashMap::from( [
				( Consumer::System, 0 ),
				( Consumer::User, 0 ),
				( Consumer::Player, 0 ),
				( Consumer::Enemy, 0 ),
			] ),
			state: MachineState::Off,
		} );
}
