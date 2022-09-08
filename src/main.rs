use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_startup_system(add_people)
        .add_system(hello_world)
        .add_system(greet_people)
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
