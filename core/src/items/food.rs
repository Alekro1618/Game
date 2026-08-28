use std::fmt::{Display, write};


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Food {
    CoffeeBeans,
    Espresso,
    Bread,
    CuttedBread,
    Latte,
    Flour,
}

impl Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Food::CoffeeBeans =>  write!(f, "Coffee beans"),
            Food::Espresso =>  write!(f, "Espresso", ),
            Food::Bread =>  write!(f, "Bread"),
            Food::Flour => write!(f, "Flour"),
            Food::CuttedBread => write!(f, "Cutted bread"),
            Food::Latte => write!(f, "Latte"),
        }
    }
}