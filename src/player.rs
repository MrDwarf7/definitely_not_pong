use crate::game_constants::*;
use bevy::prelude::*;

// use crate::PlayerPaddle; // One import to call into the sub-mods // Re-export the player module

#[derive(Component)] // Also known as a "tag" component
pub struct PlayerPaddle;

pub struct PlayerKeys<L, R> {
    pub left: L,
    pub right: R,
}

pub struct LeftKeys(pub Vec<KeyCode>);
pub struct RightKeys(pub Vec<KeyCode>);

impl PlayerKeys<LeftKeys, RightKeys> {
    pub fn new(left: Vec<KeyCode>, right: Vec<KeyCode>) -> Self {
        Self {
            left: LeftKeys(left),
            right: RightKeys(right),
        }
    }
}

impl Default for PlayerKeys<LeftKeys, RightKeys> {
    fn default() -> Self {
        let lefts = vec![
            KeyCode::ArrowLeft,
            KeyCode::KeyA,
            KeyCode::KeyJ,
            KeyCode::KeyH,
        ];

        let rights = vec![
            KeyCode::ArrowRight,
            KeyCode::KeyD,
            KeyCode::KeyK,
            KeyCode::KeyL,
        ];

        Self::new(lefts, rights)
    }
}

pub fn move_paddle(
    input: Res<ButtonInput<KeyCode>>,
    time_step: Res<Time>,
    mut query: Query<&mut Transform, With<PlayerPaddle>>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    let player_keys = PlayerKeys::default();

    for key in &player_keys.left.0 {
        if input.pressed(*key) {
            direction -= 1.0;
        }
    }

    for key in &player_keys.right.0 {
        if input.pressed(*key) {
            direction += 1.0;
        }
    }

    let mut new_x =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time_step.delta_seconds();

    new_x = new_x.clamp(
        LEFT_WALL + (PADDLE_SIZE.x / 2.0),
        RIGHT_WALL - (PADDLE_SIZE.x / 2.0),
    );

    paddle_transform.translation.x = new_x;
}
