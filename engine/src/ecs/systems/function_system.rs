use std::marker::PhantomData;

use crate::ecs::{change_detection::params::Mut, query::{fetch::QueryData, filter::QueryFilter, state::QueryState}, resource::Resource, systems::{commands::{Commands, command::CommandQueue}, query::Query, system::System}, worlds::World};

pub struct FunctionSystem<Marker, In, Out, F>
where
    F: SystemParamFunction<Marker>,
{
    func: F,
    name: &'static str,
    pub state : Option<<F::Param as SystemParamTuple>::State>,
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

    pub fn set_state(&mut self, state : Option<<F::Param as SystemParamTuple>::State>) {
        self.state = state;
    }
}

pub trait SystemParamTuple {
    type State: Send + Sync + 'static;
    type Item<'w>;
    fn init_state(world: &mut World) -> Self::State;
    fn apply(_state: &mut Self::State, _world: &mut World) {}
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

    fn init_state(_world: &mut World) -> Self::State {}

    unsafe fn get_param<'w>(_state: &'w mut Self::State, _world: &'w mut World) -> Self::Item<'w> {}
}

impl<P1, P2> SystemParamTuple for (P1, P2)
where 
    P1: SystemParamTuple,
    P2: SystemParamTuple {
    type State = (P1::State, P2::State);

    type Item<'w> = (P1::Item<'w>, P2::Item<'w>);

    fn init_state(world: &mut World) -> Self::State {
        (P1::init_state(world), P2::init_state(world))
    }

    fn apply(state: &mut Self::State, world: &mut World) {
        P1::apply(&mut state.0, world);
        P2::apply(&mut state.1, world);
    }

    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        let world_ptr: *mut World = world;
        unsafe {
            (
                P1::get_param(&mut state.0, world_ptr.as_mut().unwrap()),
                P2::get_param(&mut state.1, world_ptr.as_mut().unwrap())
            )
        }
    }
}

impl<P1, P2, P3> SystemParamTuple for (P1, P2, P3)
where 
    P1: SystemParamTuple,
    P2: SystemParamTuple,
    P3: SystemParamTuple, {
    type State = (P1::State, P2::State, P3::State);

    type Item<'w> = (P1::Item<'w>, P2::Item<'w>, P3::Item<'w>);

    fn init_state(world: &mut World) -> Self::State {
        (P1::init_state(world), P2::init_state(world), P3::init_state(world))
    }

    fn apply(state: &mut Self::State, world: &mut World) {
        P1::apply(&mut state.0, world);
        P2::apply(&mut state.1, world);
        P3::apply(&mut state.2, world);
    }

    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        let world_ptr: *mut World = world;
        unsafe {
            (
                P1::get_param(&mut state.0, world_ptr.as_mut().unwrap()),
                P2::get_param(&mut state.1, world_ptr.as_mut().unwrap()),
                P3::get_param(&mut state.2, world_ptr.as_mut().unwrap())
            )
        }
    }
}

impl<P1, P2, P3, P4> SystemParamTuple for (P1, P2, P3, P4)
where 
    P1: SystemParamTuple,
    P2: SystemParamTuple,
    P3: SystemParamTuple,
    P4: SystemParamTuple, {
    type State = (P1::State, P2::State, P3::State, P4::State);

    type Item<'w> = (P1::Item<'w>, P2::Item<'w>, P3::Item<'w>, P4::Item<'w>);

    fn init_state(world: &mut World) -> Self::State {
        (P1::init_state(world), P2::init_state(world), P3::init_state(world), P4::init_state(world))
    }

    fn apply(state: &mut Self::State, world: &mut World) {
        P1::apply(&mut state.0, world);
        P2::apply(&mut state.1, world);
        P3::apply(&mut state.2, world);
        P4::apply(&mut state.3, world);
    }

    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        let world_ptr: *mut World = world;
        unsafe {
            (
                P1::get_param(&mut state.0, world_ptr.as_mut().unwrap()),
                P2::get_param(&mut state.1, world_ptr.as_mut().unwrap()),
                P3::get_param(&mut state.2, world_ptr.as_mut().unwrap()),
                P4::get_param(&mut state.3, world_ptr.as_mut().unwrap()),
            )
        }
    }
}

impl<P1, P2, P3, P4, P5> SystemParamTuple for (P1, P2, P3, P4, P5)
where 
    P1: SystemParamTuple,
    P2: SystemParamTuple,
    P3: SystemParamTuple,
    P4: SystemParamTuple,
    P5: SystemParamTuple, {
    type State = (P1::State, P2::State, P3::State, P4::State, P5::State);

    type Item<'w> = (P1::Item<'w>, P2::Item<'w>, P3::Item<'w>, P4::Item<'w>, P5::Item<'w>);

    fn init_state(world: &mut World) -> Self::State {
        (P1::init_state(world), P2::init_state(world), P3::init_state(world), P4::init_state(world), P5::init_state(world))
    }

    fn apply(state: &mut Self::State, world: &mut World) {
        P1::apply(&mut state.0, world);
        P2::apply(&mut state.1, world);
        P3::apply(&mut state.2, world);
        P4::apply(&mut state.3, world);
        P5::apply(&mut state.4, world);
    }


    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        let world_ptr: *mut World = world;
        unsafe {
            (
                P1::get_param(&mut state.0, world_ptr.as_mut().unwrap()),
                P2::get_param(&mut state.1, world_ptr.as_mut().unwrap()),
                P3::get_param(&mut state.2, world_ptr.as_mut().unwrap()),
                P4::get_param(&mut state.3, world_ptr.as_mut().unwrap()),
                P5::get_param(&mut state.4, world_ptr.as_mut().unwrap())
            )
        }
    }
}

impl<P1, P2, P3, P4, P5, P6> SystemParamTuple for (P1, P2, P3, P4, P5, P6)
where 
    P1: SystemParamTuple,
    P2: SystemParamTuple,
    P3: SystemParamTuple,
    P4: SystemParamTuple,
    P5: SystemParamTuple,
    P6: SystemParamTuple {
    type State = (P1::State, P2::State, P3::State, P4::State, P5::State, P6::State);

    type Item<'w> = (P1::Item<'w>, P2::Item<'w>, P3::Item<'w>, P4::Item<'w>, P5::Item<'w>, P6::Item<'w>);

    fn init_state(world: &mut World) -> Self::State {
        (P1::init_state(world), P2::init_state(world), P3::init_state(world), P4::init_state(world), P5::init_state(world), P6::init_state(world))
    }

    fn apply(state: &mut Self::State, world: &mut World) {
        P1::apply(&mut state.0, world);
        P2::apply(&mut state.1, world);
        P3::apply(&mut state.2, world);
        P4::apply(&mut state.3, world);
        P5::apply(&mut state.4, world);
        P6::apply(&mut state.5, world);
    }

    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        let world_ptr: *mut World = world;
        unsafe {
            (
                P1::get_param(&mut state.0, world_ptr.as_mut().unwrap()),
                P2::get_param(&mut state.1, world_ptr.as_mut().unwrap()),
                P3::get_param(&mut state.2, world_ptr.as_mut().unwrap()),
                P4::get_param(&mut state.3, world_ptr.as_mut().unwrap()),
                P5::get_param(&mut state.4, world_ptr.as_mut().unwrap()),
                P6::get_param(&mut state.5, world_ptr.as_mut().unwrap())
            )
        }
    }
}

impl<R : Resource + Default> SystemParamTuple for &R {
    type State = ();

    type Item<'w> = &'w R;

    fn init_state(world: &mut World) -> Self::State {
        world.init_resource::<R>();
    }
    
    unsafe fn get_param<'w>(_state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        world.get_resource::<R>().unwrap()
    }
}

impl<R : Resource + Default> SystemParamTuple for &mut R {
    type State = ();

    type Item<'w> = Mut<'w, R>;
    
    fn init_state(world: &mut World) -> Self::State {
        world.init_resource::<R>();
    }
    
    unsafe fn get_param<'w>(_state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
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

    unsafe fn get_param<'w>(state: &'w mut Self::State, _world: &'w mut World) -> Self::Item<'w> {
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

pub struct F0;
pub struct F1<P1>(PhantomData<P1>);
pub struct F2<P1, P2>(PhantomData<(P1, P2)>);
pub struct F3<P1, P2, P3>(PhantomData<(P1, P2, P3)>);
pub struct F4<P1, P2, P3, P4>(PhantomData<(P1, P2, P3, P4)>);
pub struct F5<P1, P2, P3, P4, P5>(PhantomData<(P1, P2, P3, P4, P5)>);
pub struct F6<P1, P2, P3, P4, P5, P6>(PhantomData<(P1, P2, P3, P4, P4, P5, P6)>);


impl<F> SystemParamFunction<F0> for F
where F : Send + Sync + 'static + Fn() -> () {
    type Param = ();

    fn run(
        &mut self,
        _param_value: <Self::Param as SystemParamTuple>::Item<'_>,
    ) {
        self()
    }
}

impl<P1, F> SystemParamFunction<F1<P1>> for F
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

impl<P1, P2, F> SystemParamFunction<F2<P1, P2>> for F
where 
    F : Send + Sync + 'static + Fn(P1::Item<'_>, P2::Item<'_>) -> (),
    P1 : SystemParamTuple,
    P2 : SystemParamTuple
    {
    type Param = (P1, P2);

    fn run(
        &mut self,
        param_value: <Self::Param as SystemParamTuple>::Item<'_>,
    ) {
        self(param_value.0, param_value.1)
    }
}

impl<P1, P2, P3, F> SystemParamFunction<F3<P1, P2, P3>> for F
where 
    F : Send + Sync + 'static + Fn(P1::Item<'_>, P2::Item<'_>, P3::Item<'_>) -> (),
    P1 : SystemParamTuple,
    P2 : SystemParamTuple,
    P3 : SystemParamTuple
    {
    type Param = (P1, P2, P3);

    fn run(
        &mut self,
        param_value: <Self::Param as SystemParamTuple>::Item<'_>,
    ) {
        self(param_value.0, param_value.1, param_value.2)
    }
}

impl<P1, P2, P3, P4, F> SystemParamFunction<F4<P1, P2, P3, P4>> for F
where 
    F : Send + Sync + 'static + Fn(P1::Item<'_>, P2::Item<'_>, P3::Item<'_>, P4::Item<'_>) -> (),
    P1 : SystemParamTuple,
    P2 : SystemParamTuple,
    P3 : SystemParamTuple,
    P4 : SystemParamTuple,
    {
    type Param = (P1, P2, P3, P4);

    fn run(
        &mut self,
        param_value: <Self::Param as SystemParamTuple>::Item<'_>,
    ) {
        self(param_value.0, param_value.1, param_value.2, param_value.3)
    }
}

impl<P1, P2, P3, P4, P5, F> SystemParamFunction<F5<P1, P2, P3, P4, P5>> for F
where 
    F : Send + Sync + 'static + Fn(P1::Item<'_>, P2::Item<'_>, P3::Item<'_>, P4::Item<'_>, P5::Item<'_>) -> (),
    P1 : SystemParamTuple,
    P2 : SystemParamTuple,
    P3 : SystemParamTuple,
    P4 : SystemParamTuple,
    P5 : SystemParamTuple,
    {
    type Param = (P1, P2, P3, P4, P5);

    fn run(
        &mut self,
        param_value: <Self::Param as SystemParamTuple>::Item<'_>,
    ) {
        self(param_value.0, param_value.1, param_value.2, param_value.3, param_value.4)
    }
}

impl<P1, P2, P3, P4, P5, P6, F> SystemParamFunction<F6<P1, P2, P3, P4, P5, P6>> for F
where 
    F : Send + Sync + 'static + Fn(P1::Item<'_>, P2::Item<'_>, P3::Item<'_>, P4::Item<'_>, P5::Item<'_>, P6::Item<'_>) -> (),
    P1 : SystemParamTuple,
    P2 : SystemParamTuple,
    P3 : SystemParamTuple,
    P4 : SystemParamTuple,
    P5 : SystemParamTuple,
    P6 : SystemParamTuple,
    {
    type Param = (P1, P2, P3, P4, P5, P6);

    fn run(
        &mut self,
        param_value: <Self::Param as SystemParamTuple>::Item<'_>,
    ) {
        self(param_value.0, param_value.1, param_value.2, param_value.3, param_value.4, param_value.5)
    }
}