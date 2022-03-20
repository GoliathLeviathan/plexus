//! This module contains everything related to the computer.




//=============================================================================
// Crates


use std::cmp;

use rand::Rng;
use bevy::prelude::*;

use crate::materials::CustomColor;
use crate::schedule::{Load, Clock, MachineState, Machine, MachineSchedule};




//=============================================================================
// Constants


/// The size of the CPU sprite. The first entry is the width, the second the height.
const CPU_SIZE: [f32; 2] = [ 120.0, 120.0 ];


/// The margin between computer component representations and the info they display.
const MARGIN: f32 = 10.0;


/// The maximum size of the usage bars. The first entry is the width, the second the height.
const USAGE_BAR_SIZE: [f32; 2] = [ CPU_SIZE[0] - MARGIN, CPU_SIZE[1] - MARGIN ];




//=============================================================================
// Enums


#[derive( Debug, PartialEq, Eq, Hash, Clone, Component )]
pub enum Consumer {
	System,
	User,
	Player,
	Enemy,
}




//=============================================================================
// Components


/// This component represents a CPU of the Computer.
#[derive( Component )]
pub struct Cpu {
	/// The capacity represents the computers performance. The higher the [`capacity`] the more operations it can perform each time unit.
	capacity: u32,
}


/// This component represents an instrument of the CPU.
#[derive( Component )]
pub struct InstrumentCpu;


/// This component represents a status bar.
#[derive( Component )]
pub struct StatusBar;




//=============================================================================
// Systems


pub fn spawn_cpu(
	mut commands: Commands,
) {
	// Create CPU-block
	commands
		.spawn_bundle( SpriteBundle {
			transform: Transform::from_xyz( -140.0, 100.0, 0.0 ),
			sprite: Sprite {
				custom_size: Some( Vec2::new( CPU_SIZE[0], CPU_SIZE[1] ) ),
				color: CustomColor::COMPONENT,
				..Default::default()
			},
			..Default::default()
		} )
		.insert( Cpu {
			capacity: 1000,
		} )
		.with_children( |parent| {
			// Create CPU usage bars
			parent
				.spawn_bundle( SpriteBundle {
					transform: Transform::from_xyz( 0.0, -USAGE_BAR_SIZE[1] / 2.0, 1.0 ),
					sprite: Sprite {
						custom_size: Some( Vec2::new( USAGE_BAR_SIZE[0], USAGE_BAR_SIZE[1] ) ),
						color: CustomColor::SYSTEM,
						..Default::default()
					},
					..Default::default()
				} )
				.insert( InstrumentCpu )
				.insert( StatusBar )
				.insert( Consumer::System );
			parent
				.spawn_bundle( SpriteBundle {
					transform: Transform::from_xyz( 0.0, -USAGE_BAR_SIZE[1] / 2.0, 1.0 ),
					sprite: Sprite {
						custom_size: Some( Vec2::new( USAGE_BAR_SIZE[0], USAGE_BAR_SIZE[1] ) ),
						color: CustomColor::USER,
						..Default::default()
					},
					..Default::default()
				} )
				.insert( InstrumentCpu )
				.insert( StatusBar )
				.insert( Consumer::User );
			parent
				.spawn_bundle( SpriteBundle {
					transform: Transform::from_xyz( 0.0, -USAGE_BAR_SIZE[1] / 2.0, 1.0 ),
					sprite: Sprite {
						custom_size: Some( Vec2::new( USAGE_BAR_SIZE[0], USAGE_BAR_SIZE[1] ) ),
						color: CustomColor::ENEMY,
						..Default::default()
					},
					..Default::default()
				} )
				.insert( InstrumentCpu )
				.insert( StatusBar )
				.insert( Consumer::Enemy );
			parent
				.spawn_bundle( SpriteBundle {
					transform: Transform::from_xyz( 0.0, -USAGE_BAR_SIZE[1] / 2.0, 1.0 ),
					sprite: Sprite {
						custom_size: Some( Vec2::new( USAGE_BAR_SIZE[0], USAGE_BAR_SIZE[1] ) ),
						color: CustomColor::PLAYER,
						..Default::default()
					},
					..Default::default()
				} )
				.insert( InstrumentCpu )
				.insert( StatusBar )
				.insert( Consumer::Player );
		} );
}


/// Update the computer usage.
pub fn update_usage(
	query: Query<&Consumer>,
	clock_query: Query<&Clock>,
	schedule_query: Query<&MachineSchedule>,
	mut machine_query: Query<&mut Machine>,
) {
	let clock = clock_query.single();
	let schedule = schedule_query.single();
	let mut machine = machine_query.single_mut();

	match machine.state {
		MachineState::Off => {
			for consumer in query.iter() {
				machine.set_load( &consumer, 0 );
			}
			return ();
		},
		_ => (),
	}

	for consumer in query.iter() {
		let load_target = MachineState::load( &machine.state );

		let load_goal = match load_target {
			Load::Exact( 0 ) => 0,
			Load::Exact( x ) => {
				let mut load = machine.get_load( &consumer );
				let diff = i64::from( x ) - i64::from( load );
				let jump_quick = rand::thread_rng().gen_range( 1..32 );
				let jump_slow = rand::thread_rng().gen_range( 1..8 );
				if diff < -8 {
					load -= cmp::min( jump_quick, load );
				} else if diff < 0 {
					load -= cmp::min( jump_quick, load );
				} else if diff > 8 {
					load += jump_quick;
				} else {
					load += jump_slow;
				}
				load
			},
			Load::Max => machine.cpu - rand::thread_rng().gen_range( 1..32 ),
		};
		machine.set_load( &consumer, load_goal );

		match machine.state {
			 MachineState::Booting | MachineState::ShuttingDown => {
				match consumer {
					Consumer::System => {
						let work = ( f64::from( load_goal ) * 0.1 ) as u32;
						let done = machine.work_done.get( &consumer ).unwrap() + work;
						machine.work_done.insert( consumer.clone(), done );
					},
					_ => (),
				}
			}
			_ => (),
		}
	}
}


/// Switch between operational states.
pub fn update_state(
	mut machine_query: Query<&mut Machine>,
	clock_query: Query<&Clock>,
	schedule_query: Query<&MachineSchedule>,
) {
	let mut machine = machine_query.single_mut();
	let clock = clock_query.single();
	let schedule = schedule_query.single();

	match machine.state {
		MachineState::Off => {
			if schedule.is_on( clock.datetime.time() ) {
				machine.state = MachineState::Booting;
			}
		},
		MachineState::Booting => {
			match MachineState::work( &machine.state ) {
				Some( x ) => {
					if machine.work_done.get( &Consumer::System ).unwrap() >= &x {
						machine.state = MachineState::Ready;
						machine.work_done.insert( Consumer::System, 0 );
					}
				},
				None => (),
			}
		},
		MachineState::Ready => {
			if !schedule.is_on( clock.datetime.time() ) {
				machine.state = MachineState::ShuttingDown;
			}
		},
		MachineState::ShuttingDown => {
			match MachineState::work( &machine.state ) {
				Some( x ) => {
					if machine.work_done.get( &Consumer::System ).unwrap() >= &x {
						machine.state = MachineState::Off;
						machine.work_done.insert( Consumer::System, 0 );
					}
				},
				None => (),
			}
		},
	}
}


/// Update the usage display. This moves the current usage value slowly to the target usage value so that the change is smooth and is not jumping around.
pub fn draw_usage(
	mut query: Query<( &mut Transform, &Consumer ), With<InstrumentCpu>>,
	cpu_query: Query<&Cpu>,
	machine_query: Query<&Machine>,
) {
	let cpu = cpu_query.single();
	let hardware = machine_query.single();
	let mut transform_prev: Option<Mut<Transform>> = None;
	for ( mut transform, consumer ) in query.iter_mut() {
		let scale_target = hardware.get_load( &consumer ) as f32 / cpu.capacity as f32;
		transform.scale.y = scale_target;

		match transform_prev {
			Some( v ) => {
				let shift = v.translation.y + USAGE_BAR_SIZE[1] * v.scale.y / 2.0 + ( transform.scale.y / 2.0 ) * USAGE_BAR_SIZE[1];
				transform.translation = Vec3::new( 0.0, shift, 1.0 );
			},
			None => {
				let shift: f32 = -USAGE_BAR_SIZE[1] / 2.0 + ( transform.scale.y / 2.0 ) * USAGE_BAR_SIZE[1];
				transform.translation = Vec3::new( 0.0, shift, 1.0 );
			},
		}

		// Store handled transform as previously handled.
		transform_prev = Some( transform );
	}
}
