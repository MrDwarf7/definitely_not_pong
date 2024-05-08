#![allow(dead_code, unused_imports)]
// Entity - basically just an ID or something we can FIND in the world (which will HAVE components)
// Components - {"transform", "Health", "Velocity"} - data that can be attached to an entity, can almost think of it as the attributes of a struct
// Systems - the things that OPERATE on the components of an entity, can almost think of it as the methods of a struct,
//--- Systems are things that will DO stuff in the gameworld (like update the position of an entity, or check if a player has died, that sort of thing)
//
// Entity exists -> Entity is connected to a Component (for it's health etc.) <- System will operate on the component
//                                                                      (which will, by extension, operate on the entity)
use bevy::prelude::*;

// Global Game Constants
mod game_constants;
use game_constants::*;

// Player Controls
mod player;
use player::{move_paddle, LeftKeys, PlayerKeys, PlayerPaddle, RightKeys};

mod score;
use score::*;

mod ball;
use ball::*;

mod brick;
use brick::*;

// Bounds & Collisions
mod walls_container;
use walls_container::{WallBundle, WallLocation};

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;

// bricks

//
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(window_constructor()))
        .insert_resource(Score(0))
        .insert_resource(background_color())
        .add_systems(Update, bevy::window::close_on_esc)
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup) // Startup will only run ONCE at the start of the game, not every frame
        .add_systems(
            FixedUpdate,
            (
                move_paddle,
                apply_velocity,
                check_ball_collisions.after(apply_velocity),
            ),
        )
        .add_systems(Update, ScoreboardUi::update_scoreboard)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //
    // Create a camera so we can see stuff
    commands.spawn(Camera2dBundle::default());

    // Then we create the player paddle
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0., PADDLE_START_Y, 0.),
                scale: PADDLE_SIZE.extend(1.0),
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        PlayerPaddle,
        Collider,
    ));

    // Then we create the ball
    // Create the ball texture in mem and load it into the asset server, before we spawn the ball
    let ball_tex = asset_server.load("textures/ball.png");
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: BALL_STARTING_POS,
                ..default()
            },
            sprite: Sprite {
                color: BALL_COLOR,
                custom_size: Some(BALL_DIAMATER * Vec2::ONE),
                ..default()
            },
            texture: ball_tex,
            ..default()
        },
        Ball,
        Velocity(BALL_INITIAL_DIRECTION.normalize() * BALL_SPEED),
    ));

    ScoreboardUi::setup_scoreboard(&mut commands);

    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));

    Brick::spawn_bricks(&mut commands);
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.delta_seconds();
        transform.translation.y += velocity.y * time_step.delta_seconds();
    }
}
