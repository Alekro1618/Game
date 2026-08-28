use crate::{
    animation::Animatable,
    math::{transform::Transform, vec::Vec2},
};

impl Animatable for f32 {
    fn advance(self, rhs: Self, time: f32) -> Self {
        self * (1. - time) + rhs * time
    }
}

impl Animatable for Vec2 {
    fn advance(self, rhs: Self, time: f32) -> Self {
        Vec2::lerp(self, rhs, time)
    }
}

impl Animatable for Transform {
    fn advance(self, rhs: Self, time: f32) -> Self {
        Transform {
            position: self.position.advance(rhs.position, time),
            rotation: self.rotation.advance(rhs.rotation, time),
            scale: self.scale.advance(rhs.scale, time),
        }
    }
}
