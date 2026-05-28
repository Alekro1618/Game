use std::fmt::Display;


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Food {
    CoffeeBeans,
    Espresso,
    Bread,
    CuttedBread,
    Flour,
}

impl Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Food::CoffeeBeans =>  write!(f, "CoffeeBeans"),
            Food::Espresso =>  write!(f, "Espresso", ),
            Food::Bread =>  write!(f, "Bread"),
            Food::Flour => write!(f, "Flour"),
            Food::CuttedBread => write!(f, "CuttedBread"),
        }
       
    }
}

impl Food {
    pub fn all() -> &'static [Food] {
        static FOODS: [Food; 4] = [Food::CoffeeBeans, Food::Espresso, Food::Bread, Food::Flour];
        &FOODS
    }

    pub fn is_display(&self) -> bool {
        match self {
            Food::CoffeeBeans => false,
            Food::Espresso => false,
            Food::Bread => false,
            Food::Flour => false,
            Food::CuttedBread => false,
        }
    }

    pub fn is_fridge(&self) -> bool {
        match self {
            Food::CoffeeBeans => false,
            Food::Espresso => false,
            Food::Bread => false,
            Food::Flour => false,
            Food::CuttedBread => false,
        }
    }
}