use crate::ui::ui_node::Node;

pub struct Modifier {
    elements : Vec<Box<dyn ModifierElement>>
}

impl Modifier {
    pub fn new() -> Self { 
        Self { elements: Vec::new() }
    }

    pub fn apply(self, node : &mut Node) {
        self.elements.iter().for_each(|el| el.apply(node));
    }
}

trait ModifierElement {
    fn apply(&self, node : &mut Node) {

    }
}