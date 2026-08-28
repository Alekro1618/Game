use crate::app::plugin::Plugin;

pub mod transform;

pub struct RelationshipPlugin;

impl Plugin for RelationshipPlugin {
    fn build(self, _app: &mut crate::app::App) {}
}
