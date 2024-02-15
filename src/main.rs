use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ), HelloPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}


#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>
) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don’t need to change any other names
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, ((update_people, greet_people).chain()));
    }
}

#[derive(Component)]
enum DirectionX {
    Left,
    Right,
}

#[derive(Component)]
enum DirectionY {
    Up,
    Down,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("pixel/bevy_pixel_dark.png"),
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        DirectionX::Right,
        DirectionY::Up,
    ));
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut DirectionX, &mut DirectionY, &mut Transform)>) {
    for (mut logo, mut logo2, mut transform) in &mut sprite_position {
        match *logo {
            DirectionX::Right => transform.translation.x += 30. * time.delta_seconds(),
            DirectionX::Left => transform.translation.x -= 30. * time.delta_seconds(),
        }
        
        if transform.translation.x > 200. {
            *logo = DirectionX::Left;
        } else if transform.translation.x < -200. {
            *logo = DirectionX::Right;
        }

        match *logo2 {
            DirectionY::Up => transform.translation.y += 10. * time.delta_seconds(),
            DirectionY::Down => transform.translation.y -= 10. * time.delta_seconds(),
        }
        
        if transform.translation.y > 200. {
            *logo2 = DirectionY::Down;
        } else if transform.translation.y < -200. {
            *logo2 = DirectionY::Up;
        }
    }
}