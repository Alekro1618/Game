use std::f32;

use raylib::math::Vector2;

pub fn mouse_spiral(path : &Vec<Vector2>) -> f32 {
    todo!()
}

pub fn mouse_shaking(path : &Vec<Vector2>) -> f32 {
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