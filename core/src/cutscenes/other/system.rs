use engine::{camera::{Camera, MainCamera}, ecs::{entities::{Entity, id::EntityId}, observer::system_param::On, query::filter::With, systems::{commands::Commands, query::Query}}, render::sprite::Sprite};

use crate::cutscenes::{actor::{AppearEffect, HideEffect}, other::{Background, event::BackgroundTransition}};

pub fn on_background_transition(
    on : On<BackgroundTransition>,
    mut commands : Commands,
    background: Query<EntityId, With<Background>>,
    camera : Query<&Camera, With<MainCamera>>
) {
    let BackgroundTransition(transition) = on.event();
    let camera = camera.iter().next().unwrap();
    let background = background.iter().next().unwrap();

    commands.add(background, 
        HideEffect::new(true)
    );

    commands.spawn((
        Sprite::from_image(
            transition.clone(), 
            camera.rect().size()
        ),
        AppearEffect::new(),
        Background
    ));
}