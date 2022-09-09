use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people)
            .add_system(hello_world)
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            present_mode: bevy::window::PresentMode::AutoVsync,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello {}!", name.0);
    }
}

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Adinelson".to_string()));

    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo".to_string()));

    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna".to_string()));
}

fn hello_world() {
    println!("Hello World!");
}
