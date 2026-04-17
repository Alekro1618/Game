use crate::ecs::{change_detection::tick::Tick, components::id::ComponentId, message::Message, worlds::World};


struct RegisteredMessage {
    messages_component: ComponentId,
    previously_updated: bool,
    update: unsafe fn(*mut u8),
}

#[derive(Default)]
pub struct MessageRegistry {
    pub should_update: ShouldUpdateMessages,
    message_updates: Vec<RegisteredMessage>,
}


#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShouldUpdateMessages {
    /// Without any fixed timestep, messages should always be updated each frame.
    #[default]
    Always,
    /// We need to wait until at least one pass of the fixed update schedules to update the messages.
    Waiting,
    /// At least one pass of the fixed update schedules has occurred, and the messages are ready to be updated.
    Ready,
}

impl MessageRegistry {
    pub fn register_message<T: Message>(world: &mut World) {
        // By initializing the resource here, we can be sure that it is present,
        // and receive the correct, up-to-date `ComponentId` even if it was previously removed.
        let component_id = world.init_resource::<Messages<T>>();
        let mut registry = world.get_resource_or_init::<Self>();
        registry.message_updates.push(RegisteredMessage {
            messages_component: component_id,
            previously_updated: false,
            update: |ptr| {
                // SAFETY: The resource was initialized with the type Messages<T>.
                unsafe { ptr.with_type::<Messages<T>>() }
                    .bypass_change_detection()
                    .update();
            },
        });
    }

    /// Updates all of the registered messages in the World.
    pub fn run_updates(&mut self, world: &mut World, last_change_tick: Tick) {
        for registered_message in &mut self.message_updates {
            // Bypass the type ID -> Component ID lookup with the cached component ID.
            if let Some(messages) =
                world.get_resource_mut_by_id(registered_message.messages_component)
            {
                let has_changed = messages.has_changed_since(last_change_tick);
                if registered_message.previously_updated || has_changed {
                    // SAFETY: The update function pointer is called with the resource
                    // fetched from the same component ID.
                    unsafe { (registered_message.update)(messages) };
                    // Always set to true if the messages have changed, otherwise disable running on the second invocation
                    // to wait for more changes.
                    registered_message.previously_updated =
                        has_changed || !registered_message.previously_updated;
                }
            }
        }
    }

    /// Removes a message from the world and its associated [`MessageRegistry`].
    pub fn deregister_messages<T: Message>(world: &mut World) {
        let component_id = world.init_resource::<Messages<T>>();
        let mut registry = world.get_resource_or_init::<Self>();
        registry
            .message_updates
            .retain(|e| e.messages_component != component_id);
        world.remove_resource::<Messages<T>>();
    }
}
