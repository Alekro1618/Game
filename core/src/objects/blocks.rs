use ::Game::object;
use crate::engine::game::manager::Game;


#[object(
    name = "wall",
    texture = "red",
    tags =["block"],
    visible = true)]
pub struct Wall;

#[object(
    name = "floor",
    texture = "background",
    tags = ["block"],
    visible = true)]
pub struct Floor;

#[object(
    name = "blender", 
    texture = "green", 
    tags = ["block"], 
    visible = true,
    states = [
        Idle,
        Making,
        Ready
    ])]
pub struct Blender;

#[object(
    name = "coffee_bag", 
    texture = "blue", 
    tags = ["machine"], 
    visible = true)]
pub struct CoffeeBag;

#[object(
    name = "coffee_machine", 
    texture = "yellow", 
    components = ["coffee_cooking"], 
    tags = ["machine"], 
    visible = true,
    states = [
        Idle,
        Making,
        Ready
    ]
)]
pub struct CoffeeMachine;

#[object(
    name = "cutter", 
    texture = "yellow", 
    components = ["cutting"], 
    tags = ["machine"], 
    visible = true,
    states = [
        Idle,
        Making,
        Ready
    ])]
pub struct Cutter;

#[object(
    name = "rubbish_can",
    texture = "purple",
    tags = ["machine"],
    visible = true,
    states = [
        Idle,
        Throw
    ])]
pub struct RubbishCan;

#[object(
    name = "display",
    texture = "purple",
    tags = ["machine"],
    visible = true,
    states = [
        Idle,
        Insert
    ])]
pub struct Display;


pub trait Blocks {
    fn add_blocks(&mut self) -> &mut Self;
}

impl Blocks for Game {
    fn add_blocks(&mut self) -> &mut Self {
        self
            .add_prefab(Wall::new())
            .add_prefab(Floor::new())
            .add_prefab(Blender::new())
            .add_prefab(CoffeeMachine::new())
            .add_prefab(CoffeeBag::new())
            .add_prefab(RubbishCan::new())
    }
}
