use crate::{game_constants::*, Collider};
use bevy::prelude::*;

#[derive(Component)]
pub struct Brick;
impl Brick {
    pub fn spawn_bricks(commands: &mut Commands) {
        let total_width_bricks = (RIGHT_WALL - LEFT_WALL) - 2. * GAP_BETWEEN_BRICKS_AND_SIDEWALLS;
        let bottom_edge_of_bricks = PADDLE_START_Y + GAP_BETWEEN_PADDLE_AND_BRICKS * 2.0;
        let total_height_of_bricks =
            TOP_WALL - bottom_edge_of_bricks - GAP_BETWEEN_BRICKS_AND_CEILING;

        let n_columns = (total_width_bricks / (BRICK_SIZE.x + GAP_BETWEEN_BRICKS)).floor() as usize;
        let n_rows =
            (total_height_of_bricks / (BRICK_SIZE.y + GAP_BETWEEN_BRICKS)).floor() as usize;
        let n_vert_gaps = n_columns - 1;

        let center_of_bricks = (LEFT_WALL + RIGHT_WALL) / 2.;
        let left_edge_of_bricks = center_of_bricks
            - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
            - n_vert_gaps as f32 / 2.0 * GAP_BETWEEN_BRICKS;

        let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.;
        let offset_y = bottom_edge_of_bricks + BRICK_SIZE.y / 2.;

        for row in 0..n_rows {
            for col in 0..n_columns {
                let brick_position = Vec2::new(
                    offset_x + col as f32 * (BRICK_SIZE.x + GAP_BETWEEN_BRICKS),
                    offset_y + row as f32 * (BRICK_SIZE.y + GAP_BETWEEN_BRICKS),
                );

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: BRICK_COLOR,
                            ..default()
                        },
                        transform: Transform {
                            translation: brick_position.extend(0.0),
                            scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                            ..default()
                        },
                        ..default()
                    },
                    Brick,
                    Collider,
                ));
            }
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct BrickCounter(pub usize);

pub fn brick_counter_system(mut brick_counter: ResMut<BrickCounter>, query: Query<&Brick>) {
    brick_counter.0 = query.iter().count();
}
