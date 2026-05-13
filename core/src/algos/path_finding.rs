use std::collections::{HashMap, HashSet, VecDeque};

use crate::utils::grid::{Grid, GridName, Point};

pub fn bfs(grid : &Grid, start : impl Into<Point>, end : impl Into<Point>) -> Vec<Point> {
    let start = start.into();
    let end = end.into();
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent: HashMap<Point, Point> = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];

    while let Some(current) = queue.pop_front() {
        if current == end {
            break;
        }

        for (dx, dy) in dirs.iter() {
            let resut : Result<Point, _> = (current.x() as isize + dx, current.y() as isize + dy).try_into();
            if resut.is_err() {
                continue;
            }
            
            let next = resut.unwrap();
            
            if !grid.get_point(GridName::Walls, next).value() {
                if !visited.contains(&next) {
                    visited.insert(next);
                    parent.insert(next, current);
                    queue.push_back(next);
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

    path
}

pub fn find_path(grid : &Grid, start : impl Into<Point>, end : impl Into<Point>) -> Vec<Point> {
    bfs(grid, start, end)
}