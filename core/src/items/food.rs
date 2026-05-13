
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Food {
    CoffeeBeans,
    Espresso,
    Bread,
    Flour,
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
        }
    }

    pub fn is_fridge(&self) -> bool {
        match self {
            Food::CoffeeBeans => false,
            Food::Espresso => false,
            Food::Bread => false,
            Food::Flour => true,
        }
    }
}