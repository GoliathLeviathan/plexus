//! This module contains all colors and materials used.




//=============================================================================
// Crates


use bevy::prelude::Color;




//=============================================================================
// Resources


pub enum CustomColor {}

impl CustomColor {
	pub const NORMAL: Color = Color::rgb( 0.0, 0.4, 0.0 );
	pub const HOVERED: Color = Color::rgb( 0.0, 0.45, 0.0 );
	pub const PRESSED: Color = Color::rgb( 0.0, 0.4, 0.0 );
	pub const DISABLED: Color = Color::rgb( 0.0, 0.45, 0.0 );

	pub const COMPONENT: Color = Color::rgb( 0.1, 0.1, 0.1 );
	pub const PLAYER: Color = Color::rgb( 0.0, 0.5, 0.0 );
	pub const SYSTEM: Color = Color::rgb( 0.5, 0.0, 0.5 );
	pub const USER: Color = Color::rgb( 0.0, 0.0, 0.5 );
	pub const ENEMY: Color = Color::rgb( 0.5, 0.0, 0.0 );
}
