use engine::{app::FIXED_UPDATE, ecs::systems::query::Query, math::transform::{self, Transform}};
use engine_macros::Component;

use crate::utils::grid::{Grid, Point};

#[derive(Component)]
pub struct Path {
    path : Vec<Point>,
    current : Point,
    time : f32,
}


impl Path {
    #[inline]
    pub fn new(pos : impl Into<Point>) -> Self {
        let pos =  pos.into();
        Self { 
            path: Vec::new(),
            current: pos,
            time: 0.0
        }
    }

    #[inline]
    pub fn update(&mut self, interval : f32) {
        if let Some(_) = self.path.last() {
            self.time += interval;

            if self.time > 1.0 {
                self.time -= 1.0;
                self.current = self.path.pop().unwrap();
            }
        } else {
            self.time = 1.0;
        }
    }

    #[inline] 
    pub fn target(&self) -> Option<Point> {
        self.path.last().map(|d| *d)
    }

    #[inline]
    pub fn set_path(&mut self,  mut path : Vec<Point>) {
        if let Some(target) = path.pop() {
            self.current = target
        }
        self.time = 0.0;
        self.path = path;
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    #[inline]
    pub fn current(&self) -> Point {
        self.current
    }
}


pub fn traverse_path(query : Query<(&mut Transform, &Path)>, grid : &Grid) {
    for (mut transform, path) in query {
        if let Some(target) = path.target() {
            transform.position = grid.lerp(path.current, target, path.time);
        } else {
            transform.position= grid.to_vec(path.current);
        }
    }
}


