use engine::{ecs::{bundle::Bundle, systems::commands::Commands}, ui::{geometry::{UiRect, vh, vw}, ui_node::{AlignItems, Display, JustifyContent, Node}, widget::Text}};



fn splash_screen(mut commands : Commands) {
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
        ))
        .add_children(|commands| 
            commands.spawn_with_id((
                Node {
                    ..Default::default()
                },
                Text::new("Loading...")
            )).and()
        );

    commands.despawn(entity);
}