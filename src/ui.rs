//! This module contains all resources, components and systems regarding the user interface (UI).




//=============================================================================
// Crates


use bevy::prelude::*;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};

use crate::materials::CustomColor;
use crate::machine::{Clock, MachineState, Machine};
use crate::computer::Consumer;




//=============================================================================
// Constants


/// The main font to be used in the UI.
const FONT_MAIN: &str = "fonts/Orbitron/Orbitron-Regular.ttf";


/// The mono font to be used in the UI.
const FONT_MONO: &str = "fonts/DejaVu/DejaVuLGCSansMono.ttf";


/// The size of the main font to be used in the UI.
const FONT_MAIN_SIZE: f32 = 20.0;


/// The size of the debug text to be used in the UI.
const FONT_DEBUG_SIZE: f32 = 12.0;


/// The margin magnitude around ui elements.
const MARGIN: Val = Val::Px( 5.0 );


/// The text height.
const TEXT_HEIGHT: Val = Val::Px( FONT_MAIN_SIZE );




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


#[derive( Component )]
pub struct StateText;


#[derive( Component )]
pub struct LoadText;


#[derive( Component )]
pub struct DiagnosticsText;




//=============================================================================
// Displays


fn disp_state(
	builder: &mut ChildBuilder<'_, '_, '_>,
	asset_server: &Res<AssetServer>,
) {
	builder
		.spawn_bundle( NodeBundle {
			style: Style {
				size: Size::new( Val::Auto, Val::Auto ),
				flex_direction: FlexDirection::Row,
				justify_content: JustifyContent::SpaceBetween,
				..Default::default()
			},
			color: Color::NONE.into(),
			..Default::default()
		} )
		.with_children( |parent| {
			parent
				.spawn_bundle( TextBundle {
					style: Style {
						size: Size::new( Val::Undefined, TEXT_HEIGHT ),
						..Default::default()
					},
					text: Text::with_section(
						"State:",
						TextStyle {
							font: asset_server.load( FONT_MAIN ),
							font_size: FONT_MAIN_SIZE,
							color: Color::WHITE,
						},
						TextAlignment {
							horizontal: HorizontalAlign::Left,
							..Default::default()
						},
					),
					..Default::default()
				} );

			parent
				.spawn_bundle( TextBundle {
					style: Style {
						size: Size::new( Val::Undefined, TEXT_HEIGHT ),
						..Default::default()
					},
					text: Text::with_section(
						"",
						TextStyle {
							font: asset_server.load( FONT_MAIN ),
							font_size: FONT_MAIN_SIZE,
							color: Color::WHITE,
						},
						TextAlignment {
							horizontal: HorizontalAlign::Right,
							..Default::default()
						},
					),
					..Default::default()
				} )
				.insert( StateText );
		} );
}


fn disp_load(
	builder: &mut ChildBuilder<'_, '_, '_>,
	asset_server: &Res<AssetServer>,
	text: &str,
	component: Consumer,
) {
	builder
		.spawn_bundle( NodeBundle {
			style: Style {
				size: Size::new( Val::Auto, Val::Auto ),
				flex_direction: FlexDirection::Row,
				justify_content: JustifyContent::SpaceBetween,
				..Default::default()
			},
			color: Color::NONE.into(),
			..Default::default()
		} )
		.with_children( |parent| {
			parent
				.spawn_bundle( TextBundle {
					style: Style {
						size: Size::new( Val::Undefined, TEXT_HEIGHT ),
						..Default::default()
					},
					text: Text::with_section(
						text,
						TextStyle {
							font: asset_server.load( FONT_MAIN ),
							font_size: FONT_MAIN_SIZE,
							color: Color::WHITE,
						},
						TextAlignment {
							horizontal: HorizontalAlign::Left,
							..Default::default()
						},
					),
					..Default::default()
				} );

			parent
				.spawn_bundle( TextBundle {
					style: Style {
						size: Size::new( Val::Undefined, TEXT_HEIGHT ),
						..Default::default()
					},
					text: Text::with_section(
						"0.0",
						TextStyle {
							font: asset_server.load( FONT_MAIN ),
							font_size: FONT_MAIN_SIZE,
							color: Color::WHITE,
						},
						TextAlignment {
							horizontal: HorizontalAlign::Right,
							..Default::default()
						},
					),
					..Default::default()
				} )
				.insert( LoadText )
				.insert( component );
		} );
}




//=============================================================================
// Buttons


fn button_load(
	builder: &mut ChildBuilder<'_, '_, '_>,
	asset_server: &Res<AssetServer>,
	load: i32,
) {
	let load_text = if load < -10 {
		"Load −−"
	} else if load < 0 {
		"Load −"
	} else if load > 10 {
		"Load ++"
	} else {
		"Load +"
	};

	builder
		.spawn_bundle( ButtonBundle {
			style: Style {
				size: Size::new( Val::Auto, Val::Px( 50.0 ) ),
				margin: Rect::all( MARGIN ),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
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
			value: load,
		} )
		.with_children( |parent| {
			parent.spawn_bundle( TextBundle {
				text: Text::with_section(
					load_text,
					TextStyle {
						font: asset_server.load( FONT_MAIN ),
						font_size: FONT_MAIN_SIZE,
						color: Color::rgb( 0.9, 0.9, 0.9 ),
					},
					Default::default(),
				),
				..Default::default()
			} );
		} );
}


fn button_multiplier(
	builder: &mut ChildBuilder<'_, '_, '_>,
	asset_server: &Res<AssetServer>,
	multiplier: f32,
) {
	builder
		.spawn_bundle( ButtonBundle {
			style: Style {
				size: Size::new( Val::Percent( 25.0 ), Val::Px( 50.0 ) ),
				margin: Rect::all( MARGIN ),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..Default::default()
			},
			color: UiColor::from( CustomColor::NORMAL ),
			..Default::default()
		} )
		.insert( Widget {
			disabled: false,
		} )
		.insert( SpeedButton {
			multiplier: multiplier,
		} )
		.with_children( |parent| {
			parent.spawn_bundle( TextBundle {
				text: Text::with_section(
					format!( "×{}", multiplier.to_string() ),
					TextStyle {
						font: asset_server.load( FONT_MAIN ),
						font_size: FONT_MAIN_SIZE,
						color: Color::rgb( 0.9, 0.9, 0.9 ),
					},
					Default::default(),
				),
				..Default::default()
			} );
		} );
}




//=============================================================================
// Systems


pub fn spawn_ui(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
) {
	// Root node
	commands
		.spawn_bundle( NodeBundle {
			style: Style {
				size: Size::new( Val::Percent( 100.0 ), Val::Percent( 100.0 ) ),
				padding: Rect::all( MARGIN ),
				justify_content: JustifyContent::SpaceBetween,
				..Default::default()
			},
			color: Color::NONE.into(),
			..Default::default()
		} )
		.with_children( |parent| {
			// The left button column (controlling computer)
			parent
				.spawn_bundle( NodeBundle {
					style: Style {
						size: Size::new( Val::Px( 200.0 ), Val::Auto ),
						flex_direction: FlexDirection::ColumnReverse,
						justify_content: JustifyContent::SpaceBetween,
						..Default::default()
					},
					color: Color::NONE.into(),
					..Default::default()
				} )
				.with_children( |parent| {
					// Buttons to control the load the player is allocating.
					parent
						.spawn_bundle( NodeBundle {
							style: Style {
								size: Size::new( Val::Auto, Val::Auto ),
								flex_direction: FlexDirection::ColumnReverse,
// 								justify_content: JustifyContent::FlexStart,
								..Default::default()
							},
							color: Color::NONE.into(),
							..Default::default()
						} )
						.with_children( |parent| {
							button_load( parent, &asset_server, 100 );
							button_load( parent, &asset_server, 10 );
							button_load( parent, &asset_server, -10 );
							button_load( parent, &asset_server, -100 );
						} );

					// Status information about the computer.
					parent
						.spawn_bundle( NodeBundle {
							style: Style {
								size: Size::new( Val::Auto, Val::Auto ),
								flex_direction: FlexDirection::ColumnReverse,
// 								justify_content: JustifyContent::FlexStart,
								..Default::default()
							},
							color: Color::NONE.into(),
							..Default::default()
						} )
						.with_children( |parent| {
							disp_state( parent, &asset_server );
							disp_load( parent, &asset_server, "System", Consumer::System );
							disp_load( parent, &asset_server, "User", Consumer::User );
							disp_load( parent, &asset_server, "Player", Consumer::Player );
							disp_load( parent, &asset_server, "Enemy", Consumer::Enemy );
						} );
				} );

			// The right button column (controlling time)
			parent
				.spawn_bundle( NodeBundle {
					style: Style {
						size: Size::new( Val::Px( 250.0 ), Val::Percent( 100.0 ) ),
// 						padding: Rect::all( MARGIN ),
						flex_direction: FlexDirection::ColumnReverse,
						align_items: AlignItems::FlexStart,
						..Default::default()
					},
					color: Color::NONE.into(),
					..Default::default()
				} )
				.with_children( |parent| {
					// Clock
					parent
						.spawn_bundle( TextBundle {
							style: Style {
								size: Size::new( Val::Percent( 100.0 ), TEXT_HEIGHT ),
								..Default::default()
							},
							text: Text::with_section(
								"YYYY-MM-DD hh:mm:ss.µµµ",
								TextStyle {
									font: asset_server.load( FONT_MAIN ),
									font_size: FONT_MAIN_SIZE,
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

					// Button row
					parent
						.spawn_bundle( NodeBundle {
							style: Style {
								size: Size::new( Val::Percent( 100.0 ), Val::Undefined ),
								flex_direction: FlexDirection::Row,
								justify_content: JustifyContent::SpaceBetween,
								..Default::default()
							},
							color: Color::NONE.into(),
							..Default::default()
						} )
						.with_children( |parent| {
							// Buttons to control the in-game time.
							button_multiplier( parent, &asset_server, 1.0 );
							button_multiplier( parent, &asset_server, 16.0 );
							button_multiplier( parent, &asset_server, 128.0 );
							button_multiplier( parent, &asset_server, 1024.0 );
						} );

					// Diagnostics
					parent
						.spawn_bundle( TextBundle {
							style: Style {
								align_self: AlignSelf::FlexEnd,
								..Default::default()
							},
							text: Text {
								sections: vec![
									TextSection {
										value: "FPS: ".to_string(),
										style: TextStyle {
											font: asset_server.load( FONT_MONO ),
											font_size: FONT_DEBUG_SIZE,
											color: Color::MAROON,
										},
									},
									TextSection {
										value: "".to_string(),
										style: TextStyle {
											font: asset_server.load( FONT_MONO ),
											font_size: FONT_DEBUG_SIZE,
											color: Color::MAROON,
										},
									},
								],
								..Default::default()
							},
							..Default::default()
						} )
						.insert( DiagnosticsText );
				} );
		} );
}


pub fn diagnostics_update(
	diagnostics: Res<Diagnostics>,
	mut query: Query<&mut Text, With<DiagnosticsText>>
) {
	for mut text in query.iter_mut() {
		let mut diags = [ "".to_string(), "".to_string() ];
		if let Some( fps ) = diagnostics.get( FrameTimeDiagnosticsPlugin::FPS ) {
			if let Some( average ) = fps.average() {
				diags[0] = format!( "{:.2}", average );
			}
		}
		if let Some( frame_time ) = diagnostics.get( FrameTimeDiagnosticsPlugin::FRAME_TIME ) {
			if let Some( average ) = frame_time.average() {
				diags[1] = format!( "{:.4}", average );
			}
		}
		text.sections[1].value = format!( "{} ({})", diags[0], diags[1] );
	}
}


/// Disable widgets that control the Computer, when the computer is off.
/// TODO: This is checking the clock every frame and changes the material every frame. There must be a better way.
pub fn ui_disable(
	machine_query: Query<&Machine, Changed<Machine>>,
	mut query: Query<
		( &mut Widget, &mut UiColor ),
		( With<Button>, With<ComputerInteraction> )
	>,
) {
	// This query can be empty, if no change happened. So instead of using `.single()` (which would result in a panic), a for loop is used.
	for machine in machine_query.iter() {
		for ( mut widget, mut color ) in query.iter_mut() {
			match machine.state {
				MachineState::Off => {
					widget.disabled = true;
					*color = CustomColor::DISABLED.into();
				},
				_ => {
					widget.disabled = false;
					*color = CustomColor::NORMAL.into();
				}
			}
		}
	}
}


pub fn ui_interact(
	mut interaction_query: Query<
		( &Interaction, &Widget, &mut UiColor ),
		( Changed<Interaction>, With<Button> )
	>,
) {
	for ( interaction, widget, mut color ) in interaction_query.iter_mut() {
		if widget.disabled {
			// Disabled widgets give no feedback.
			continue;
		}
		match *interaction {
			Interaction::Clicked => {
				*color = CustomColor::PRESSED.into();
			},
			Interaction::Hovered => {
				*color = CustomColor::HOVERED.into();
			},
			Interaction::None => {
				*color = CustomColor::NORMAL.into();
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
	mut machine_query: Query<&mut Machine>,
	mut interaction_query: Query<
		( &LoadButton, &Interaction, &Widget ),
		( Changed<Interaction>, With<Button> )
	>,
) {
	let mut machine = machine_query.single_mut();
	for ( button, interaction, widget ) in interaction_query.iter_mut() {
		if widget.disabled {
			continue;
		}
		match *interaction {
			Interaction::Clicked => {
				let mut load = machine.get_load_target( &Consumer::Player ) as i32;
				load += i32::max( button.value, -load );
				machine.set_load_target( &Consumer::Player, load as u32 );
			},
			_ => (),
		}
	}
}


pub fn display_load(
	mut query: Query<( &mut Text, &Consumer ), With<LoadText>>,
	machine_query: Query<&Machine>,
) {
	let machine = machine_query.single();
	for ( mut text, consumer ) in query.iter_mut() {
		text.sections[0].value = machine.get_load( &consumer ).to_string();
	}
}


pub fn display_state(
	mut query: Query<&mut Text, With<StateText>>,
	machine_query: Query<&Machine>,
) {
	let machine = machine_query.single();
	for mut text in query.iter_mut() {
		text.sections[0].value = machine.state.to_string();
	}
}
