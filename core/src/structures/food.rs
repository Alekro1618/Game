#[derive(Clone, PartialEq, Eq, Debug)]
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

    pub fn to(&self) -> &'static str {
        match self {
            Food::CoffeeBeans => "coffee_beans",
            Food::Espresso => "espresso",
            Food::Bread => "bread",
            Food::Flour => "flour"
        }
    }

    pub fn from(name : &str) -> Self {
        match name {
            "coffee_beans" => Food::CoffeeBeans,
            "espresso" => Food::Espresso,
            "bread" => Food::Bread,
            "flour" => Food::Flour,
            s => panic!("Can't convert food: {}!", s)
        }
    }
}