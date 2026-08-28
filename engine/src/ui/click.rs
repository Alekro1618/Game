use std::marker::PhantomData;

use crate::{
    app::{main_schedule::ScheduleLabel::Update, plugin::Plugin},
    ecs::{
        components::{Component, Mutable},
        entities::id::EntityId,
        event::Event,
        systems::{
            commands::Commands,
            query::Query,
        },
    },
    input::{
        button_input::ButtonInput,
        mouse::MouseButton,
    },
    ui::render::interact::Hovering,
};

pub struct ClickEventPlugin<E: ClickEvent>(PhantomData<E>);

impl<E: ClickEvent> ClickEventPlugin<E> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<E: ClickEvent> Plugin for ClickEventPlugin<E> {
    fn build(self, app: &mut crate::app::App) {
        app.add_systems(
            Update,
            on_click::<E>
        );
    }
}

pub trait ClickEvent: Event + Clone + Copy {
    fn on_click(&self, _node_id: EntityId, commands: &mut Commands) {
        commands.trigger(*self);
    }
}

pub struct OnClick<E: ClickEvent>(pub E);

impl<E: ClickEvent> Component for OnClick<E> {
    type Mutability = Mutable;
}

pub fn on_click<E: ClickEvent>(
    mut commands: Commands,
    query: Query<(EntityId, &OnClick<E>, &Hovering)>,
    mouse_input: &ButtonInput<MouseButton>,
) {
    for (entity, on_click, hovering) in query.iter() {
        if hovering.0 && mouse_input.just_pressed(MouseButton::Left) {
            on_click.0.on_click(entity, &mut commands);
        }
    }
}
