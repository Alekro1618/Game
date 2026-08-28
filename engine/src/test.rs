use engine_macros::Component;

use crate::{
    app::{App, main_schedule::ScheduleLabel},
    ecs::{
        components::{Component, Mutable, StorageType},
        systems::{
            commands::{self, Commands},
            query::Query,
        },
    },
};

pub struct T;

impl Component for T {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    type Mutability = Mutable;
}

// #[test]
// async fn test_remove_and_insert() {
//     App::new()
//         .add_systems(ScheduleLabel::Startup, start_up)
//         .run().await;
// }

// fn start_up(mut commands : Commands) {
//     commands.spawn((T, ))
// }

// fn insert(mut commands : Commands, query : Query<>) {

// }

// fn remove(mut commands : Commands) {

// }
