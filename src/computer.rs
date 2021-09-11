//! This module contains everything related to the computer.




//=============================================================================
// Crates


use bevy::prelude::*;




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
pub struct Cpu;


/// This component represents a usage information.
#[derive( Debug )]
pub struct Usage {
	/// The type of the consumer having this usage.
	pub consumer: Consumer,

	/// The load between 0 (no load at all) and 1 (full load).
	pub load: f32,

	/// The amount of jitter of the usage. The higher the number the more the value jitters.
	pub jitter: f32,
}


/// This component represents a status bar.
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
			sprite: Sprite::new( Vec2::new( 120.0, 120.0 ) ),
			..Default::default()
		} )
		.insert( Cpu )
		.with_children( |parent| {
			// Create CPU usage bars
			parent
				.spawn_bundle( SpriteBundle {
					material: materials.system.clone(),
					transform: Transform::from_xyz( -30.0, 0.0, 1.0 ),
					sprite: Sprite::new( Vec2::new( 20.0, 100.0 ) ),
					..Default::default()
				} )
				.insert( StatusBar )
				.insert( Usage{
					consumer: Consumer::System,
					load: 0.0,
					jitter: 0.0,
				} );
			parent
				.spawn_bundle( SpriteBundle {
					material: materials.user.clone(),
					transform: Transform::from_xyz( -10.0, 0.0, 1.0 ),
					sprite: Sprite::new( Vec2::new( 20.0, 100.0 ) ),
					..Default::default()
				} )
				.insert( StatusBar )
				.insert( Usage{
					consumer: Consumer::User,
					load: 0.0,
					jitter: 0.0,
				} );
			parent
				.spawn_bundle( SpriteBundle {
					material: materials.enemy.clone(),
					transform: Transform::from_xyz( 10.0, 0.0, 1.0 ),
					sprite: Sprite::new( Vec2::new( 20.0, 100.0 ) ),
					..Default::default()
				} )
				.insert( StatusBar )
				.insert( Usage{
					consumer: Consumer::Enemy,
					load: 0.0,
					jitter: 0.0,
				} );
			parent
				.spawn_bundle( SpriteBundle {
					material: materials.player.clone(),
					transform: Transform::from_xyz( 30.0, 0.0, 1.0 ),
					sprite: Sprite::new( Vec2::new( 20.0, 100.0 ) ),
					..Default::default()
				} )
				.insert( StatusBar )
				.insert( Usage{
					consumer: Consumer::Player,
					load: 0.0,
					jitter: 0.0,
				} );
		} );
}
