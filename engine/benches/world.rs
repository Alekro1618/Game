use criterion::{Criterion, black_box, criterion_group, criterion_main};

use engine::ecs::{
    components::{Component, Mutable},
    entities::{Entity, id::EntityId},
    worlds::World,
};

fn world_get_component_benchmark(c: &mut Criterion) {
    let world = create_world();
    c.bench_function("world get_component", |b| {
        b.iter(|| get_component(&world, black_box(EntityId::new(2_000, 0))))
    });
}

fn world_query_data(c: &mut Criterion) {
    let mut world = create_world();
    c.bench_function("world query_components", |b| {
        b.iter(|| query_components(&mut world));
    });
}

fn world_get_entity_mut(c: &mut Criterion) {
    let mut world = create_world();
    c.bench_function("world get_entity_mut", |b| {
        b.iter(|| get_entity(&mut world, black_box(EntityId::new(2_000, 0))))
    });
}

pub struct TestData {
    a: u64,
    text: String,
}

impl Component for TestData {
    type Mutability = Mutable;
}

fn create_world() -> World {
    let mut world = World::new();
    for i in 1..10_000 {
        world.spawn(TestData {
            a: i,
            text: i.to_string(),
        });
    }
    world
}

fn get_component(world: &World, entity: EntityId) {
    let _ = world.get_component::<TestData>(entity).unwrap();
}

fn query_components(world: &mut World) {
    let _ = world.query::<&TestData>();
}

fn get_entity(world: &mut World, entity: EntityId) {
    let _ = world.entity(entity).get_mut::<TestData>().unwrap();
}

criterion_group!(
    benches,
    world_get_component_benchmark,
    world_query_data,
    world_get_entity_mut
);
criterion_main!(benches);
