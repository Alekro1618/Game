use crate::app::App;

pub trait Plugin
where Self : Sized {
    fn build(self, _app:  &mut App) {}
}