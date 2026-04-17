use std::collections::{HashMap, HashSet, VecDeque};
use raylib::math::Vector2;
use crate::engine::utils::grid::Grid;

fn metric(start : (usize, usize), end : (usize, usize)) -> usize {
    start.0.abs_diff(end.0).max(start.1.abs_diff(end.1))
}


pub fn bfs<const W : usize, const H : usize>(grid : &Grid<W, H>, start : (usize, usize), end : (usize, usize)) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    while let Some(current) = queue.pop_front() {
        if current == end {
            break;
        }

        for (dx, dy) in dirs.iter() {
            let next = (current.0 as isize + dx, current.1 as isize + dy);

            if !grid.is_wall(&next) {
                let next_in_grid = (next.0 as usize, next.1 as usize);
                if !visited.contains(&next_in_grid) {
                    visited.insert(next_in_grid);
                    parent.insert(next_in_grid, current);
                    queue.push_back(next_in_grid);
                }
            }
        }   
    }

    if !visited.contains(&end) {
        return Vec::new();
    }

    let mut path = Vec::new();
    let mut current = end;

    while current != start {
        path.push(current);
        current = *parent.get(&current).unwrap();
    }
    path.push(start);
    path.reverse();

    path
}

pub fn path<const W : usize, const H : usize>(grid : &Grid<W, H>, start : (usize, usize), end : (usize, usize)) -> Vec<(usize, usize)> {
    bfs(grid, start, end)
}