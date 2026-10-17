use engine::{ecs::{bundle::Bundle, query::filter::With, systems::{commands::Commands, query::{Maybe, Query}}}, render::color::{Color, RGBA}, ui::{geometry::{UiRect, vh, vw}, ui_node::{AlignItems, BackgroundColor, Display, JustifyContent, Node}, widget::{Text, TextColor, TextSize}}, utils::Timer};
use engine_macros::Component;

use crate::scenes::{FadedIn, FadedOut};

#[derive(Component)]
pub struct SplashScreen;

pub fn spawn_splash_screen(mut commands : Commands) {
    commands
        .spawn_with_id((
            Node {
                width: vw(100),
                height: vh(100),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                display: Display::Block,
                ..Default::default()
            },
            BackgroundColor(Color::NONE),
            SplashScreen
        ))
        .add_children(|commands| 
            commands.spawn_with_id((
                Node {
                    ..Default::default()
                },
                Text::new("Loading..."),
                TextSize(30),
                TextColor(Color::RGBA(RGBA::new(1., 1., 1., 0.))),
                SplashScreen
            )).and()
        );
}

pub fn fading_out_splash(
    fading_timer : Maybe<&Timer<FadedOut, ()>>,
    query : Query<(Option<&mut BackgroundColor>), With<SplashScreen>>
) {
    let Some(timer) = *fading_timer else {
        return;
    };

    query.into_iter().for_each(|(backround_color)| {
        if let Some(mut backround_color)= backround_color {
            backround_color.0.set_alpha(timer.time());
        }
    });
}


pub fn fading_in_splash(
    fading_timer : Maybe<&Timer<FadedIn, ()>>,
    query : Query<(Option<&mut BackgroundColor>), With<SplashScreen>>
) {
    let Some(timer) = *fading_timer else {
        return;
    };

    query.into_iter().for_each(|backround_color| {
        if let Some(mut backround_color)= backround_color {
            backround_color.0.set_alpha((1.0 - timer.time()).max(0.0));
        }
    });
}