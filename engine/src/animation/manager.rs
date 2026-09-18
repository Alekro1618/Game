use std::collections::{HashMap, hash_map::Entry};

use serde::{Deserialize, Serialize};

use crate::{
    animation::{
        Animatable, player::Player,
    }, asset::{AssetPath, asset::Asset, handle::Handle}, ecs::components::{Component, Mutable},
};

#[derive(Serialize, Deserialize)]
pub enum TransitionStyle {
    Instant,
    OnEnd,
}

#[derive(Serialize, Deserialize)]
pub struct Transition {
    from: String,
    to: String,
    style: TransitionStyle,
}

impl Transition {
    pub fn new<S1: ToString, S2: ToString>(from: S1, to: S2, style: TransitionStyle) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
            style,
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct TransitionMap {
    map: HashMap<String, usize>,
    next: Option<String>,
}

impl TransitionMap {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            next: None,
        }
    }

    pub fn insert(&mut self, to : String, position : usize) {
        self.map.insert(to, position);
    }

    pub fn set_next(&mut self, next : String) {
        self.next = Some(next)
    }
}

#[derive(Serialize, Deserialize)]
pub struct AnimationManagerErased {
    pub recordings : HashMap<String, AssetPath>,
    pub transitions : Vec<Transition>
}

pub struct AnimationManager<A : Send + Sync + 'static, P : Player<State = A>> {
    recordings_name_to_pos : HashMap<String, usize>,
    recordings : Vec<Handle<P::Recording>>,
    transitions: Vec<Transition>,
    name_to_transition: HashMap<String, TransitionMap>,
}
impl<A: Send + Sync + 'static,  P : Player<State = A>> Asset for AnimationManager<A, P> {}

impl<A : Send + Sync + 'static, P : Player<State = A>> AnimationManager<A, P> {
    pub fn new(transitions: Vec<Transition>, recordings : HashMap<String, Handle<P::Recording>>) -> Self {
        let mut name_to_transition: HashMap<String, TransitionMap> = HashMap::new();

        for (position, transition) in transitions.iter().enumerate() {
            name_to_transition
                .entry(transition.from.clone())
                .or_insert_with(TransitionMap::new)
                .insert(transition.to.clone(), position);
        }

        let mut recording_list = Vec::new();
        let mut name_to_pos = HashMap::new();
        
        for (pos, (name, handle)) in recordings.into_iter().enumerate() {
            recording_list.push(handle);
            name_to_pos.insert(name, pos);
        }

        Self {
            recordings: recording_list,
            recordings_name_to_pos: name_to_pos,
            transitions: transitions,
            name_to_transition: name_to_transition,
        }
    }

    pub fn add_transition(&mut self, transition: Transition) {
        let len = self.transitions.len();
        let entry = self.name_to_transition.entry(transition.from.clone());

        match entry {
            Entry::Occupied(mut occupied_entry) => {
                occupied_entry
                    .get_mut()
                    .map
                    .insert(transition.to.clone(), len);
            }
            Entry::Vacant(vacant_entry) => {
                let mut transition_map = TransitionMap::new();
                transition_map.map.insert(transition.to.clone(), len);
                vacant_entry.insert(transition_map);
            }
        }

        self.transitions.push(transition);
    }

    pub fn set_transition_next(&mut self, from: &str, to: &str) -> Option<()> {
        let transition_map = self.transition_map_mut(from)?;

        if !transition_map.map.contains_key(to) {
            return None;
        }

        transition_map.next = Some(to.to_string());

        Some(())
    }

    pub fn find_transition(&self, from: &str, to: &str) -> Option<usize> {
        self.name_to_transition.get(from)?.map.get(to).cloned()
    }

    pub fn transition_map(&self, from: &str) -> Option<&TransitionMap> {
        self.name_to_transition.get(from)
    }

    pub fn transition_map_mut(&mut self, from: &str) -> Option<&mut TransitionMap> {
        self.name_to_transition.get_mut(from)
    }

    pub fn next_transition(&self, from: &str) -> Option<String> {
        self.transition_map(from)?.next.clone()
    }

    pub fn find_recording(&self, name : &str) -> Option<usize> {
        self.recordings_name_to_pos.get(name).map(|d| *d)
    }
}

pub struct AnimationManagerPlayer<A: Animatable, P : Player<State = A>> {
    animation_manager: Handle<AnimationManager<A, P>>,
    pub current_recording_name: String,
    current_transition: Option<usize>,
}

impl<A: Animatable, P : Player<State = A>> Component for AnimationManagerPlayer<A, P> {
    type Mutability = Mutable;
}

impl<A: Animatable, P : Player<State = A>> AnimationManagerPlayer<A, P> {
    pub fn new<S: ToString>(animation_manager: Handle<AnimationManager<A, P>>, start: S) -> Self {
        let current_recording_name = start.to_string();

        Self {
            animation_manager,
            current_recording_name,
            current_transition: None,
        }
    }

    pub fn handle(
        &self
    ) -> &Handle<AnimationManager<A, P>> {
        &self.animation_manager
    }

    pub fn pick_transition(
        &mut self,
        animation_manager: &AnimationManager<A, P>,
        from: &str,
        to: &str,
    ) {
        let Some(pos) = animation_manager.find_transition(from, to) else {
            println!("No transition in animation manager from {} to {}", from, to);
            return;
        };

        self.current_transition = Some(pos);
    }

    pub fn pick_next_transition(&mut self, animation_manager: &AnimationManager<A, P>, to: &str) {
        let Some(pos) = animation_manager.find_transition(&self.current_recording_name, to) else {
            println!(
                "No transition in animation manager from {} to {}",
                self.current_recording_name, to
            );
            return;
        };

        self.current_transition = Some(pos);
    }

    pub fn pick_recording(&mut self, animation_manager: &AnimationManager<A, P>, name: &str) -> Option<Handle<P::Recording>> {
        let Some(pos) = animation_manager.find_recording(name) else {
            println!("No recording in animation manager called {}", name);
            return None;
        };

        self.current_recording_name = String::from(name);
        animation_manager.recordings
            .get(pos)
            .map(|h| h.clone())
    }

    pub fn advance_transition(&mut self, is_end: bool, animation_manager: &AnimationManager<A, P>) -> Option<Handle<P::Recording>> {
        let Some(current_transition) = self.current_transition else {
            return None;
        };

        let transition = &animation_manager.transitions[current_transition];

        if matches!(transition.style, TransitionStyle::OnEnd)
            && !is_end
        {
            return None;
        }

        let to = transition.to.clone();

        let transition_next = animation_manager.next_transition(&to);

        if let Some(next) = transition_next {
            self.pick_transition(animation_manager, &to, &next);
        } else {
            self.current_transition = None;
        }

        self.pick_recording(animation_manager, &to)
    }
}