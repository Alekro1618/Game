use std::{collections::HashMap, fmt::{Display, Pointer}};

use bitmap::Bitmap;
use bitvec::vec::BitVec;
use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{change_detection::params::Mut, systems::{function_system::{F1, F2}, query::Query}}, error::error::{Error, ErrorKind}, math::{transform::{self, Transform}, vec::Vec2}};
use engine_macros::{Component, Resource};

#[derive(Clone, Copy, Eq, Hash)]
pub struct Point {
    x : usize, 
    y : usize,
    value : Option<bool>
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Point {
    pub const fn new(x : usize, y : usize) -> Self {
        Self {
            x,
            y,
            value: None
        }
    }

    fn byte_index(&self, width : usize, height : usize) -> usize {
        self.y * height + self.x
    }

    pub fn value(&self) -> bool {
        Some(true) == self.value
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

impl Into<Point> for (usize, usize) {
    fn  into(self) -> Point {
        Point { 
            x: self.0,
            y: self.1, 
            value: None }
    }    
}


impl TryInto<Point> for (isize, isize) {
    type Error = Error;

    fn try_into(self) -> Result<Point, Self::Error> {
        if self.0 < 0 || self.1 < 0 {
            Err(Error::new(ErrorKind::Validation, "Can't create point out of bounds"))
        } else {
            Ok((self.0 as usize, self.1 as usize).into())
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum GridName {
    Players,
    Walls,
    Interactables
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum SpecialKeys {
    CashRegister,
    Entrence,
    Exit,
    ServicePlace
}



#[derive(Resource)]
pub struct Grid {
    width : usize,
    height : usize,
    maps: HashMap<GridName, BitVec>,
    position : Vec2,
    rotation : f32,
    dx : f32,
    dy : f32,
    vectors : Vec<Vec2>,
    specials : HashMap<SpecialKeys, Point>
}

impl Default for Grid {
    fn default() -> Self {
        Self { 
            width: Default::default(),
            height: Default::default(),
            maps: Default::default(),
            position: Default::default(),
            rotation: Default::default(),
            dx: Default::default(), 
            dy: Default::default(),
            vectors: Vec::with_capacity(0),
            specials: Default::default()}
    }
}

impl Grid  {
    #[inline]
    pub fn new(width : usize, height : usize, dx : f32, dy: f32) -> Self {
        Self::from(width, height, Vec2::ZERO, 0.0, dx, dy)
    }

    pub fn from(width : usize, height : usize, position : Vec2, rotation : f32, dx : f32, dy : f32) -> Self {
        let mut vectors = Vec::with_capacity(width * height);
        for j in 0..height {
            for i in 0..width {
                vectors.insert(i + j * height, Vec2 {
                    x: i as f32 * dx + position.x,
                    y: j as f32 * dy + position.y
                });
            }
        }
        Self { 
            width,
            height,
            maps: HashMap::new(),
            position, 
            rotation, 
            dx, 
            dy,
            vectors,
            specials: HashMap::new()
        }
    }

    #[inline]
    pub fn create_map(&mut self, label : GridName) {
        self.maps.insert(label, BitVec::repeat(false, self.width * self.height));
    }

    #[inline]
    pub fn add_to_map(&mut self, label : GridName, point : impl Into<Point>) {
        let point = point.into();
        let index = point.byte_index(self.width, self.height);
        self.maps.get_mut(&label).map(|b| b.set(index, true));
    }

    #[inline]
    pub fn add_special(&mut self, label : SpecialKeys, point : impl Into<Point>) {
        let point = point.into();
        self.specials.insert(label, point);
    }

    #[inline]
    pub fn get_special(&self, label : SpecialKeys) -> Point {
        self.specials[&label]
    }

    #[inline]
    pub fn clear(&mut self) {
        self.maps.iter_mut().for_each(|(_, b)| b.fill(false));
    }

    #[inline]
    pub fn clear_map<'a>(&mut self, label : GridName) {
        self.maps.get_mut(&label).map(|b| b.fill(false));
    }

    #[inline]
    pub fn remove_from_map<'a>(&mut self, label : GridName, point : impl Into<Point>) {
        let index = point.into().byte_index(self.width, self.height);
        self.maps.get_mut(&label).map(|b| b.set(index, false));
    }

    pub fn to_vec(&self, point : impl Into<Point>) -> Vec2 {
        let d= point.into();
        let index = d.byte_index(self.width, self.height);
        self.vectors[index]
    }

    #[inline]
    pub fn lerp(&self, a : impl Into<Point>, b : impl Into<Point>, time : f32) -> Vec2 {
        let start = self.to_vec(a);
        let end = self.to_vec(b);
        start.lerp(end, time)
    }

    #[inline]
    pub fn what_cell(&self, point : Vec2) -> Point {
        let mut actual = point - self.position;
        actual.x /= self.dx;
        actual.y /= self.dy;
        let x = actual.x.floor() as usize;
        let y = actual.y.floor() as usize;

        Point { x, y, value: None }
    }

    pub fn in_cell(&self, point : Vec2, cell : impl Into<Point>) -> bool {
        self.what_cell(point) == cell.into()
    }

    pub fn same_cell(&self, a : Vec2, b : Vec2) -> bool {
        self.what_cell(a) == self.what_cell(b)
    }

    pub fn get_point<'a>(&self, label : GridName, point : impl Into<Point>) -> Point {
        let mut point = point.into();
        let index = point.byte_index(self.width, self.height);
        point.value = self.maps.get(&label).map(|b| *b.get(index).as_deref().unwrap_or_else(|| panic!("{}", point)));
        point
    }

    pub fn contains(&self, point : impl Into<Point>) -> bool {
        let point = point.into();
        point.x < self.width as usize && point.y < self.height as usize
    }
}

#[derive(Component)]
pub struct LockToGrid {
    map_name : GridName,
    pub position : Point
}

#[derive(Component)]
pub struct LookInGrid {
    map_name : GridName,
    pub position : Point
}

impl LockToGrid {
    pub fn new(map_name : GridName, point : impl Into<Point>) -> Self {
        Self { map_name: map_name, position: point.into() }
    }
}

#[derive(Component)]
pub struct SmoothToGrid {
    map_name : GridName,
    pub start : Point,
    pub target : Point, 
    pub time : f32,
    is_end : bool
}

impl SmoothToGrid {
    pub fn new(map_name : GridName, start : impl Into<Point>, target : impl Into<Point>) -> Self {
        Self {
            map_name: map_name,
            start: start.into(),
            target: target.into(),
            time: 0.0,
            is_end: false
        }
    }

    #[inline]
    pub fn set(&mut self, target : impl Into<Point>) {
        self.start = self.target;
        self.target = target.into();
        self.time = 0.0;
        self.is_end = false;
    }

    #[inline]
    pub fn update(&mut self, interval : f32) {
        let new_time = self.time + interval;
        if new_time > 1.0 {
            self.is_end = true;
        }
        self.time = new_time.clamp(0.0, 1.0)
    }

    #[inline]
    pub fn is_end(&self) -> bool {
        self.is_end
    }
}


pub fn point_to_transform(query : Query<(&mut Transform, &LockToGrid)>, grid : &Grid) {
    for (mut transform, lock_to_grid) in query {
        transform.position = grid.to_vec(lock_to_grid.position);
    }
}

pub fn smooth_point_to_transform(query : Query<(&mut Transform, &SmoothToGrid)>, grid : &Grid) {
    for (mut transform, smooth_to_grid) in query {
        transform.position = grid.lerp(smooth_to_grid.start, smooth_to_grid.target, smooth_to_grid.time);
    }
}

pub fn transform_to_point(query : Query<(&Transform, &mut LockToGrid)>, grid : &Grid) {
    for (transform, mut lock_to_grid) in query {
        lock_to_grid.position = grid.what_cell(transform.position);
    }
}

pub fn update_grid(mut grid : Mut<'_, Grid>, query_lock : Query<&LockToGrid>, query_smooth : Query<&SmoothToGrid>) {
    grid.clear();
    for point in query_lock {
        grid.add_to_map(point.map_name, point.position);
    }

    for smooth in query_smooth {
        grid.add_to_map(smooth.map_name, smooth.target);
    }
}

