//! This module contains all resources, components and systes regarding the user interface (UI).




//=============================================================================
// Crates


use bevy::prelude::*;

use crate::schedule::{Clock, ComputerSchedule};
use crate::computer::{Usage, InstrumentCpu, ConsumerPlayer};




//=============================================================================
// Resources


enum CustomColor {}

impl CustomColor {
	pub const NORMAL: Color = Color::rgb( 0.0, 0.4, 0.0 );
	pub const HOVERED: Color = Color::rgb( 0.0, 0.45, 0.0 );
	pub const PRESSED: Color = Color::rgb( 0.0, 0.4, 0.0 );
	pub const DISABLED: Color = Color::rgb( 0.0, 0.45, 0.0 );
}


pub struct UiMaterials {
	normal: Handle<ColorMaterial>,
	hovered: Handle<ColorMaterial>,
	pressed: Handle<ColorMaterial>,
	disabled: Handle<ColorMaterial>,
}

impl FromWorld for UiMaterials {
	fn from_world( world: &mut World ) -> Self {
		let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
		UiMaterials {
			normal: materials.add( Color::rgb( 0.0, 0.4, 0.0 ).into() ),
			hovered: materials.add( Color::rgb( 0.0, 0.45, 0.0 ).into() ),
			pressed: materials.add( Color::rgb( 0.0, 0.6, 0.0 ).into() ),
			disabled: materials.add( Color::rgb( 0.5, 0.5, 0.5 ).into() ),
		}
	}
}




//=============================================================================
// Components


#[derive( Component )]
pub struct Widget {
	disabled: bool,
}


#[derive( Component )]
pub struct ClockWidget;


#[derive( Component )]
pub struct ComputerInteraction;


#[derive( Component )]
pub struct SpeedButton {
	multiplier: f32,
}


#[derive( Component )]
pub struct LoadButton {
	value: i32,
}




//=============================================================================
// Systems


pub fn spawn_ui(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	materials: Res<UiMaterials>,
) {
	// Clock
	commands
		.spawn_bundle( TextBundle {
			style: Style {
				size: Size::new( Val::Px( 240.0 ), Val::Px( 10.0 ) ),
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
				"YYYY-MM-DD hh:mm:ss.µµµ",
				TextStyle {
					font: asset_server.load( "fonts/Orbitron/Orbitron-Regular.ttf" ),
					font_size: 20.0,
					color: Color::WHITE,
				},
				TextAlignment {
					horizontal: HorizontalAlign::Left,
					..Default::default()
				},
			),
			..Default::default()
		} )
		.insert( ClockWidget );

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
					right: Val::Px( 180.0 ),
					..Default::default()
				},
				..Default::default()
			},
			color: UiColor::from( CustomColor::NORMAL ),
			..Default::default()
		} )
		.insert( Widget {
			disabled: false,
		} )
		.insert( SpeedButton {
			multiplier: 1.0,
		} )
		.with_children( |parent| {
			parent.spawn_bundle( TextBundle {
				text: Text::with_section(
					"×1",
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
					right: Val::Px( 125.0 ),
					..Default::default()
				},
				..Default::default()
			},
			color: UiColor::from( CustomColor::NORMAL ),
			..Default::default()
		} )
		.insert( Widget {
			disabled: false,
		} )
		.insert( SpeedButton {
			multiplier: 16.0,
		} )
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
					right: Val::Px( 70.0 ),
					..Default::default()
				},
				..Default::default()
			},
			color: UiColor::from( CustomColor::NORMAL ),
			..Default::default()
		} )
		.insert( Widget {
			disabled: false,
		} )
		.insert( SpeedButton {
			multiplier: 128.0,
		} )
		.with_children( |parent| {
			parent.spawn_bundle(TextBundle {
				text: Text::with_section(
					"×128",
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
			color: UiColor::from( CustomColor::NORMAL ),
			..Default::default()
		} )
		.insert( Widget {
			disabled: false,
		} )
		.insert( SpeedButton {
			multiplier: 1024.0,
		} )
		.with_children( |parent| {
			parent.spawn_bundle(TextBundle {
				text: Text::with_section(
					"×1024",
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


	// Buttons to control the load the player is allocating.
	commands
		.spawn_bundle( ButtonBundle {
			style: Style {
				size: Size::new( Val::Px( 150.0 ), Val::Px( 50.0 ) ),
				margin: Rect::all( Val::Auto ),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				position_type: PositionType::Absolute,
				position: Rect {
					top: Val::Px( 10.0 ),
					left: Val::Px( 10.0 ),
					..Default::default()
				},
				..Default::default()
			},
			color: UiColor::from( CustomColor::NORMAL ),
			..Default::default()
		} )
		.insert( Widget {
			disabled: false,
		} )
		.insert( ComputerInteraction )
		.insert( LoadButton {
			value: 10,
		} )
		.with_children( |parent| {
			parent.spawn_bundle( TextBundle {
				text: Text::with_section(
					"Load +",
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
	commands
		.spawn_bundle( ButtonBundle {
			style: Style {
				size: Size::new( Val::Px( 150.0 ), Val::Px( 50.0 ) ),
				margin: Rect::all( Val::Auto ),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				position_type: PositionType::Absolute,
				position: Rect {
					top: Val::Px( 60.0 ),
					left: Val::Px( 10.0 ),
					..Default::default()
				},
				..Default::default()
			},
			color: UiColor::from( CustomColor::NORMAL ),
			..Default::default()
		} )
		.insert( Widget {
			disabled: false,
		} )
		.insert( ComputerInteraction )
		.insert( LoadButton {
			value: -10,
		} )
		.with_children( |parent| {
			parent.spawn_bundle( TextBundle {
				text: Text::with_section(
					"Load -",
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


/// Disable widgets that control the Computer, when the computer is off.
/// TODO: This is checking the clock every frame and changes the material every frame. There must be a better way.
pub fn ui_disable(
	materials: Res<UiMaterials>,
	clock_query: Query<&Clock>,
	schedule_query: Query<&ComputerSchedule>,
	mut query: Query<( &mut Widget, &mut Handle<ColorMaterial> ), ( With<Button>, With<ComputerInteraction> )>,
) {
	let clock = clock_query.single();
	let schedule = schedule_query.single();
	for ( mut widget, mut material ) in query.iter_mut() {
		if schedule.is_on( clock.datetime.time() ) {
			widget.disabled = false;
			*material = materials.normal.clone();
		} else {
			widget.disabled = true;
			*material = materials.disabled.clone();
		}
	}
}


pub fn ui_interact(
	materials: Res<UiMaterials>,
	mut interaction_query: Query<
		( &Interaction, &Widget, &mut Handle<ColorMaterial> ),
		( Changed<Interaction>, With<Button> )
	>,
) {
	for ( interaction, widget, mut material ) in interaction_query.iter_mut() {
		if widget.disabled {
			// Disabled widgets give no feedback.
			continue;
		}
		match *interaction {
			Interaction::Clicked => {
				*material = materials.pressed.clone();
			},
			Interaction::Hovered => {
				*material = materials.hovered.clone();
			},
			Interaction::None => {
				*material = materials.normal.clone();
			},
		}
	}
}


pub fn change_time_speed_by_button(
	mut interaction_query: Query<
		( &SpeedButton, &Interaction ),
		( Changed<Interaction>, With<Button> )
	>,
	mut clock_query: Query<&mut Clock>,
) {
	let mut clock = clock_query.single_mut();
	for ( button, interaction ) in interaction_query.iter_mut() {
		match *interaction {
			Interaction::Clicked => {
				clock.speed = button.multiplier;
			},
			_ => (),
		}
	}
}


pub fn change_load_by_button(
	mut usage_query: Query<&mut Usage, ( With<InstrumentCpu>, With<ConsumerPlayer> )>,
	mut interaction_query: Query<
		( &LoadButton, &Interaction ),
		( Changed<Interaction>, With<Button> )
	>,
) {
	let mut usage = usage_query.single_mut();
	for ( button, interaction ) in interaction_query.iter_mut() {
		match *interaction {
			Interaction::Clicked => {
				if button.value < 0 {
					let val = -button.value as u32;
					if usage.load < val {
						usage.load = 0;
					} else {
						usage.load -= -button.value as u32;
					}
				} else {
					usage.load += button.value as u32;
				}
			},
			_ => (),
		}
	}
}
