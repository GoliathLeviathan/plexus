//! This module contains all resources, components and systes regarding the user interface (UI).




//=============================================================================
// Crates


use bevy::prelude::*;

use crate::materials::CustomColor;
use crate::schedule::{Clock, ComputerSchedule};
use crate::computer::{Usage, InstrumentCpu, ConsumerPlayer};




//=============================================================================
// Constants


/// The margin magnitude around ui elements.
const MARGIN: Val = Val::Px( 5.0 );




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
						justify_content: JustifyContent::FlexStart,
						..Default::default()
					},
					color: Color::NONE.into(),
					..Default::default()
				} )
				.with_children( |parent| {
					// Buttons to control the load the player is allocating.
					parent
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

					parent
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
								size: Size::new( Val::Percent( 100.0 ), Val::Px( 20.0 ) ),
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
							parent
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

							parent
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
									multiplier: 16.0,
								} )
								.with_children( |parent| {
									parent.spawn_bundle( TextBundle {
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

							parent
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
									multiplier: 128.0,
								} )
								.with_children( |parent| {
									parent.spawn_bundle( TextBundle {
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

							parent
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
									multiplier: 1024.0,
								} )
								.with_children( |parent| {
									parent.spawn_bundle( TextBundle {
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
						} );
				} );
		} );
}


/// Disable widgets that control the Computer, when the computer is off.
/// TODO: This is checking the clock every frame and changes the material every frame. There must be a better way.
pub fn ui_disable(
	clock_query: Query<&Clock>,
	schedule_query: Query<&ComputerSchedule>,
	mut query: Query<
		( &mut Widget, &mut UiColor ),
		( With<Button>, With<ComputerInteraction> )
	>,
) {
	let clock = clock_query.single();
	let schedule = schedule_query.single();
	for ( mut widget, mut color ) in query.iter_mut() {
		if schedule.is_on( clock.datetime.time() ) {
			widget.disabled = false;
			*color = CustomColor::NORMAL.into();
		} else {
			widget.disabled = true;
			*color = CustomColor::DISABLED.into();
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
