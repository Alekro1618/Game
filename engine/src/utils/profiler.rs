use std::time::Instant;

use crate::{
    app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{change_detection::params::Mut, components::{Component, Mutable}, resource::Resource},
};

pub struct UpdateTimeDifference{
    instant : Instant,
    difference : f32
}

impl Default for UpdateTimeDifference {
    fn default() -> Self {
        Self { 
            instant: Instant::now(), 
            difference: 0.0
        }
    }
}

impl Component for UpdateTimeDifference {
    type Mutability = Mutable;
}

impl Resource for UpdateTimeDifference {}

pub struct ProfilerPlugin;

impl Plugin for ProfilerPlugin {
    fn build(self, app: &mut crate::app::App) {
        app
            .add_systems(ScheduleLabel::PreUpdate, pre_update)
            .add_systems(ScheduleLabel::PostUpdate, post_update);
    }
}

fn pre_update(mut utd : Mut<UpdateTimeDifference>) {
    utd.instant = Instant::now();
}

fn post_update(mut utd : Mut<UpdateTimeDifference>) {
    utd.difference = utd.instant.elapsed().as_secs_f64() as f32;
    utd.instant = Instant::now();
    println!("{}", utd.difference);
}
