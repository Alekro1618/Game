use crate::{
    ecs::entities::id::EntityId,
    error::error::Error,
    math::{
        rect::Rect,
        vec::{Vec2, vec2},
    },
    ui::{
        tree::NodeEdge,
        ui_node::Display,
    },
};

#[derive(Debug, Default, Clone)]
pub struct Dimensions {
    // Position of the content area relative to the document origin:
    content: Rect,

    // Surrounding edges:
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}

impl Dimensions {
    pub fn with_content(rect: Rect) -> Self {
        Self {
            content: rect,
            ..Default::default()
        }
    }

    pub fn content(&self) -> &Rect {
        &self.content
    }

    /// The area covered by the content area plus its padding.
    pub fn padding_box(&self) -> Rect {
        expanded_by(&self.content, &self.padding)
    }
    /// The area covered by the content area plus padding and borders.
    pub fn border_box(&self) -> Rect {
        expanded_by(&self.padding_box(), &self.border)
    }
    /// The area covered by the content area plus padding, borders, and margin.
    pub fn margin_box(&self) -> Rect {
        expanded_by(&self.border_box(), &self.margin)
    }
}

pub fn expanded_by(rect: &Rect, edges: &EdgeSizes) -> Rect {
    let min = rect.min - vec2(edges.left, edges.bottom);
    let max = rect.max + vec2(edges.right, edges.top);
    Rect::new(min, max)
}

#[derive(Debug, Default, Clone)]
pub struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

pub struct LayoutBox<'a> {
    pub entity: EntityId,
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

impl<'a> LayoutBox<'a> {
    fn new(entity: EntityId, box_type: BoxType<'a>) -> LayoutBox<'a> {
        LayoutBox {
            entity,
            box_type,
            dimensions: Default::default(),
            children: Vec::new(),
        }
    }

    fn get_node_edge(&self) -> &'a NodeEdge<'a> {
        self.box_type.get()
    }

    fn get_inline_container(&mut self) -> &mut LayoutBox<'a> {
        match self.box_type {
            BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
            BoxType::BlockNode(_) => {
                match self.children.last() {
                    Some(&LayoutBox {
                        box_type: BoxType::AnonymousBlock,
                        ..
                    }) => {}
                    _ => self
                        .children
                        .push(LayoutBox::new(self.entity, BoxType::AnonymousBlock)),
                }
                self.children.last_mut().unwrap()
            }
        }
    }

    fn layout(&mut self, available: &Dimensions, target: Vec2) -> Result<(), Error> {
        match self.box_type {
            BoxType::BlockNode(_) => self.layout_block(available, target),
            BoxType::InlineNode(_) => self.layout_inline(available, target),
            BoxType::AnonymousBlock => self.layout_anonymous(available, target),
        }
    }

    fn layout_block(&mut self, available: &Dimensions, target: Vec2) -> Result<(), Error> {
        self.calculate_block_width(&available, target)?;
        self.calculate_block_position(&available, target)?;
        self.layout_block_children(target)?;
        self.calculate_block_height(&available, target);

        println!(
            "entity: {} rect: {:#?} av: {:#?}",
            self.entity,
            self.dimensions.content.size(),
            available.content.size()
        );

        Ok(())
    }

    fn calculate_block_width(&mut self, available: &Dimensions, target: Vec2) -> Result<(), Error> {
        let node = self.get_node_edge().node;
        let content_width = available.content.width();

        let width = node.width.resolve(content_width, target)?;

        let margin_left = node.margin.left.resolve(content_width, target)?;
        let margin_right = node.margin.right.resolve(content_width, target)?;

        let border_left = node.border.left.resolve(content_width, target)?;
        let border_right = node.border.right.resolve(content_width, target)?;

        let padding_left = node.padding.left.resolve(content_width, target)?;
        let padding_right = node.padding.right.resolve(content_width, target)?;

        let total: f32 = [
            &margin_left,
            &margin_right,
            &border_left,
            &border_right,
            &padding_left,
            &padding_right,
            &width,
        ]
        .iter()
        .map(|v| *v)
        .sum();

        if total > content_width {
            println!("some UI elements are overflowed!");
        }

        let d = &mut self.dimensions;
        d.content.set_width(width);

        d.padding.left = padding_left;
        d.padding.right = padding_right;

        d.border.left = border_left;
        d.border.right = border_right;

        d.margin.left = margin_left;
        d.margin.right = margin_right;

        Ok(())
    }

    fn calculate_block_position(
        &mut self,
        available: &Dimensions,
        target: Vec2,
    ) -> Result<(), Error> {
        let node = self.get_node_edge().node;

        let content_height = available.content.height();

        let margin_top = node.margin.top.resolve(content_height, target)?;
        let margin_bottom = node.margin.bottom.resolve(content_height, target)?;

        let border_top = node.border.top.resolve(content_height, target)?;
        let border_bottom = node.border.bottom.resolve(content_height, target)?;

        let padding_top = node.padding.top.resolve(content_height, target)?;
        let padding_bottom = node.padding.bottom.resolve(content_height, target)?;

        let d = &mut self.dimensions;

        d.margin.top = margin_top;
        d.margin.bottom = margin_bottom;

        d.border.top = border_top;
        d.border.bottom = border_bottom;

        d.padding.top = padding_top;
        d.padding.bottom = padding_bottom;

        let x = available.content.x() + d.margin.left + d.border.left + d.padding.left;

        let y =
            content_height + available.content.y() + d.margin.top + d.border.top + d.padding.top;

        d.content.set_pos(vec2(x, y));

        Ok(())
    }

    fn layout_block_children(&mut self, target: Vec2) -> Result<(), Error> {
        for child in &mut self.children {
            child.layout(&self.dimensions, target)?;
            self.dimensions
                .content
                .add_height(child.dimensions.margin_box().height());
        }

        Ok(())
    }

    fn calculate_block_height(&mut self, available: &Dimensions, target: Vec2) {
        if let Ok(h) = self
            .get_node_edge()
            .node
            .height
            .resolve(available.content.height(), target)
        {
            self.dimensions.content.set_height(h);
        }
    }

    fn layout_inline(&mut self, _available: &Dimensions, _target: Vec2) -> Result<(), Error> {
        Ok(())
    }

    fn layout_anonymous(&mut self, _available: &Dimensions, _target: Vec2) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Default)]
pub struct LayoutBoxWithoutNode {
    pub entity: EntityId,
    pub dimensions: Dimensions,
    pub children: Vec<LayoutBoxWithoutNode>,
}

impl LayoutBoxWithoutNode {
    pub fn hovers(&self, vec: Vec2) -> bool {
        self.dimensions.content.inside(vec)
    }
}

pub enum BoxType<'a> {
    BlockNode(&'a NodeEdge<'a>),
    InlineNode(&'a NodeEdge<'a>),
    AnonymousBlock,
}

impl<'a> BoxType<'a> {
    pub fn get(&self) -> &'a NodeEdge<'a> {
        match self {
            BoxType::BlockNode(node_edge) | BoxType::InlineNode(node_edge) => node_edge,
            BoxType::AnonymousBlock => panic!("Anonymous block box has no node edge"),
        }
    }
}

pub fn forget_nodes<'a>(node: &'a LayoutBox<'a>) -> LayoutBoxWithoutNode {
    LayoutBoxWithoutNode {
        entity: node.entity,
        dimensions: node.dimensions.clone(),
        children: node.children.iter().map(|c| forget_nodes(c)).collect(),
    }
}

pub fn layout_tree<'a>(
    node: &'a NodeEdge<'a>,
    mut available: Dimensions,
    target: Vec2,
) -> Result<LayoutBox<'a>, Error> {
    // The layout algorithm expects the container height to start at 0.
    // TODO: Save the initial containing block height, for calculating percent heights.
    available.content.set_height(0.0);

    let mut root_box = build_layout_tree(node);
    root_box.layout(&available, target)?;
    Ok(root_box)
}

fn build_layout_tree<'a>(node_edge: &'a NodeEdge<'a>) -> LayoutBox<'a> {
    let mut root = LayoutBox::new(
        node_edge.entity,
        match node_edge.node.display {
            Display::Block => BoxType::BlockNode(node_edge),
            Display::Inline => BoxType::InlineNode(node_edge),
            Display::None => panic!("Root node has display: none."),
        },
    );

    for child in &node_edge.children {
        match child.node.display {
            Display::Block => root.children.push(build_layout_tree(child)),
            Display::Inline => root
                .get_inline_container()
                .children
                .push(build_layout_tree(child)),
            Display::None => {}
        }
    }

    return root;
}
