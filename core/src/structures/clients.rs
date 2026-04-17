

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum Client {
    ThatOne,
    TheSecond,
    Loser
}

impl Client {

    pub fn to(self) -> &'static str {
        match self {
            Client::ThatOne => "that_one",
            Client::TheSecond => "the_second",
            Client::Loser => "loser",
        }
    }
}

