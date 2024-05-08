use crate::game_constants::*;
use bevy::prelude::*;
// use bevy_iced::IcedContext;

#[derive(Resource, Deref, DerefMut)]
pub struct Score(pub usize);

#[derive(Component)]
pub struct ScoreboardUi;

impl ScoreboardUi {
    pub fn setup_scoreboard(commands: &mut Commands) {
        commands.spawn((
            ScoreboardUi,
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font_size: SCOREBOARD_FONT_SIZE,
                        color: TEXT_COLOR,
                        ..default()
                    },
                ),
                TextSection::from_style(TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: SCOREBOARD_COLOR,
                    ..default()
                }),
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            }),
        ));
    }

    pub fn update_scoreboard(score: Res<Score>, mut query: Query<&mut Text, With<ScoreboardUi>>) {
        let mut text = query.single_mut();
        text.sections[1].value = score.to_string();
    }
}

#[derive(Event)]
pub enum WinningMessage {}

pub fn winning_condition(score: Res<Score>) {
    if **score == WINNING_SCORE {
        println!("You win!")
    }
}
