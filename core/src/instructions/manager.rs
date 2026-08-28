use std::collections::HashMap;

use engine_macros::{Component, Resource};

use crate::instructions::components::{Instruction, InstructionLabel, ToInstruction};


#[derive(Default, Resource)]
pub struct InstructionManager {
    instructions : HashMap<InstructionLabel, Instruction>,
}


impl InstructionManager {
    pub fn create_new<I : ToInstruction>(
        &mut self,
        label : InstructionLabel,
        instruction : I
    ) -> Option<Instruction> {
        self.instructions.insert(label, instruction.to_instruction())
    }

    pub fn remove(
        &mut self,
        label : &InstructionLabel,
    ) -> Option<Instruction> {
        self.instructions.remove(label)
    }

    pub fn get(
        &self,
        label : & InstructionLabel
    ) -> Option<&Instruction> {
        self.instructions.get(label)
    }
}