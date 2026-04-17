use engine::{app::{main_schedule::ScheduleLabel, *}, ecs::{name::Name, systems::{commands::Commands, query::Query}}, math::{transform::Transform, vec::Vec2}, render::{color::Color, sprite::Sprite, sprite_renderer::RenderPlugin}};

#[macroquad::main("MyGame")]
async fn main() {
    App::new()
        .add_plugin(RenderPlugin)
        .add_systems::<Commands>(ScheduleLabel::Startup, example)
        .add_systems::<Query<(&Name, &mut Transform)>>(ScheduleLabel::Update, second)
        .run()
        .await;
}


fn example(mut commands : Commands) {
    commands.spawn((Name::from("e1"), Transform::IDENTITY, Sprite::from_color(Color::RED, Vec2 { x: 100.0, y: 100.0 })));
    commands.spawn((Name::from("e2"),));
    commands.spawn((Name::from("e3"), Transform::IDENTITY));
}

fn second(query : Query<(&Name, &mut Transform)>) {
    for (_, mut transform) in query {
        transform.position += Vec2::new(10.0, 10.0);
    }
}