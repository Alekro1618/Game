
use std::time::Instant;

use crate::{
    app::{App, FIXED_UPDATE, main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{
        change_detection::params::Mut, components::{Component, Mutable, StorageType}, resource::Resource, systems::commands::Commands,
    },
};

pub struct ResourcePlugin;

impl Plugin for ResourcePlugin {
    fn build(self, app: &mut App) {
        app.add_systems(ScheduleLabel::PreStartup, setup)
            .add_systems(ScheduleLabel::Update, calculate_time);
    }
}

fn setup(mut commands: Commands) {
    commands.insert(GameSpeed::default());
    commands.insert(Time::default());
}

pub struct GameSpeed(f32);

impl Default for GameSpeed {
    fn default() -> Self {
        Self(1.0)
    }
}

impl GameSpeed {
    ///Returns actual game speed value.
    pub fn value(&self) -> f32 {
        self.0
    }
    ///Returns game speed relative to the fixed update. Use only for fixed update!
    pub fn relative(&self) -> f32 {
        self.0 * FIXED_UPDATE
    }

    pub fn set_speed(&mut self, value: f32) {
        self.0 = value;
    }
}

impl Component for GameSpeed {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
}

impl Resource for GameSpeed {}


pub struct Time {
    instant : Instant,
    current : f32,
    last : f32
}

impl Default for Time {
    fn default() -> Self {
        Self { 
            instant: Instant::now(),
            current: 0.0, 
            last: 0.0 }
    }
}

impl Time {
    pub fn current(&self) -> f32 {
        self.current 
    }

    pub fn delta(&self) -> f32 {
        self.current - self.last
    } 
}
impl Component for Time {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
}

impl Resource for Time {}

fn calculate_time(mut time:  Mut<Time>) {
    let elapsed = time.instant.elapsed().as_secs_f32();
    
    time.last = time.current;
    time.current = elapsed;
}