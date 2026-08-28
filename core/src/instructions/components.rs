use std::{fmt::Display, iter::Peekable};

use engine::{ecs::entities::id::EntityId, ui::ui_node::JustifyContent::SpaceAround};

use crate::{items::Item, machines::machine_type::MachineType, utils::grid::Point};


pub struct Instruction {
    steps : Vec<Step>,
}

impl ToStep for Step {
    fn to_step(self) -> Step {
        self
    }
}

pub trait ToInstruction {
    fn to_instruction(self) -> Instruction;
}

impl<T : Iterator<Item = S>, S : ToStep> ToInstruction for T {
    fn to_instruction(self) -> Instruction {
        Instruction::from(self)
    }
}

pub trait ToStep {
    fn to_step(self) -> Step;
}

impl Instruction {

    pub fn iter(&self) -> Peekable<std::slice::Iter<'_, Step>> {
        self.steps.iter().peekable()
    }
}

impl<I :Iterator<Item = T>, T : ToStep> From<I> for Instruction {
    fn from(value: I) -> Self {
        Self {
            steps: value.map(|e|  e.to_step()).collect()
        }
    }
}

pub enum Step {
    MoveTo(Point),
    Interact(MachineType, Option<Item>),
    WaitUntil(MachineType)
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum SpecialLabel {
    Exit
}

impl Display for SpecialLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpecialLabel::Exit => write!(f, "Exit"),
        }
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub enum InstructionLabel {
    Create(Item),
    WalkTo(MachineType),
    Fix(MachineType),
    Special(SpecialLabel)
}

impl ToString for InstructionLabel {
    fn to_string(&self) -> String {
        match self {
            InstructionLabel::Create(item) => item.to_string(),
            InstructionLabel::WalkTo(machine_type) => machine_type.to_string(),
            InstructionLabel::Fix(machine_type) => machine_type.to_string(),
            InstructionLabel::Special(special_label) => special_label.to_string(),
        }
    }
}