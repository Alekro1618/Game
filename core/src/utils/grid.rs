use std::{collections::HashMap, fmt::{Display, Pointer}};

use bitmap::Bitmap;
use bitvec::vec::{self, BitVec};
use engine::{app::{main_schedule::ScheduleLabel, plugin::Plugin}, ecs::{change_detection::params::Mut, systems::{ query::Query}}, error::error::{Error, ErrorKind}, math::{transform::{self, Transform}, vec::Vec2}, render::color::Color};
use engine_macros::{Component, Resource};

use crate::utils::path::traverse_path;

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

pub struct GridParams {
    position : Vec2,
    dx : f32,
    dy : f32,
    angle : f32,
    center : Vec2,
    distance : f32
}

impl GridParams {
    pub fn new(position : Vec2, dx : f32, dy : f32, angle : f32, center : Vec2, distance : f32) -> Self {
        Self { position, dx, dy, angle, center, distance }
    }

    pub fn get_vectors(self, height : usize, width : usize) -> Vec<Vec2> {
        let mut vectors = Vec::with_capacity(width * height);
        for j in 0..height {
            for i in 0..width {
                let x = i as f32 * self.dx + self.position.x;
                let y= j as f32 * self.dy + self.position.y;
                
                let vx = self.distance * ( x - self.center.x) / (y * self.angle.sin() + self.distance) + self.center.x;
                let vy = self.distance * ( y * self.angle.cos() - self.center.y) / (y * self.angle.sin() + self.distance) + self.center.y;
                vectors.insert(i + j * height, Vec2 {
                    x: vx,
                    y: vy
                });
            }
        }
        vectors
    }
}



#[derive(Resource)]
pub struct Grid {
    width : usize,
    height : usize,
    maps: HashMap<GridName, BitVec>,
    vectors : Vec<Vec2>,
    specials : HashMap<SpecialKeys, Point>
}

impl Default for Grid {
    fn default() -> Self {
        Self { 
            width: Default::default(),
            height: Default::default(),
            maps: Default::default(),
            vectors: Vec::with_capacity(0),
            specials: Default::default()}
    }
}

impl Grid  {
    #[inline]
    pub fn new(width : usize, height : usize, params : GridParams) -> Self {
        Self::from(width, height, params)
    }

    pub fn from(width : usize, height : usize, params : GridParams) -> Self {
        let vectors = params.get_vectors(height, width);

        Self { 
            width,
            height,
            maps: HashMap::new(),
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

pub fn update_grid(mut grid : Mut<'_, Grid>, query_lock : Query<&LockToGrid>, query_smooth : Query<&SmoothToGrid>) {
    grid.clear();
    for point in query_lock {
        grid.add_to_map(point.map_name, point.position);
    }

    for smooth in query_smooth {
        grid.add_to_map(smooth.map_name, smooth.target);
    }
}

pub fn draw_grid(grid : &Grid) {
    for i in 0..grid.width {
        let mut last_point = grid.to_vec((i, 0));
        for j in 1..grid.height {
            let current = grid.to_vec((i, j));
            macroquad::shapes::draw_line(last_point.x, last_point.y, current.x, current.y, 10., macroquad::color::Color::new(1., 1., 1., 1.));
            last_point = current;
        }
    }

    for j in 0..grid.height {
        let mut last_point = grid.to_vec((0, j));
        for i in 1..grid.width {
            let current = grid.to_vec((i, j));
            macroquad::shapes::draw_line(last_point.x, last_point.y, current.x, current.y, 10., macroquad::color::Color::new(1., 1., 1., 1.));
            last_point = current;
        }
    }
   
}


pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(self, app:  &mut engine::app::App) {
        app.add_systems(
            ScheduleLabel::PreUpdate, 
            smooth_point_to_transform
        );
        app.add_systems(
            ScheduleLabel::PreUpdate, 
            point_to_transform
        );
        app.add_systems(
            ScheduleLabel::FixedUpdate, 
            update_grid
        );
        app.add_systems(
            ScheduleLabel::FixedUpdate, 
            traverse_path
        );
        app.add_systems(
            ScheduleLabel::Update, 
            draw_grid
        );
    }
}