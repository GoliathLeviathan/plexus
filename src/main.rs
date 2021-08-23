//! Entry point when starting the program.




//=============================================================================
// Crates


use bevy::prelude::*;




//=============================================================================
// Components


struct Person;


struct Name( String );




//=============================================================================
// Plugins


pub struct HelloPlugin;

impl Plugin for HelloPlugin {
	fn build( &self, app: &mut AppBuilder ) {
		app.insert_resource( GreetTimer( Timer::from_seconds( 2.0, true ) ) )
			.add_startup_system( add_people.system() )
			.add_system( greet_people.system() );
	}
}




//=============================================================================
// Resources


struct GreetTimer( Timer );




//=============================================================================
// Systems


fn add_people( mut commands: Commands ) {
	commands.spawn().insert( Person ).insert( Name( "Elaina Proctor".to_string() ) );
	commands.spawn().insert( Person ).insert( Name( "Renzo Hume".to_string() ) );
	commands.spawn().insert( Person ).insert( Name( "Zayna Nieves".to_string() ) );
}


fn greet_people(
	time: Res<Time>,
	mut timer: ResMut<GreetTimer>,
	query: Query<&Name, With<Person>>
) {
	// update our timer with the time elapsed since the last update
	// if that caused the timer to finish, we say hello to everyone
	if timer.0.tick(time.delta()).just_finished() {
		for name in query.iter() {
			println!( "Hello {}!", name.0 );
		}
	}
}




//=============================================================================
// Main


fn main() {
	App::build()
		.add_plugins( DefaultPlugins )
		.add_plugin( HelloPlugin )
		.run();
}
