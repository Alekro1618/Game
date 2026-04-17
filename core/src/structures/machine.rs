

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Machine {
    CoffeeBag,
    CoffeMachine,
    Display,
    RubbishCan,
    Cutter
}

impl Machine {
    pub fn from(name : &str) -> Self {
        match name {
            "coffee_bag" => Machine::CoffeeBag,
            "coffee_machine" => Machine::CoffeMachine,
            "display" => Machine::Display,
            "rubbish_can" => Machine::RubbishCan,
            "cutter" => Machine::Cutter,
            _ => panic!("Unknown machine!")
        }
    }
}

