use engine_macros::Resource;

use crate::{app::{App, FIXED_UPDATE, main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{components::{Component, Mutable, StorageType}, resource::Resource, systems::{commands::Commands, function_system::F1}}};

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(self, app:  &mut App) {
        app.add_systems::<F1<Commands>>(ScheduleLabel::PreStartup, setup);
    }
}

fn setup(mut commands : Commands) {
    commands.insert(GameSpeed::default());
}

pub struct GameSpeed(f32);

impl Default for GameSpeed {
    fn default() -> Self {
        Self(1.0)
    }
}

impl GameSpeed {
    ///Returns actual game speed value. Use only for update each frame!
    pub fn value(&self) -> f32 {
        self.0
    }
    ///Returns game speed relative to the fixed update. Use only for fixed update!
    pub fn relative(&self) -> f32 {
        self.0 * FIXED_UPDATE
    }

    pub fn set_speed(&mut self, value : f32) {
        self.0 = value;
    }
}

impl Component for GameSpeed
{
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
}

impl Resource for GameSpeed {}