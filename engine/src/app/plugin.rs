use crate::app::App;

pub trait Plugin {
    fn build(&self, _app:  &mut App) {}
}

pub trait GenericPlugin<T> {
    fn build(&self, _app:  &mut App) {}
}