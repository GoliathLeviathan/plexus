//! Entry point when starting the program.




//=============================================================================
// Crates


use rand::Rng;
use chrono::{NaiveDate, NaiveDateTime};
use bevy::prelude::*;
use bevy::core::FixedTimestep;

mod schedule;
use schedule::Tracker;




//=============================================================================
// Constants


const TIMESTAMP_START: i64 = 2481201120;




//=============================================================================
// Enums


#[derive( Debug )]
enum Consumer {
	System,
	User,
	Player,
	Enemy,
}




//=============================================================================
// Components


struct Clock{
	timestamp: i64,
}


struct Cpu;


#[derive( Debug )]
struct Usage {
	consumer: Consumer,
	load: f32,
	jitter: f32,
}


struct StatusBar;




//=============================================================================
// Resources


struct Materials {
	// UI
	ui_normal: Handle<ColorMaterial>,
	ui_hovered: Handle<ColorMaterial>,
	ui_pressed: Handle<ColorMaterial>,
	// Components
	component: Handle<ColorMaterial>,
	player: Handle<ColorMaterial>,
	system: Handle<ColorMaterial>,
	user: Handle<ColorMaterial>,
	enemy: Handle<ColorMaterial>,
}




//=============================================================================
// Plugins


pub struct ComputerPlugin;

impl Plugin for ComputerPlugin {
	fn build( &self, app: &mut AppBuilder ) {
		app.add_startup_system( setup.system() )
			.add_startup_stage( "ui_setup", SystemStage::single( spawn_ui.system() ) )
			.add_startup_stage( "cpu_setup", SystemStage::single( spawn_cpu.system() ) )
			.add_system_set(
				SystemSet::new()
					.with_run_criteria( FixedTimestep::step( 5.0 ) )
					.with_system( randomize_cpu_load.system() ),
			)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria( FixedTimestep::step( 1.0 ) )
					.with_system( update_clock.system() )
					.with_system( jitter_cpu_load.system() ),
			)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria( FixedTimestep::step( 0.01 ) )
					.with_system( refresh_cpu_load.system() ),
			)
			.add_system( observe_button.system() )
			.add_system( animate.system() );
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

	// Create Materials
	commands.insert_resource( Materials {
		ui_normal: materials.add( Color::rgb( 0.0, 0.4, 0.0 ).into() ),
		ui_hovered: materials.add( Color::rgb( 0.0, 0.45, 0.0 ).into() ),
		ui_pressed: materials.add( Color::rgb( 0.0, 0.6, 0.0 ).into() ),
		component: materials.add( Color::rgb( 1.0, 1.0, 1.0 ).into() ),
		player: materials.add( Color::rgb( 0.0, 0.5, 0.0 ).into() ),
		system: materials.add( Color::rgb( 0.5, 0.0, 0.5 ).into() ),
		user: materials.add( Color::rgb( 0.0, 0.0, 0.5 ).into() ),
		enemy: materials.add( Color::rgb( 0.5, 0.0, 0.0 ).into() ),
	} );

	// Load sprite
	let texture_handle = asset_server.load( "Processor.png" );
	commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
	commands.spawn_bundle( SpriteBundle {
		material: materials.add( texture_handle.into() ),
		..Default::default()
	} );

	// Implement timer that controls the in-game time flow.
	commands.spawn_bundle( ( Tracker::new(), ) );

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


fn spawn_ui(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: Res<Materials>,
) {
	// Clock
	commands
		.spawn_bundle( TextBundle {
			style: Style {
				align_self: AlignSelf::FlexEnd,
				position_type: PositionType::Absolute,
				position: Rect {
					top: Val::Px( 15.0 ),
					right: Val::Px( 15.0 ),
					..Default::default()
				},
				..Default::default()
			},
			text: Text::with_section(
				"CLOCK",
				TextStyle {
					font: asset_server.load( "fonts/Orbitron/Orbitron-Regular.ttf" ),
					font_size: 20.0,
					color: Color::WHITE,
				},
				TextAlignment {
					horizontal: HorizontalAlign::Center,
					..Default::default()
				},
			),
			..Default::default()
		})
		.insert( Clock{
			timestamp: TIMESTAMP_START,
		} );

	// Buttons to control the in-game time.
	commands
		.spawn_bundle( ButtonBundle {
			style: Style {
				size: Size::new( Val::Px( 50.0 ), Val::Px( 50.0 ) ),
				// The button is centerd
				margin: Rect::all( Val::Auto ),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				position_type: PositionType::Absolute,
				position: Rect {
					top: Val::Px( 40.0 ),
					right: Val::Px( 15.0 ),
					..Default::default()
				},
				..Default::default()
			},
			material: materials.ui_normal.clone(),
			..Default::default()
		})
		.with_children( |parent| {
			parent.spawn_bundle(TextBundle {
				text: Text::with_section(
					"×16",
					TextStyle {
						font: asset_server.load( "fonts/Orbitron/Orbitron-Regular.ttf" ),
						font_size: 20.0,
						color: Color::rgb( 0.9, 0.9, 0.9 ),
					},
					Default::default(),
				),
				..Default::default()
			} );
		} );
}


fn spawn_cpu(
	mut commands: Commands,
	materials: Res<Materials>,
) {
	// Create CPU-block
	commands
		.spawn_bundle( SpriteBundle {
			material: materials.component.clone(),
			transform: Transform::from_xyz( -140.0, 100.0, 0.0 ),
			sprite: Sprite::new( Vec2::new( 120.0, 120.0 ) ),
			..Default::default()
		} )
		.insert( Cpu );

	// Create CPU usage bars
	commands
		.spawn_bundle( SpriteBundle {
			material: materials.system.clone(),
			transform: Transform::from_xyz( -160.0, 100.0, 0.0 ),
			sprite: Sprite::new( Vec2::new( 20.0, 100.0 ) ),
			..Default::default()
		} )
		.insert( StatusBar )
		.insert( Usage{
			consumer: Consumer::System,
			load: 0.0,
			jitter: 0.0,
		} );
	commands
		.spawn_bundle( SpriteBundle {
			material: materials.user.clone(),
			transform: Transform::from_xyz( -140.0, 100.0, 0.0 ),
			sprite: Sprite::new( Vec2::new( 20.0, 100.0 ) ),
			..Default::default()
		} )
		.insert( StatusBar )
		.insert( Usage{
			consumer: Consumer::User,
			load: 0.0,
			jitter: 0.0,
		} );
	commands
		.spawn_bundle( SpriteBundle {
			material: materials.enemy.clone(),
			transform: Transform::from_xyz( -120.0, 100.0, 0.0 ),
			sprite: Sprite::new( Vec2::new( 20.0, 100.0 ) ),
			..Default::default()
		} )
		.insert( StatusBar )
		.insert( Usage{
			consumer: Consumer::Player,
			load: 0.0,
			jitter: 0.0,
		} );
}


fn animate(time: Res<Time>, mut query: Query<&mut Transform, With<Text>>) {
	// Moving the text slowly in a circle.
	for mut transform in query.iter_mut() {
		transform.translation.x = 100.0 * time.seconds_since_startup().sin() as f32;
		transform.translation.y = 100.0 * time.seconds_since_startup().cos() as f32;
	}
}

fn update_clock( time: Res<Time>, mut query: Query<( &mut Clock, &mut Text )> ) {
	for ( mut clock, mut text ) in query.iter_mut() {
		clock.timestamp += 1;
		let time_start: NaiveDateTime = NaiveDateTime::from_timestamp( clock.timestamp, 0 );
		text.sections[0].value = time_start.format( "%Y-%m-%d %H:%M:%S" ).to_string();
	}
}


fn observe_button(
	materials: Res<Materials>,
	mut interaction_query: Query<( &Interaction, &mut Handle<ColorMaterial> ), ( Changed<Interaction>, With<Button> )>,
	mut tracker_query: Query<&mut Tracker>,
) {
	let mut tracker = tracker_query.single_mut().unwrap();
	for ( interaction, mut material ) in interaction_query.iter_mut() {
		match *interaction {
			Interaction::Clicked => {
				*material = materials.ui_pressed.clone();
				tracker.speed = 16.0;
			}
			Interaction::Hovered => {
				*material = materials.ui_hovered.clone();
			}
			Interaction::None => {
				*material = materials.ui_normal.clone();
			}
		}
	}
}


fn randomize_cpu_load( mut query: Query<&mut Usage, With<StatusBar>> ) {
	for mut usage in query.iter_mut() {
		usage.load = match &usage.consumer {
			Consumer::System => rand::random::<f32>(),
			Consumer::User => rand::random::<f32>(),
			Consumer::Player => rand::random::<f32>(),
			Consumer::Enemy => rand::random::<f32>(),
		};
	}
}


fn jitter_cpu_load( mut query: Query<&mut Usage, With<Cpu>> ) {
	for mut usage in query.iter_mut() {
		usage.jitter = usage.load + 0.2 * rand::random::<f32>() - 0.1;
	}
}


fn refresh_cpu_load( mut query: Query<( &Usage, &mut Transform )> ) {
	let step = 0.01;
	for ( usage, mut transform ) in query.iter_mut() {
		let scale_target = usage.load + usage.jitter;
		if transform.scale.y > scale_target {
			transform.scale.y -= step;
		} else if transform.scale.y < scale_target {
			transform.scale.y += step;
		}
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
