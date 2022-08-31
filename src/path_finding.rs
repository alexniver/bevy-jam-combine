use bevy::prelude::*;

use crate::common::MAP_SIZE;

pub fn next_grid(start: Vec2, end: Vec2) -> Vec<Vec2> {
    let mut res = vec![];

    let distance = (start - end).length();

    let mut round_list = vec![];
    if start.x > 0. {
        round_list.push(Vec2::new(start.x - 1., start.y));
    }
    if start.x < (MAP_SIZE - 1) as f32 {
        round_list.push(Vec2::new(start.x + 1., start.y));
    }
    if start.y > 0. {
        round_list.push(Vec2::new(start.x, start.y - 1.));
    }
    if start.y < (MAP_SIZE - 1) as f32 {
        round_list.push(Vec2::new(start.x, start.y + 1.));
    }

    for pos in round_list {
        if (pos - end).length() < distance {
            res.push(pos);
        }
    }

    res
}
