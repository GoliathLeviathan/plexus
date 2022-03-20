//! This module contains everything related to the computer.




//=============================================================================
// Crates


use std::cmp;

use rand::Rng;
use bevy::prelude::*;

use crate::materials::CustomColor;
use crate::schedule::{Clock, Hardware, ComputerSchedule};




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


/// This component represents a usage information.
#[derive( Debug, Component )]
pub struct Usage {
	/// The type of the consumer having this usage.
	pub consumer: Consumer,

	/// The load between 0 (no load at all) and 1 (full load).
	pub load: u32,
}


/// This is used by the player as a consumer.
#[derive( Component )]
pub struct ConsumerPlayer;


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
				.insert( Usage{
					consumer: Consumer::System,
					load: 0,
				} );
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
				.insert( Usage{
					consumer: Consumer::User,
					load: 0,
				} );
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
				.insert( Usage{
					consumer: Consumer::Enemy,
					load: 0,
				} );
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
				.insert( Usage{
					consumer: Consumer::Player,
					load: 0,
				} )
				.insert( ConsumerPlayer );
		} );
}


/// Update the computer usage.
pub fn update_usage(
	query: Query<&Usage>,
	clock_query: Query<&Clock>,
	schedule_query: Query<&ComputerSchedule>,
	mut hw_query: Query<&mut Hardware>,
) {
	let clock = clock_query.single();
	let schedule = schedule_query.single();
	let mut hardware = hw_query.single_mut();
	for usage in query.iter() {
		let load_target = match schedule.load( &usage.consumer, clock.datetime.time() ) {
			Ok( x ) => x,
			Err( _ ) => continue,
		};

		let mut load;
		if load_target == 0 {
			load = 0;
		} else {
			load = *hardware.load.get( &usage.consumer ).unwrap();
			let diff = i64::from( load_target ) - i64::from( load );
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
		}
		hardware.load.insert( usage.consumer.clone(), load );
	}
}


/// Update the usage display. This moves the current usage value slowly to the target usage value so that the change is smooth and is not jumping around.
pub fn draw_usage(
	mut query: Query<( &mut Transform, &Usage ), With<InstrumentCpu>>,
	cpu_query: Query<&Cpu>,
	hw_query: Query<&Hardware>,
) {
	let cpu = cpu_query.single();
	let hardware = hw_query.single();
	let mut transform_prev: Option<Mut<Transform>> = None;
	for ( mut transform, usage ) in query.iter_mut() {
// 		let scale_target = usage.load as f32 / cpu.capacity as f32;
		let scale_target = *hardware.load.get( &usage.consumer ).unwrap() as f32 / cpu.capacity as f32;
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
