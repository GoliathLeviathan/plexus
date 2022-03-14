//! This module contains everything related to the computer.




//=============================================================================
// Crates


use bevy::prelude::*;

use crate::schedule::{Clock, ComputerSchedule};




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


#[derive( Debug )]
pub enum Consumer {
	System,
	User,
	Player,
	Enemy,
}




//=============================================================================
// Resources


pub struct ComputerMaterials {
	component: Handle<ColorMaterial>,
	player: Handle<ColorMaterial>,
	system: Handle<ColorMaterial>,
	user: Handle<ColorMaterial>,
	enemy: Handle<ColorMaterial>,
}

impl FromWorld for ComputerMaterials {
	fn from_world( world: &mut World ) -> Self {
		let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
		ComputerMaterials {
			component: materials.add( Color::rgb( 0.1, 0.1, 0.1 ).into() ),
			player: materials.add( Color::rgb( 0.0, 0.5, 0.0 ).into() ),
			system: materials.add( Color::rgb( 0.5, 0.0, 0.5 ).into() ),
			user: materials.add( Color::rgb( 0.0, 0.0, 0.5 ).into() ),
			enemy: materials.add( Color::rgb( 0.5, 0.0, 0.0 ).into() ),
		}
	}
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
	consumer: Consumer,

	/// The load between 0 (no load at all) and 1 (full load).
	pub load: u32,

	/// The amount of jitter of the usage. The higher the number the more the value jitters.
	jitter: f32,
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
	materials: Res<ComputerMaterials>,
) {
	// Create CPU-block
	commands
		.spawn_bundle( SpriteBundle {
			material: materials.component.clone(),
			transform: Transform::from_xyz( -140.0, 100.0, 0.0 ),
			sprite: Sprite::new( Vec2::new( CPU_SIZE[0], CPU_SIZE[1] ) ),
			..Default::default()
		} )
		.insert( Cpu {
			capacity: 1000,
		} )
		.with_children( |parent| {
			// Create CPU usage bars
			parent
				.spawn_bundle( SpriteBundle {
					material: materials.system.clone(),
					transform: Transform::from_xyz( 0.0, -USAGE_BAR_SIZE[1] / 2.0, 1.0 ),
					sprite: Sprite::new( Vec2::new( USAGE_BAR_SIZE[0], USAGE_BAR_SIZE[1] ) ),
					..Default::default()
				} )
				.insert( InstrumentCpu )
				.insert( StatusBar )
				.insert( Usage{
					consumer: Consumer::System,
					load: 0,
					jitter: 0.0,
				} );
			parent
				.spawn_bundle( SpriteBundle {
					material: materials.user.clone(),
					transform: Transform::from_xyz( 0.0, -USAGE_BAR_SIZE[1] / 2.0, 1.0 ),
					sprite: Sprite::new( Vec2::new( USAGE_BAR_SIZE[0], USAGE_BAR_SIZE[1] ) ),
					..Default::default()
				} )
				.insert( InstrumentCpu )
				.insert( StatusBar )
				.insert( Usage{
					consumer: Consumer::User,
					load: 0,
					jitter: 0.0,
				} );
			parent
				.spawn_bundle( SpriteBundle {
					material: materials.enemy.clone(),
					transform: Transform::from_xyz( 0.0, -USAGE_BAR_SIZE[1] / 2.0, 1.0 ),
					sprite: Sprite::new( Vec2::new( USAGE_BAR_SIZE[0], USAGE_BAR_SIZE[1] ) ),
					..Default::default()
				} )
				.insert( InstrumentCpu )
				.insert( StatusBar )
				.insert( Usage{
					consumer: Consumer::Enemy,
					load: 0,
					jitter: 0.0,
				} );
			parent
				.spawn_bundle( SpriteBundle {
					material: materials.player.clone(),
					transform: Transform::from_xyz( 0.0, -USAGE_BAR_SIZE[1] / 2.0, 1.0 ),
					sprite: Sprite::new( Vec2::new( USAGE_BAR_SIZE[0], USAGE_BAR_SIZE[1] ) ),
					..Default::default()
				} )
				.insert( InstrumentCpu )
				.insert( StatusBar )
				.insert( Usage{
					consumer: Consumer::Player,
					load: 0,
					jitter: 0.0,
				} )
				.insert( ConsumerPlayer );
		} );
}


/// Update the computer usage.
pub fn update_usage(
	mut query: Query<&mut Usage, With<StatusBar>>,
	clock_query: Query<&Clock>,
	schedule_query: Query<&ComputerSchedule>
) {
	let clock = clock_query.single().unwrap();
	let schedule = schedule_query.single().unwrap();
	for mut usage in query.iter_mut() {
		let load = schedule.load( &usage.consumer, clock.datetime.time() );
		match load {
			Ok( x ) => usage.load = x,
			Err( _ ) => return (),
		}
	}
}


/// Introduce a slight jitter on all usage displays.
pub fn jitter_usage(
	mut query: Query<&mut Usage, With<InstrumentCpu>>
) {
	for mut usage in query.iter_mut() {
		if usage.load > 0 {
			usage.jitter = 0.04 * rand::random::<f32>() - 0.02;
		} else {
			usage.jitter = 0.0;
		}
	}
}


/// Update the usage display. This moves the current usage value slowly to the target usage value so that the change is smooth and is not jumping around.
pub fn draw_usage_smooth(
	mut query: Query<( &mut Transform, &Usage ), With<InstrumentCpu>>,
	cpu_query: Query<&Cpu>,
) {
	let cpu = cpu_query.single().unwrap();
	let step = 0.01;
	let mut transform_prev: Option<Mut<Transform>> = None;
	for ( mut transform, usage ) in query.iter_mut() {
		let scale_target = ( usage.load as f32 / cpu.capacity as f32 ) + usage.jitter;
		if transform.scale.y > scale_target + step {
			transform.scale.y -= step;
		} else if transform.scale.y < scale_target - step {
			transform.scale.y += step;
		}

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
