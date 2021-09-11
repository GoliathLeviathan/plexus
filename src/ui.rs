//! This module contains all resources, components and systes regarding the user interface (UI).




//=============================================================================
// Crates


use bevy::prelude::*;

use crate::schedule::Clock;




//=============================================================================
// Resources


pub struct UiMaterials {
	normal: Handle<ColorMaterial>,
	hovered: Handle<ColorMaterial>,
	pressed: Handle<ColorMaterial>,
}

impl FromWorld for UiMaterials {
	fn from_world( world: &mut World ) -> Self {
		let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
		UiMaterials {
			normal: materials.add( Color::rgb( 0.0, 0.4, 0.0 ).into() ),
			hovered: materials.add( Color::rgb( 0.0, 0.45, 0.0 ).into() ),
			pressed: materials.add( Color::rgb( 0.0, 0.6, 0.0 ).into() ),
		}
	}
}




//=============================================================================
// Components


pub struct ClockWidget;


pub struct SpeedButton {
	multiplier: f32,
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
			material: materials.normal.clone(),
			..Default::default()
		} )
		.insert( SpeedButton {
			multiplier: 1.0,
		} )
		.with_children( |parent| {
			parent.spawn_bundle(TextBundle {
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
			material: materials.normal.clone(),
			..Default::default()
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
			material: materials.normal.clone(),
			..Default::default()
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
			material: materials.normal.clone(),
			..Default::default()
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
}


pub fn observe_button(
	materials: Res<UiMaterials>,
	mut interaction_query: Query<( &SpeedButton ,&Interaction, &mut Handle<ColorMaterial> ), ( Changed<Interaction>, With<Button> )>,
	mut clock_query: Query<&mut Clock>,
) {
	let mut clock = clock_query.single_mut().unwrap();
	for ( button, interaction, mut material ) in interaction_query.iter_mut() {
		match *interaction {
			Interaction::Clicked => {
				*material = materials.pressed.clone();
				clock.speed = button.multiplier;
			}
			Interaction::Hovered => {
				*material = materials.hovered.clone();
			}
			Interaction::None => {
				*material = materials.normal.clone();
			}
		}
	}
}
