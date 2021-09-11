//! Entry point when starting the program.




//=============================================================================
// Crates


use chrono::Duration;
use chrono::naive::NaiveDateTime;
use bevy::prelude::*;
use bevy::core::FixedTimestep;

mod schedule;
use schedule::{Clock, ComputerSchedule};

mod ui;
use ui::{UiMaterials, ClockWidget};

mod computer;
use computer::ComputerMaterials;




//=============================================================================
// Constants


const TIMESTAMP_START: i64 = 2481201120;




//=============================================================================
// Plugins


pub struct ComputerPlugin;

impl Plugin for ComputerPlugin {
	fn build( &self, app: &mut AppBuilder ) {
		app.init_resource::<UiMaterials>()
			.init_resource::<ComputerMaterials>()
			.add_startup_system( setup.system() )
			.add_startup_system( ui::spawn_ui.system() )
			.add_startup_system( computer::spawn_cpu.system() )
			.add_system( bevy::input::system::exit_on_esc_system.system() )
			.add_system( update_clock.system() )
			.add_system_set(
				SystemSet::new()
					.with_run_criteria( FixedTimestep::step( 1.0 ) )
					.with_system( computer::jitter_usage.system() ),
			)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria( FixedTimestep::step( 0.01 ) )
					.with_system( computer::update_usage_smooth.system() ),
			)
			.add_system( ui::observe_button.system() )
			.add_system( computer::update_usage.system() );
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

	// Load sprite
	let texture_handle = asset_server.load( "Processor.png" );
	commands.spawn_bundle( SpriteBundle {
		material: materials.add( texture_handle.into() ),
		..Default::default()
	} );

	// Implement clock that tracks and controls the in-game time flow.
	commands.spawn_bundle( (
		Clock {
			datetime: NaiveDateTime::from_timestamp( TIMESTAMP_START, 0 ),
			speed: 1.0,
		},
	) );

	// Implement Computer usage schedule.
	commands.spawn_bundle( (
		ComputerSchedule::from_template( "Family" ),
	) );
}


fn update_clock(
	time: Res<Time>,
	mut query: Query<&mut Text, With<ClockWidget>>,
	mut clock_query: Query<&mut Clock>,
) {
	let mut clock = clock_query.single_mut().unwrap();
	let mut text = query.single_mut().unwrap();

	// Advance in-game time by the real time since the last frame but with the in-game multiplier.
	let time_step_msecs = time.delta_seconds() * clock.speed * 1_000_000.0;
	clock.advance( Duration::microseconds( time_step_msecs.floor() as i64 ) );

	// Write the current in-gane date and time to the game clock widget.
	text.sections[0].value = clock.datetime.format( "%Y-%m-%d %H:%M:%S%.3f" ).to_string();
}




//=============================================================================
// Main


fn main() {
	App::build()
		.add_plugins( DefaultPlugins )
		.add_plugin( ComputerPlugin )
		.run();
}
