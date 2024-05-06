use bevy::prelude::*; // One import to call into the sub-mods

pub use self::{
    ball::*,   // Re-export the ball module
    game::*,   // Re-export the game module
    player::*, // Re-export the ball module
    walls::*,  // Re-export the walls module
};

pub mod game {
    use super::*;
    use bevy::window::{PresentMode, WindowResolution, WindowTheme};

    pub const WINDOW_HEIGHT: f32 = 720.0;
    pub const WINDOW_WIDTH: f32 = 1280.0;

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

    pub const PADDLE_START_Y: f32 = BOTTOM_WALL + 50.0;
    pub const PADDLE_SIZE: Vec2 = Vec2::new(250.0, 20.0);
    pub const PADDLE_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
    pub const PADDLE_SPEED: f32 = 500.0;
}

pub mod ball {
    // Use the super::* to call into the parent module (has the bevy::prelude::* import)
    use super::*;

    pub const BALL_COLOR: Color = Color::Rgba {
        red: 0.96,
        green: 0.06,
        blue: 0.04,
        alpha: 1.0,
    };
    pub const BALL_STARTING_POS: Vec3 = Vec3::new(0.0, -50.0, 1.0);
    pub const BALL_DIAMATER: f32 = 40.0;
    pub const BALL_SPEED: f32 = 550.0;
    pub const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);
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
        red: 0.0,
        green: 0.8,
        blue: 0.8,
        alpha: 1.0,
    };
}
