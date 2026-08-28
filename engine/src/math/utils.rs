use crate::math::vec::Vec2;

///Returns center of circle intersecting 3 points
pub fn circle(points: &[Vec2; 3]) -> Vec2 {
    let [_a, _b, _c] = points;

    todo!("")
}

pub fn urn_function(x: f32, offset: f32) -> f32 {
    if -offset <= x && x < offset {
        0.
    } else if x >= offset {
        let d = (x - offset) / (1. - offset);
        d * d
    } else {
        let d = (x + offset) / (1. - offset);
        -d * d
    }
}
