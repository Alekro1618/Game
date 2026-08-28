use std::{any::type_name, marker::PhantomData};

use crate::ecs::{
    change_detection::params::Mut, query::{fetch::QueryData, filter::QueryFilter, state::QueryState}, resource::Resource, systems::{
        commands::{Commands, command::CommandQueue}, query::Query, system::{System, SystemInput},
    }, worlds::World,
};

pub struct FunctionSystem<Marker, In, Out, F>
where
    F: SystemParamFunction<Marker, In = In, Out = Out>,
{
    func: F,
    name: &'static str,
    pub state: Option<<F::Param as SystemParamTuple>::State>,
    marker: PhantomData<fn(In) -> (Marker, Out)>,
}

impl<Marker: 'static, In: SystemInput + 'static, Out: 'static, F> System for FunctionSystem<Marker, In, Out, F>
where
    F: SystemParamFunction<Marker, In = In, Out = Out>,
{
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

    fn run(&mut self, input : Self::In, world: &mut World) -> Self::Out {
        let params = unsafe {
            let option = &mut self.state;
            if let Some(_d) = option.as_mut() {
                F::Param::get_param(option.as_mut().unwrap(), world)
            } else {
                panic!("here")
            }
        };

        self.func.run(input, params)
    }
}

impl<Marker, In, Out, F> FunctionSystem<Marker, In, Out, F>
where
    F: SystemParamFunction<Marker, In = In, Out = Out>,
{
    pub fn new(func: F) -> Self {
        Self {
            func,
            name: F::name(),
            state: None,
            marker: PhantomData,
        }
    }

    pub fn set_state(&mut self, state: Option<<F::Param as SystemParamTuple>::State>) {
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
    type In : SystemInput;
    type Out;
    type Param: SystemParamTuple;
    fn name() -> &'static str {
        ""
    }

    fn run(
        &mut self, 
        input: Self::In, 
        param_value: <Self::Param as SystemParamTuple>::Item<'_>
    ) -> Self::Out;
}

impl SystemParamTuple for () {
    type State = ();

    type Item<'w> = ();

    fn init_state(_world: &mut World) -> Self::State {}

    unsafe fn get_param<'w>(_state: &'w mut Self::State, _world: &'w mut World) -> Self::Item<'w> {}
}

impl<P1> SystemParamTuple for (P1,)
where
    P1: SystemParamTuple
{
    type State = (P1::State,);

    type Item<'w> = (P1::Item<'w>,);

    fn init_state(world: &mut World) -> Self::State {
        (P1::init_state(world), )
    }

    fn apply(state: &mut Self::State, world: &mut World) {
        P1::apply(&mut state.0, world);
    }

    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        let world_ptr: *mut World = world;
        unsafe {
            (
                P1::get_param(&mut state.0, world_ptr.as_mut().unwrap()),
            )
        }
    }
}

impl<P1, P2> SystemParamTuple for (P1, P2)
where
    P1: SystemParamTuple,
    P2: SystemParamTuple,
{
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
                P2::get_param(&mut state.1, world_ptr.as_mut().unwrap()),
            )
        }
    }
}

impl<P1, P2, P3> SystemParamTuple for (P1, P2, P3)
where
    P1: SystemParamTuple,
    P2: SystemParamTuple,
    P3: SystemParamTuple,
{
    type State = (P1::State, P2::State, P3::State);

    type Item<'w> = (P1::Item<'w>, P2::Item<'w>, P3::Item<'w>);

    fn init_state(world: &mut World) -> Self::State {
        (
            P1::init_state(world),
            P2::init_state(world),
            P3::init_state(world),
        )
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
                P3::get_param(&mut state.2, world_ptr.as_mut().unwrap()),
            )
        }
    }
}

impl<P1, P2, P3, P4> SystemParamTuple for (P1, P2, P3, P4)
where
    P1: SystemParamTuple,
    P2: SystemParamTuple,
    P3: SystemParamTuple,
    P4: SystemParamTuple,
{
    type State = (P1::State, P2::State, P3::State, P4::State);

    type Item<'w> = (P1::Item<'w>, P2::Item<'w>, P3::Item<'w>, P4::Item<'w>);

    fn init_state(world: &mut World) -> Self::State {
        (
            P1::init_state(world),
            P2::init_state(world),
            P3::init_state(world),
            P4::init_state(world),
        )
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
    P5: SystemParamTuple,
{
    type State = (P1::State, P2::State, P3::State, P4::State, P5::State);

    type Item<'w> = (
        P1::Item<'w>,
        P2::Item<'w>,
        P3::Item<'w>,
        P4::Item<'w>,
        P5::Item<'w>,
    );

    fn init_state(world: &mut World) -> Self::State {
        (
            P1::init_state(world),
            P2::init_state(world),
            P3::init_state(world),
            P4::init_state(world),
            P5::init_state(world),
        )
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
                P5::get_param(&mut state.4, world_ptr.as_mut().unwrap()),
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
    P6: SystemParamTuple,
{
    type State = (
        P1::State,
        P2::State,
        P3::State,
        P4::State,
        P5::State,
        P6::State,
    );

    type Item<'w> = (
        P1::Item<'w>,
        P2::Item<'w>,
        P3::Item<'w>,
        P4::Item<'w>,
        P5::Item<'w>,
        P6::Item<'w>,
    );

    fn init_state(world: &mut World) -> Self::State {
        (
            P1::init_state(world),
            P2::init_state(world),
            P3::init_state(world),
            P4::init_state(world),
            P5::init_state(world),
            P6::init_state(world),
        )
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
                P6::get_param(&mut state.5, world_ptr.as_mut().unwrap()),
            )
        }
    }
}

impl SystemParamTuple for &World {
    type State = ();

    type Item<'w> = &'w World;

    fn init_state(_world: &mut World) -> Self::State {}

    unsafe fn get_param<'w>(_state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        world
    }
}

impl SystemParamTuple for Mut<'_, World> {
    type State = ();

    type Item<'w> = Mut<'w, World>;

    fn init_state(_world: &mut World) -> Self::State {}

    unsafe fn get_param<'w>(_state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        Mut { value: world }
    }
}

impl<R: Resource + Default> SystemParamTuple for &R {
    type State = ();

    type Item<'w> = &'w R;

    fn init_state(world: &mut World) -> Self::State {
        world.init_resource::<R>();
    }

    unsafe fn get_param<'w>(_state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        world.get_resource::<R>().unwrap()
    }
}

impl<R: Resource + Default> SystemParamTuple for Mut<'_, R> {
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

impl<D: QueryData + 'static, F: QueryFilter + 'static> SystemParamTuple for Query<'_, '_, D, F> {
    type State = QueryState<D, F>;

    type Item<'w> = Query<'w, 'w, D, F>;

    fn init_state(world: &mut World) -> Self::State {
        QueryState::new(world)
    }

    unsafe fn get_param<'w>(state: &'w mut Self::State, world: &'w mut World) -> Self::Item<'w> {
        state.query(world)
    }
}

pub struct HasSystemInput;

macro_rules! impl_system_function {
    ($($param: ident),*) => {
        impl<$($param: SystemParamTuple),*, Func, Out> SystemParamFunction<fn($($param,)+) -> Out> for Func
        where
            Func: Send + Sync + 'static,
            for<'w> &'w mut Func: FnMut($($param,)+) -> Out +
            FnMut($(<$param as SystemParamTuple>::Item<'_>,)+) -> Out,
            Out: 'static
        {
            type In = ();
            type Out = Out;
            type Param = ($($param,)*);

            fn name() -> &'static str {
                type_name::<Func>()
            }

            fn run(&mut self, _input: Self::In, param_value: <Self::Param as SystemParamTuple>::Item<'_>) -> Self::Out {
                fn call_inner<Out, $($param,)*>(
                    mut f: impl FnMut($($param,)*)->Out,
                    $($param: $param,)*
                )->Out{
                    f($($param,)*)
                }
                let ($($param,)+) = param_value;
                call_inner(self, $($param),+)
            }
        }

        impl<$($param: SystemParamTuple),*, Func, In, Out> SystemParamFunction<(HasSystemInput, fn(In, $($param,)+) -> Out)> for Func
        where
            Func: Send + Sync + 'static,
            for<'w> &'w mut Func: FnMut(In, $($param,)+) -> Out +
            FnMut(In, $(<$param as SystemParamTuple>::Item<'_>,)+) -> Out,
            In : SystemInput,
            Out: 'static
        {
            type In = In;
            type Out = Out;
            type Param = ($($param,)*);

            fn name() -> &'static str {
                type_name::<Func>()
            }
         
            fn run(&mut self, input: Self::In, param_value:  <Self::Param as SystemParamTuple>::Item<'_>) -> Out {
                fn call_inner<In : SystemInput, Out, $($param,)*>(
                    _: PhantomData<In>,
                    mut f: impl FnMut(In, $($param,)*)->Out,
                    input: In,
                    $($param: $param,)*
                )->Out{
                    f(input, $($param,)*)
                }
                let ($($param,)*) = param_value;
                call_inner(PhantomData::<In>, self, input, $($param),*)
            }
        }
    };
}


impl<Func, Out> SystemParamFunction<fn() -> Out> for Func
where
    Func: Send + Sync + 'static,
    for<'w> &'w mut Func: FnMut() -> Out,
    Out: 'static
{
    type In = ();
    type Out = Out;
    type Param = ();

    fn name() -> &'static str {
        type_name::<Func>()
    }

    fn run(&mut self, _input : Self::In, _param_value: <Self::Param as SystemParamTuple>::Item<'_>) -> Self::Out {
        fn call_inner<Out>(
            mut f: impl FnMut()->Out
        )->Out{
            f()
        }
        call_inner(self)
    }
}

impl<In : SystemInput, Out, Func> SystemParamFunction<(HasSystemInput, fn(In) -> Out)> for Func
where
    Func: Send + Sync + 'static,
    for<'w> &'w mut Func: FnMut(In) -> Out,
    Out: 'static
{
    type In = In;
    type Out = Out;
    type Param = ();

    fn name() -> &'static str {
        type_name::<Func>()
    }

    fn run(&mut self, input : Self::In, _param_value: <Self::Param as SystemParamTuple>::Item<'_>) -> Self::Out {
        fn call_inner<In : SystemInput, Out>(
            _: PhantomData<In>,
            mut f: impl FnMut(In)->Out,
            input: In
        ) -> Out {
            f(input)
        }
        call_inner(PhantomData::<In>, self, input)
    }
}


impl_system_function!(A);
impl_system_function!(A, B);
impl_system_function!(A, B, C);
impl_system_function!(A, B, C, D);
impl_system_function!(A, B, C, D, E);
impl_system_function!(A, B, C, D, E, F);


#[cfg(test)]
mod test {
    use std::marker::PhantomData;

use crate::ecs::{event::Event, observer::system_param::On, schedule::{config::IntoSchedule, schedule_system::ScheduleSystem}, systems::{IntoSystem, function_system::{SystemParamFunction, SystemParamTuple}, observer_system::IntoObserverSystem}};


    pub struct Test;

    impl Event for Test{}

    pub struct SomeWraper<'w, M>(PhantomData<&'w M>);

    macro_rules! impl_system_param {
        ($param: ident) => {
            pub struct $param;

            impl SystemParamTuple for $param {
                type State = ();
            
                type Item<'w> = $param;
            
                fn init_state(_world: &mut crate::ecs::worlds::World) -> Self::State {
                    ()
                }
            
                unsafe fn get_param<'w>(_state: &'w mut Self::State, _world: &'w mut crate::ecs::worlds::World) -> Self::Item<'w> {
                    todo!()
                }
            }
        };
    }

    impl_system_param!(A);
    impl_system_param!(B);
    impl_system_param!(C);
    impl_system_param!(D);
    impl_system_param!(E);
    impl_system_param!(F);

    macro_rules! empty_fun  {
        ($name:ident, $params:tt) => {
            fn $name $params {
                
            }
        };
    }

    empty_fun!(no_params, ());
    empty_fun!(one_param, (a: A));
    empty_fun!(two_params, (a: A, b: B));
    empty_fun!(three_params, (a : A, b: B, c: C));
    empty_fun!(four_params, (a : A, b: B, c: C, d: D));
    empty_fun!(five_params, (a :A, b: B, c: C, d: D, e : E));
    empty_fun!(six_params, (a : A, b: B, c: C, d: D, e : E, f : F));
    empty_fun!(no_params_input, (on : On<Test>));
    empty_fun!(one_param_input, (on : On<Test>, a: A));
    empty_fun!(two_params_input, (on : On<Test>, a: A, b: B));
    empty_fun!(three_params_input, (on : On<Test>, a : A, b: B, c: C));
    empty_fun!(four_params_input, (on : On<Test>, a : A, b: B, c: C, d: D));
    empty_fun!(five_params_input, (on : On<Test>, a : A, b: B, c: C, d: D, e : E));
    empty_fun!(six_params_input, (on : On<Test>, a : A, b: B, c: C, d: D, e : E, f : F));

    #[test]
    fn test_if_functions_implement_into_system() {
        system_param_function(no_params);
        system_param_function(one_param);
        system_param_function(two_params);
        system_param_function(three_params);
        system_param_function(four_params);
        system_param_function(five_params);
        system_param_function(six_params);
    }

    #[test]
    fn test_if_functions_implement_into_observer() {
        observer(no_params_input);
        observer(one_param_input);
        observer(two_params_input);
        observer(three_params_input);
        observer(four_params_input);
        observer(five_params_input);
        observer(six_params_input);
    }


    fn system_param_function<Marker>(fun : impl IntoSystem<(), (), Marker>) {
        let _ = fun;
    }

    fn observer<E : Event, Marker>(fun : impl IntoObserverSystem<E, Marker>) {
        let _ = fun;
    }
}