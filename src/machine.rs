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
// Enums


/// This enum represents the load on a machines resource.
#[derive( Debug )]
pub enum Load {
	Max,
	Exact( u32 ),
}




//=============================================================================
// Resources


/// A timer to record the time since the last update.
pub struct UpdateTimer {
	pub timer: Timer,
}


/// The clock holding the actual in-game time.
pub struct Clock{
	/// The in-game date and time.
	pub datetime: NaiveDateTime,

	/// The speed of the clock. This is a factor between the real-world time and the in game time. For each second of real-world time, **speed** seconds pass in-game.
	pub speed: f32,
}

impl Clock {
	/// Advancing the in-game-time by a certain Duration.
	pub fn advance( &mut self, dur: Duration ) {
		self.datetime += dur;
	}
}




//=============================================================================
// Components


/// The usage schedule of the computer.
#[derive( Debug )]
pub enum MachineState {
	Off,
	Booting,
	Ready,
	ShuttingDown,
// 	Panic,
// 	Destroyed,
}

impl MachineState {
	/// Returns the work required to complete the state. If the returned value is `None` the state cannot be completed.
	pub fn work( state: &MachineState ) -> Option<u32> {
		match *state {
			MachineState::Booting => return Some( 9000 ),
			MachineState::ShuttingDown => return Some( 5000 ),
			_ => return None,
		}
	}

	/// Returns the continuous system during different machine states. This is always present and added to the `work()`.
	pub fn load( state: &MachineState ) -> Load {
		match *state {
			MachineState::Booting | MachineState::ShuttingDown => return Load::Max,
			MachineState::Ready => return Load::Exact( 100 ),
			_ => return Load::Exact( 0 ),
		}
	}
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
pub struct Machine {
	/// The capability of the CPU. The higher the number, the better is the CPU.
	pub cpu: u32,
	pub state: MachineState,
	pub work_done: HashMap<Consumer, u32>,
	load: HashMap<Consumer, u32>,
	load_target: HashMap<Consumer, u32>,
}

impl Machine {
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

	/// Return the load the given consumer wants to put on the machine.
	pub fn get_load_target( &self, consumer: &Consumer ) -> u32 {
		return *self.load_target.get( consumer ).unwrap();
	}

	/// Set the load the given consumer wants to put on the machine. It is not guarantieed that it can do so. To read the load the consumer actually puts on the machine see `get_load()`.
	pub fn set_load_target( &mut self, consumer: &Consumer, val: u32 ) {
		self.load_target.insert( consumer.clone(), val );
		self.set_load( consumer, val );
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
pub struct MachineSchedule {
	pub start: Vec<NaiveTime>,
	pub duration: Duration,
}

impl MachineSchedule {
	/// Create a new computer schedule from the template.
	pub fn new() -> Self {
		// TODO: Ensure that the computer is on for at least 2 minutes to allow for enough time for booting and shutting down.
		MachineSchedule {
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

	/// If the computer is currently scheduled to be on, return the time it was scheduled to be started and scheduled to be shut down. Otherwise return `None`. At the stop time, the machine is scheduled to start the shut down process.
	fn start_stop( &self, time: NaiveTime ) -> Option<( NaiveTime, NaiveTime )> {
		for start in &self.start {
			let stop = *start + self.duration;
			if time >= *start && time <= stop {
				return Some( ( *start, stop ) );
			}
		}
		return None;
	}

	/// Returns `true` if the machine is scheduled to be on, otherwise returns `false`.
	pub fn is_on( &self, time: NaiveTime ) -> bool {
		return self.start_stop( time ).is_some();
	}
}




//=============================================================================
// Systems


pub fn spawn_machine(
	mut commands: Commands,
) {
	commands
		.spawn()
		.insert( Machine {
			cpu: 1000,
			load: HashMap::from( [
				( Consumer::System, 0 ),
				( Consumer::User, 0 ),
				( Consumer::Player, 0 ),
				( Consumer::Enemy, 0 ),
			] ),
			load_target: HashMap::from( [
				( Consumer::System, 0 ),
				( Consumer::User, 0 ),
				( Consumer::Player, 0 ),
				( Consumer::Enemy, 0 ),
			] ),
			work_done: HashMap::from( [
				( Consumer::System, 0 ),
			] ),
			state: MachineState::Off,
		} );
}
