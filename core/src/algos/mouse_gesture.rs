use std::f32;
use engine::math::{circle, vec::Vec2};


pub fn mouse_spiral(path : &[Vec2]) -> f32 {
    if path.len() < 2 {
        return 0.0;
    }

    let mut power = Vec2::ZERO;
    let mut factor = 1.0;
    let center = circle(path.try_into().unwrap());    
    let mut angle = 0.0;

    //Since we find angle difference between 2 close points,
    // the center of circle will be far away and angle will be small. So we use approximation cos x = x
    
    for window in path.windows(2) {
        let a =window[0];
        let b = window[1];

        power += factor * a;
        factor /= 2.0;
        angle += (a - center).cos_angle(&(b - center));
    }
    
    power.norm() * angle
}

pub fn mouse_shaking(path : &Vec<Vec2>) -> f32 {
    if path.len() < 5 {
        return 0.0;
    }

    let horizontal : Vec<f32> = path
        .iter()
        .map(|v| v.x)
        .collect();

    let direction : Vec<f32> = horizontal
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect();

    let mut max = f32::NEG_INFINITY;
    let mut min = f32::INFINITY;

    for value in horizontal {
        if max < value {
            max = value;
        }

        if min > value {
            min = value;
        }
    }

    (monotonicty(&direction) * (max - min) / 500.0).clamp(-1.0, 1.0)
}

pub fn monotonicty(seq : &Vec<f32>) -> f32 {
    let mut up = 0;
    let mut down = 0;
    for d in seq {
        if d > &0.1 {
            up += 1;
        } else if d < &-0.1 {
            down += 1;
        }
    }

    if up > seq.len() - 3 {
        1.0
    } else if down > seq.len() - 3 {
        -1.0
    } else {
        0.0
    }
}