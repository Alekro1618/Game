use std::{collections::{HashMap, HashSet}, sync::atomic::{AtomicU32, Ordering}};

use engine_macros::{Component, Event, Resource};

use crate::{asset::{asset::{AssetIndex, Assets}, handle::Handle}, ecs::{change_detection::params::Mut, entities::id::EntityId, hierarchy::ChildOf, query::filter::{Added, Changed}, systems::{commands::Commands, query::Query}, worlds::World}, error::error::Error, scene::{components::DynamicSceneRoot, dynamic::DynamicScene}};

static ATOMIC : AtomicU32 = AtomicU32::new(0);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct InstanceId(u32);

#[derive(Clone, Copy, PartialEq, Event)]
pub struct SceneInstanceReady {
    pub entity: EntityId,
    pub instance_id: InstanceId,
}

impl InstanceId {
    fn new() -> Self {
        Self(ATOMIC.fetch_add(1, Ordering::Relaxed))
    }
}

struct InstanceInfo {
    /// Mapping of entities from the scene world to the instance world.
    entity_map: HashMap<EntityId, EntityId>,
    /// The parent to attach this instance to.
    parent: Option<EntityId>,
}


#[derive(Default, Resource)]
pub struct SceneSpawner {
    pub(crate) spawned_dynamic_scenes: HashMap<AssetIndex, HashSet<InstanceId>>,
    spawned_instances: HashMap<InstanceId, InstanceInfo>,
    dynamic_scenes_to_spawn: Vec<(Handle<DynamicScene>, InstanceId, Option<EntityId>)>,
    dynamic_scenes_to_despawn: Vec<AssetIndex>,
    instances_to_despawn: Vec<InstanceId>,
    instances_ready: Vec<(InstanceId, Option<EntityId>)>,
}

impl SceneSpawner {
    pub fn spawn_dynamic_as_child(
        &mut self,
        id: Handle<DynamicScene>,
        parent: EntityId,
    ) -> InstanceId {
        let instance_id = InstanceId::new();
        self.dynamic_scenes_to_spawn
            .push((id, instance_id, Some(parent)));

        instance_id
    }

    pub fn despawn_instance(&mut self, instance_id: InstanceId) {
        self.instances_to_despawn.push(instance_id);
    }

    // Immediately despawns all scenes scheduled for despawn by despawning their instances.
    pub fn despawn_queued_scenes(&mut self, world: &mut World) -> Result<(), Error> {
        let scenes_to_despawn = core::mem::take(&mut self.dynamic_scenes_to_despawn);
        for scene_handle in scenes_to_despawn {
            self.despawn_dynamic_sync(world, scene_handle)?;
        }
        Ok(())
    }

    /// Immediately despawns all scene instances scheduled for despawn.
    pub fn despawn_queued_instances(&mut self, world: &mut World) {
        let instances_to_despawn = core::mem::take(&mut self.instances_to_despawn);

        for instance_id in instances_to_despawn {
            self.despawn_instance_sync(world, &instance_id);
        }
    }

    /// Immediately spawns all scenes scheduled for spawn.
    pub fn spawn_queued_scenes(&mut self, world: &mut World) -> Result<(), Error> {
        let scenes_to_spawn = core::mem::take(&mut self.dynamic_scenes_to_spawn);

        for (handle, instance_id, parent) in scenes_to_spawn {
            match Self::spawn_dynamic_internal(world, handle.index()) {
                Ok(Some(entity_map)) => {
                    let instance_info = InstanceInfo { entity_map, parent };
                    Self::set_scene_instance_parent_sync(world, &instance_info);

                    self.spawned_instances.insert(instance_id, instance_info);
                    let spawned = self.spawned_dynamic_scenes.entry(handle.index()).or_default();
                    spawned.insert(instance_id);
                    // We trigger `SceneInstanceReady` events after processing all scenes
                    // SceneSpawner may not be available in the observer.
                    self.instances_ready.push((instance_id, parent));
                }
                Ok(None) => {
                    self.dynamic_scenes_to_spawn
                        .push((handle, instance_id, parent));
                }
                Err(err) => return Err(err),
            }
        }

        Ok(())
    }

    fn set_scene_instance_parent_sync(world: &mut World, instance: &InstanceInfo) {
        let Some(parent) = instance.parent else {
            return;
        };
        for &entity in instance.entity_map.values() {
            // Add the `ChildOf` component to the scene root, and update the `Children` component of
            // the scene parent
            if !world
                .entity(entity)
                .ok()
                .is_none_or(|entity| entity.contains::<ChildOf>())
            {
                world.add_child(parent, entity);
            }
        }
    }

    fn trigger_scene_ready_events(&mut self, world: &mut World) {
        for (instance_id, parent) in self.instances_ready.drain(..) {
            if let Some(parent) = parent {
                world.trigger_ref(&mut SceneInstanceReady {
                    instance_id,
                    entity: parent,
                });
            } else {
                world.trigger_ref(&mut SceneInstanceReady {
                    instance_id,
                    entity: EntityId::PLACEHOLDER,
                });
            }
        }
    }

    /// Immediately despawns all instances of a dynamic scene.
    pub fn despawn_dynamic_sync(
        &mut self,
        world: &mut World,
        id: impl Into<AssetIndex>,
    ) -> Result<(), Error> {
        if let Some(instance_ids) = self.spawned_dynamic_scenes.remove(&id.into()) {
            for instance_id in instance_ids {
                self.despawn_instance_sync(world, &instance_id);
            }
        }

        Ok(())
    }

    /// Immediately despawns a scene instance, removing all its entities from the world.
    pub fn despawn_instance_sync(&mut self, world: &mut World, instance_id: &InstanceId) {
        if let Some(mut instance) = self.spawned_instances.remove(instance_id) {
            Self::despawn_instance_internal(world, &mut instance);
        }
    }

    fn despawn_instance_internal(world: &mut World, instance: &mut InstanceInfo) {
        for &entity in instance.entity_map.values() {
            world.despawn(entity);
        }
        // Just make sure if we reuse `InstanceInfo` for something, we don't reuse the despawned entities.
        instance.entity_map.clear();
    }

    fn spawn_dynamic_internal(
        world: &mut World,
        id: AssetIndex,
    ) -> Result<Option<HashMap<EntityId, EntityId>>, Error> {
        world.resource_scope(|world : &mut World, scenes: &mut Assets<DynamicScene>| {
            let Some(scene) = scenes
                .get(id) else {
                    return Ok(None);
                };

            let result = scene.write_to_world(world)?;
            Ok(Some(result))
        })
    }

}

#[derive(Component)]
pub struct SceneInstance(pub(crate) InstanceId);

pub fn scene_spawner(
    mut commands: Commands,
    dynamic_scene_to_spawn: Query<
        (EntityId, &DynamicSceneRoot),
        Added<DynamicSceneRoot>
    >,
    mut scene_spawner: Mut<SceneSpawner>,
) {
    for (entity, dynamic_scene) in dynamic_scene_to_spawn {
        let new_instance = scene_spawner.spawn_dynamic_as_child(dynamic_scene.0.clone(), entity);
        commands.add(entity, SceneInstance(new_instance));
    }
}

pub fn scene_spawner_system(world: &mut World) {
    world.resource_scope(|world : &mut World, scene_spawner: &mut SceneSpawner| {
        // remove any loading instances where parent is deleted
        let is_parent_alive = |parent: &Option<EntityId>| {
            parent
                .map(|parent| world.get_entity(parent).is_ok())
                .unwrap_or(true) // If we don't have a parent, then consider the parent alive.
        };
        
        scene_spawner
            .dynamic_scenes_to_spawn
            .retain(|(_, _, parent)| is_parent_alive(parent));
        
        scene_spawner.despawn_queued_scenes(world).unwrap();
        scene_spawner.despawn_queued_instances(world);
        scene_spawner
            .spawn_queued_scenes(world)
            .unwrap_or_else(|err| panic!("{}", err));

        scene_spawner.trigger_scene_ready_events(world);
    });
}
