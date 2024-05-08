use bevy::prelude::*; // One import to call into the sub-mods

pub use self::{
    ball::*,       // Re-export the ball module
    bricks::*,     // Re-export the bricks module
    game::*,       // Re-export the game module
    player::*,     // Re-export the ball module
    scoreboard::*, // Re-export the scoreboard module
    walls::*,      // Re-export the walls module
};

pub mod game {
    use super::*;
    use bevy::window::{PresentMode, WindowResolution, WindowTheme};

    pub const WINDOW_HEIGHT: f32 = 720.0;
    pub const WINDOW_WIDTH: f32 = 1280.0;
    pub const TEXT_COLOR: Color = Color::WHITE;

    pub fn background_color() -> ClearColor {
        ClearColor(Color::rgb(0.59, 0.59, 0.59))
    }

    pub fn window_constructor() -> WindowPlugin {
        WindowPlugin {
            primary_window: Some(Window {
                title: "Definitely Not Pong".into(),
                name: Some("Definitely Not Pong".into()),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                present_mode: PresentMode::Immediate,
                window_theme: Some(WindowTheme::Dark),
                enabled_buttons: bevy::window::EnabledButtons { ..default() },
                visible: true,
                resizable: false,
                ..default()
            }),
            ..default()
        }
    }
}

pub mod player {
    // Use the super::* to call into the parent module (has the bevy::prelude::* import)
    use super::*;

    pub const PADDLE_START_Y: f32 = BOTTOM_WALL + 40.0;
    pub const PADDLE_SIZE: Vec2 = Vec2::new(250.0, 20.0);
    pub const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
    pub const PADDLE_SPEED: f32 = 800.0;
}

// Create virtual walls for player + ball
pub mod walls {
    use super::*;

    // Position OF the walls
    pub const LEFT_WALL: f32 = -630.0;
    pub const RIGHT_WALL: f32 = 630.0;
    pub const BOTTOM_WALL: f32 = -350.0;
    pub const TOP_WALL: f32 = 350.0;

    // Size & color OF the walls
    pub const WALL_THICCCNESS: f32 = 10.0;
    pub const WALL_BLOCK_WIDTH: f32 = RIGHT_WALL - LEFT_WALL;
    pub const WALL_BLOCK_HEIGHT: f32 = TOP_WALL - BOTTOM_WALL;

    pub const WALL_COLOR: Color = Color::Rgba {
        red: 0.9,
        green: 0.8,
        blue: 0.8,
        alpha: 1.0,
    };
}

pub mod ball {
    use super::*;

    pub const BALL_STARTING_POS: Vec3 = Vec3::new(PADDLE_START_Y + 20.0, -140.0, 1.0);
    pub const BALL_DIAMATER: f32 = 40.0;
    pub const BALL_SPEED: f32 = 550.0;
    pub const BALL_COLOR: Color = Color::Rgba {
        red: 0.96,
        green: 0.06,
        blue: 0.04,
        alpha: 1.0,
    };
}

pub mod bricks {
    use super::*;

    pub const BRICK_SIZE: Vec2 = Vec2::new(100.0, 30.0);
    pub const GAP_BETWEEN_PADDLE_AND_BRICKS: f32 = 80.0;
    pub const GAP_BETWEEN_BRICKS: f32 = 5.0;

    pub const GAP_BETWEEN_BRICKS_AND_CEILING: f32 = 20.0;
    pub const GAP_BETWEEN_BRICKS_AND_SIDEWALLS: f32 = 15.0;

    pub const BRICK_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
}

pub mod scoreboard {
    use super::*;

    pub const SCOREBOARD_FONT_SIZE: f32 = 35.0;
    pub const SCOREBOARD_COLOR: Color = Color::WHITE;
    pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(WALL_THICCCNESS + 8.0);
}
