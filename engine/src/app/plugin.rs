use crate::app::App;

pub trait Plugin {
    fn build(&self, _app:  &mut App) {}
}