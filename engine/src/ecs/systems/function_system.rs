use std::marker::PhantomData;

use crate::ecs::{change_detection::params::Mut, query::{fetch::QueryData, filter::QueryFilter, state::QueryState}, resource::Resource, systems::{commands::{Commands, command::CommandQueue}, query::Query, system::System}, worlds::World};

pub struct FunctionSystem<Marker, In, Out, F>
where
    F: SystemParamFunction<Marker>,
{
    func: F,
    name: &'static str,
    state : Option<<F::Param as SystemParamTuple>::State>,
    marker: PhantomData<fn(In) -> (Marker, Out)>,
}

impl<Marker: 'static, In: 'static, Out: 'static, F> System for FunctionSystem<Marker, In, Out, F>
where F : SystemParamFunction<Marker> {
    type In = In;

    type Out = Out;

    fn name(&self) -> &str {
        self.name
    }

    fn init(&mut self, world: &mut World) {
        self.state = Some(F::Param::init_state(world));
    }

    fn apply(&mut self, world: &mut World) {
        F::Param::apply(self.state.as_mut().unwrap(), world);
    }

    fn run(&mut self, world: &mut World) {
        
        let params = unsafe {
            let option = &mut self.state;
            F::Param::get_param( option.as_mut().unwrap(), world)
        };

        self.func.run( params);
    }
}

impl<Marker, In, Out, F> FunctionSystem<Marker, In, Out, F>
where
    F: SystemParamFunction<Marker>,
{
    pub fn new(func: F) -> Self {
        Self {
            func,
            name: "",
            state: None,
            marker: PhantomData,
        }
    }
}

pub trait SystemParamTuple {
    type State: Send + Sync + 'static;
    type Item<'w>;
    fn init_state(world: &mut World) -> Self::State;
    fn apply(state: &mut Self::State, world: &mut World) {}
    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w>;
}

pub trait SystemParamFunction<Marker>: Send + Sync + 'static {
    type Param: SystemParamTuple;

    fn run(
        &mut self,
        param_value: <Self::Param as SystemParamTuple>::Item<'_>,
    );
}

impl SystemParamTuple for () {
    type State = ();

    type Item<'w> = ();

    fn init_state(world: &mut World) -> Self::State {}

    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {}
}

impl<R : Resource + Default> SystemParamTuple for &R {
    type State = ();

    type Item<'w> = &'w R;

    fn init_state(world: &mut World) -> Self::State {
        world.init_resource::<R>();
    }
    
    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        world.get_resource::<R>().unwrap()
    }
}

impl<R : Resource + Default> SystemParamTuple for &mut R {
    type State = ();

    type Item<'w> = Mut<'w, R>;
    
    fn init_state(world: &mut World) -> Self::State {
        world.init_resource::<R>();
    }
    
    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        world.get_resource_mut::<R>().unwrap()
    }
}

impl SystemParamTuple for Commands<'_> {
    type State = CommandQueue;

    type Item<'w> = Commands<'w>;

    fn init_state(world: &mut World) -> Self::State {
        world.command_queue()
    }

    fn apply(state: &mut Self::State, world: &mut World) {
        state.apply(world);
    }

    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        Commands::from(state)
    }
}

impl<D : QueryData + 'static, F: QueryFilter + 'static> SystemParamTuple for Query<'_, '_, D, F> {
    type State = QueryState<D, F>;

    type Item<'w> = Query<'w, 'w, D, F>;

    fn init_state(world: &mut World) -> Self::State {
        QueryState::new(world)
    }

    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        state.query(world)
    }
}


impl<F> SystemParamFunction<fn() -> ()> for F
where F : Send + Sync + 'static + Fn() -> () {
    type Param = ();

    fn run(
        &mut self,
        param_value: <Self::Param as SystemParamTuple>::Item<'_>,
    ) {
        self()
    }
}

impl<P1, F> SystemParamFunction<P1> for F
where 
    F : Send + Sync + 'static + Fn(P1::Item<'_>) -> (),
    P1 : SystemParamTuple
    {
    type Param = P1;

    fn run(
        &mut self,
        param_value: <Self::Param as SystemParamTuple>::Item<'_>,
    ) {
        self(param_value)
    }
}