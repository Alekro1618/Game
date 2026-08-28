use std::marker::PhantomData;


use crate::{
    app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{
        components::{Component, Mutable, StorageType},
        entities::id::EntityId,
        event::Event,
        observer::system_param::On,
        systems::{
            commands::Commands,
            query::Query,
        },
    }, utils::resources::{GameSpeed, Time},
};

pub const TIME_EPSILON: f32 = 1e-2;

pub struct StopTimer<T, M = ()>
where
    T: Event,
    M: Sync + Send + 'static,
{
    pub owner: Option<EntityId>,
    _phantom: PhantomData<(T, M)>,
}

impl<M, T> StopTimer<T, M>
where
    T: Event,
    M: Sync + Send + 'static,
{
    pub fn new(owner: Option<EntityId>) -> Self {
        Self {
            owner,
            _phantom: PhantomData,
        }
    }
}

impl<M, T> Event for StopTimer<T, M>
where
    T: Event,
    M: Sync + Send + 'static,
{
}

pub enum OnTimerFinish {
    Remove,
    Restart,
}

pub struct Timer<T, M = ()>
where
    T: Event,
    M: Sync + Send + 'static,
{
    pub owner: Option<EntityId>,
    pub result: Option<T>,
    pub value: f32,
    pub begin: f32,
    pub end: f32,
    done: bool,
    fliped: bool,
    ignore: bool,
    on_finish: OnTimerFinish,
    _phantom: PhantomData<M>,
}

impl<M, T> Component for Timer<T, M>
where
    T: Event,
    M: Sync + Send + 'static,
{
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
}

impl<M, T> Timer<T, M>
where
    T: Event,
    M: Sync + Send + 'static,
{
    pub fn new(owner: Option<EntityId>, begin: f32, end: f32) -> Self {
        if begin < end {
            Self {
                owner,
                result: None,
                value: begin,
                begin,
                end,
                done: false,
                fliped: false,
                ignore: false,
                on_finish: OnTimerFinish::Remove,
                _phantom: PhantomData,
            }
        } else {
            Self {
                owner,
                result: None,
                value: end,
                begin,
                end,
                done: false,
                fliped: true,
                ignore: false,
                on_finish: OnTimerFinish::Remove,
                _phantom: PhantomData,
            }
        }
    }

    pub fn set_result(&mut self, result: T) {
        self.result = Some(result);
    }

    pub fn reset(&mut self) {
        if !self.fliped {
            self.value = self.begin;
        } else {
            self.value = self.end;
        }
        self.done = false;
    }

    pub fn update(&mut self, interval: f32) {
        if !self.fliped {
            self.value += interval;
            if self.end - self.value < TIME_EPSILON {
                self.value = self.end;
                self.done = true;
            }
        } else {
            if self.value - self.begin < TIME_EPSILON {
                self.value = self.begin;
                self.done = true;
            }
        }
    }

    pub fn set_on_finish(&mut self, on_finish: OnTimerFinish) {
        self.on_finish = on_finish;
    }

    pub fn ignore(&mut self) {
        self.ignore = true;
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn take_result(&mut self) -> Option<T> {
        self.result.take()
    }

    pub fn time(&self) -> f32 {
        self.value
    }
}

fn update_timers<T, M>(
    mut command: Commands,
    query: Query<(EntityId, &mut Timer<T, M>)>,
    time: &Time,
) where
    T: Event,
    M: Sync + Send + 'static,
{
    for (entity, mut timer) in query {
        timer.update(time.delta());
        if timer.done {
            if !timer.ignore {
                if let Some(result) = timer.take_result() {
                    command.trigger(result);
                }
            }

            match timer.on_finish {
                OnTimerFinish::Remove => {
                    command.despawn(entity);
                }
                OnTimerFinish::Restart => {
                    timer.reset();
                }
            }
        }
    }
}

fn on_stop_timer<T, M>(on: On<StopTimer<T, M>>, query: Query<&mut Timer<T, M>>)
where
    T: Event,
    M: Sync + Send + 'static,
{
    let stop_timer = on.event();

    let Some(mut timer) = query
        .into_iter()
        .find(|timer| timer.owner == stop_timer.owner)
    else {
        panic!("Failed to find timer matching the owner");
    };

    timer.ignore();
}

pub struct TimerPlugin<T, M = ()>(pub PhantomData<(M, T)>);

impl<M, T> TimerPlugin<T, M>
where
    T: Event,
    M: Sync + Send + 'static,
{
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<M, T> Plugin for TimerPlugin<T, M>
where
    T: Event,
    M: Sync + Send + 'static,
{
    fn build(self, app: &mut crate::app::App) {
        app.add_systems(
            ScheduleLabel::Update,
            update_timers::<T, M>,
        )
        .add_observer(
            on_stop_timer::<T, M>,
        );
    }
}
