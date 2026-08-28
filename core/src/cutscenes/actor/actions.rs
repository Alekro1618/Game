use engine::{ animation::{NextTransition, manager::AnimationManagerPlayer, player::PlayerInstance, recording::RecordPlayer, sprite::TextureState}, camera::{Camera, MainCamera}, ecs::{entities::id::EntityId, query::filter::With, worlds::World}, render::sprite::Sprite};

use crate::{actions::action::Action, cutscenes::{RelativePosition, TransitionOption, actor::{AppearEffect, HideEffect, Pos, TransitionPos, animation::{ActorAnimationManager, ActorAnimationPlayer}}}};


#[derive(Clone)]
pub struct MoveTo(pub RelativePosition, pub TransitionOption);

impl Action for MoveTo {
    fn on_start(&mut self, agent : EntityId, world : &mut World) -> bool {
        match self.1 {
            TransitionOption::Immidiate => {
                let pos = world.get_component_mut::<Pos>(agent).unwrap();
                pos.0 = self.0.clone();
                true
            },
            TransitionOption::Smooth => {
                let pos = world.get_component::<Pos>(agent).unwrap();
                let transition = TransitionPos::from_camera(pos.0.clone(), self.0.clone(), world);
                world.insert(agent, transition);
                false
            },
        }
    }

    fn on_end(&mut self, agent : EntityId, world :  &mut World) {
        world.remove::<TransitionPos>(agent);
    }

    fn on_stop(&mut self, agent : EntityId, world :  &mut World) {
        world.remove::<TransitionPos>(agent);
    }

    fn is_finished(&self, agent : EntityId, world : &World) -> bool {
        let pos = world.get_component::<Pos>(agent).unwrap();

        pos.0 == self.0
    }
}

#[derive(Clone)]
pub struct SetState(pub Box<str>, pub TransitionOption);

impl Action for SetState {
    fn on_start(&mut self, agent : EntityId, world : &mut World) -> bool {
        world.trigger_ref(&mut NextTransition::<TextureState>::new(agent, self.0.to_string()));
        match self.1 {
            TransitionOption::Immidiate => {
                world.get_component_mut::<ActorAnimationPlayer>(agent).unwrap().player.end();
                true
            },
            TransitionOption::Smooth => {
                false
            },
        }
    }

    fn on_end(&mut self, agent : EntityId, world :  &mut World) {}

    fn on_stop(&mut self, agent : EntityId, world :  &mut World) {
        world.get_component_mut::<ActorAnimationPlayer>(agent).unwrap().player.end();
    }

    fn is_finished(&self, agent : EntityId, world : &World) -> bool {
        world.get_component::<ActorAnimationPlayer>(agent).unwrap().is_end
    }
}

#[derive(Clone)]
pub struct Hide(pub TransitionOption);

impl Action for Hide {
    fn on_start(&mut self, agent : EntityId, world : &mut World) -> bool {
        match self.0  {
            TransitionOption::Immidiate => {
                world.get_component_mut::<Sprite>(agent).unwrap().set_alpha(0.0);
                true
            },
            TransitionOption::Smooth => {
                world.insert(agent, HideEffect::new(false));
                false
            }
        }
    }

    fn on_end(&mut self, agent : EntityId, world :  &mut World) {
        world.remove::<HideEffect>(agent);
    }

    fn on_stop(&mut self, agent : EntityId, world :  &mut World) {
        world.remove::<HideEffect>(agent);
    }

    fn is_finished(&self, agent : EntityId, world : &World) -> bool {
        world.get_component::<HideEffect>(agent).unwrap().is_end
    }
}

#[derive(Clone)]
pub struct Show(pub RelativePosition, pub TransitionOption);

impl Action for Show {
    fn on_start(&mut self, agent : EntityId, world : &mut World) -> bool {
        let pos = world.get_component_mut::<Pos>(agent).unwrap();
        pos.0 = self.0.clone();

        match self.1 {
            TransitionOption::Immidiate => {
                world.get_component_mut::<Sprite>(agent).unwrap().set_alpha(1.0);
                true
            },
            TransitionOption::Smooth => {
                world.insert(agent, (AppearEffect::new()));
                false
            },
        }
    }

    fn on_end(&mut self, agent : EntityId, world :  &mut World) {
        world.remove::<AppearEffect>(agent);
    }

    fn on_stop(&mut self, agent : EntityId, world :  &mut World) {
        world.remove::<AppearEffect>(agent);
    }

    fn is_finished(&self, agent : EntityId, world : &World) -> bool {
        world.get_component::<AppearEffect>(agent).unwrap().is_end
    }
}