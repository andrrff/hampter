use bevy::prelude::*;

struct Person;

struct Name(String);

struct GreetTimer(Timer);

fn string_people(commands: &mut Commands)
{
    commands
        .spawn((Person, Name("André".to_string())))
        .spawn((Person, Name("Eric".to_string())))
        .spawn((Person, Name("Rosilene".to_string())));
}

fn greet_people(
    time: Res<Time>, 
    mut timer: ResMut<GreetTimer>, 
    query: Query<&Name, With<Person>>)
{

    if !timer.0.tick(time.delta_seconds()).just_finished()
    {
        return;
    }

    for name in query.iter()
    {
        println!("Hello {}!", name.0);
    }
}

fn hello_world()
{
    println!("Hello World!");
}

pub struct MyFirstPlugin;

impl Plugin for MyFirstPlugin
{
    fn build(&self, app: &mut AppBuilder)
    {
        app
        // .add_resource(GreetTimer(Timer::from_seconds(5.0, false)))
        .add_startup_system(string_people.system())
        .add_system(greet_people.system())
        .add_system(hello_world.system());
    }
}

fn main()
{
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(MyFirstPlugin)
        .run();
}