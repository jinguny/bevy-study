use bevy::prelude::*;

struct Person;
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Alice".to_string()));
    commands.spawn().insert(Person).insert(Name("Bob".to_string()));
}

fn hello_world() {
    println!("hello world!")
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("hello {}!", name.0)
    }
}

fn main() {
    App::build()
        .add_startup_system(add_people.system())
        .add_system(hello_world.system())
        .add_system(greet_people.system())
        .run();
}