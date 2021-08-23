//! Entry point when starting the program.




//=============================================================================
// Crates


use bevy::prelude::*;
use rand::Rng;




//=============================================================================
// Components


struct Cpu;


struct Usage {
	player: f32,
	system: f32,
	user: f32,
	npc: f32,
}

impl Usage {
	fn idle( &self ) -> f32 {
		return 1.0 - self.player - self.system - self.user - self.npc;
	}
}


struct UsageBar;




//=============================================================================
// Plugins


pub struct ComputerPlugin;

impl Plugin for ComputerPlugin {
	fn build( &self, app: &mut AppBuilder ) {
		app.add_startup_system( setup.system() )
			.add_system( animate.system() )
			.add_system( display_cpu_usage.system() );
	}
}




//=============================================================================
// Systems


fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	// Cameras
	commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
	commands.spawn_bundle( UiCameraBundle::default() );

	// Create CPU frame
	commands
		.spawn_bundle( SpriteBundle {
			material: materials.add(Color::rgb( 0.0, 0.0, 0.0 ).into() ),
			transform: Transform::from_xyz( -140.0, 100.0, 0.0 ),
			sprite: Sprite::new( Vec2::new( 120.0, 120.0 ) ),
			..Default::default()
		} )
		.insert( Cpu )
		.insert( Usage {
			player: 0.0,
			system: 0.0,
			user: 0.0,
			npc: 0.0,
		} );

	// Create CPU usage bars
	commands
		.spawn_bundle( SpriteBundle {
			material: materials.add( Color::rgb( 0.5, 0.0, 0.0 ).into() ),
			transform: Transform::from_xyz( -160.0, 100.0, 0.0 ),
			sprite: Sprite::new( Vec2::new( 20.0, 100.0 ) ),
			..Default::default()
		} )
		.insert( UsageBar );
	commands
		.spawn_bundle( SpriteBundle {
			material: materials.add( Color::rgb( 0.0, 0.5, 0.0 ).into() ),
			transform: Transform::from_xyz( -140.0, 100.0, 0.0 ),
			sprite: Sprite::new( Vec2::new( 20.0, 100.0 ) ),
			..Default::default()
		} )
		.insert( UsageBar );

	// Load sprite
	let texture_handle = asset_server.load( "Processor.png" );
	commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
	commands.spawn_bundle( SpriteBundle {
		material: materials.add( texture_handle.into() ),
		..Default::default()
	} );

	// 2D Text
	commands.spawn_bundle( Text2dBundle {
		text: Text::with_section(
			"Simple text message.",
			TextStyle {
				font: asset_server.load( "fonts/Orbitron/Orbitron-Regular.ttf" ),
				font_size: 60.0,
				color: Color::WHITE,
			},
			TextAlignment {
				vertical: VerticalAlign::Center,
				horizontal: HorizontalAlign::Center,
			},
		),
		..Default::default()
	} );
}


fn animate(time: Res<Time>, mut query: Query<&mut Transform, With<Text>>) {
	// Moving the text slowly in a circle.
	for mut transform in query.iter_mut() {
		transform.translation.x = 100.0 * time.seconds_since_startup().sin() as f32;
		transform.translation.y = 100.0 * time.seconds_since_startup().cos() as f32;
	}
}


fn display_cpu_usage( mut query: Query<( &UsageBar, &mut Transform )> ) {
// 	info!( "{:?}, {:?}", time, query );
	for ( _usage_bar, mut transform ) in query.iter_mut() {
		transform.scale.y = rand::random();
	}
}




//=============================================================================
// Main


fn main() {
	App::build()
		.add_plugins( DefaultPlugins )
		.add_plugin( ComputerPlugin )
		.run();
}
