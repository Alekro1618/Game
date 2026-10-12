use taffy::style_helpers;

use crate::ui::{geometry::{UiRect, Val}, layout::Available, ui_node::{AlignContent, AlignItems, AlignSelf, BoxSizing, Display, FlexDirection, FlexWrap, GridAutoFlow, JustifyContent, JustifyItems, JustifySelf, Node, OverflowAxis, PositionType}};


pub fn to_style(node : &Node, available : &Available) -> taffy::style::Style {
    taffy::style::Style {
        display: node.display.into(),
        box_sizing: node.box_sizing.into(),
        item_is_table: false,
        text_align: taffy::TextAlign::Auto,
        overflow: taffy::Point {
            x: node.overflow.x.into(),
            y: node.overflow.y.into(),
        },
        scrollbar_width: node.scrollbar_width,
        position: node.position_type.into(),
        flex_direction: node.flex_direction.into(),
        flex_wrap: node.flex_wrap.into(),
        align_items: node.align_items.into(),
        justify_items: node.justify_items.into(),
        align_self: node.align_self.into(),
        justify_self: node.justify_self.into(),
        align_content: node.align_content.into(),
        justify_content: node.justify_content.into(),
        inset: taffy::Rect {
            left: node.left.into_length_percentage_auto(available),
            right: node.right.into_length_percentage_auto(available),
            top: node.top.into_length_percentage_auto(available),
            bottom: node.bottom.into_length_percentage_auto(available),
        },
        margin: node
            .margin
            .map_to_taffy_rect(|m| m.into_length_percentage_auto(available)),
        padding: node
            .padding
            .map_to_taffy_rect(|m| m.into_length_percentage(available)),
        border: node
            .border
            .map_to_taffy_rect(|m| m.into_length_percentage(available)),
        flex_grow: node.flex_grow,
        flex_shrink: node.flex_shrink,
        flex_basis: node.flex_basis.into_dimension(available),
        size: taffy::Size {
            width: node.width.into_dimension(available),
            height: node.height.into_dimension(available),
        },
        min_size: taffy::Size {
            width: node.min_width.into_length_percentage_auto(available),
            height: node.min_height.into_length_percentage_auto(available),
        },
        max_size: taffy::Size {
            width: node.max_width.into_length_percentage_auto(available),
            height: node.max_height.into_length_percentage_auto(available),
        },
        aspect_ratio: node.aspect_ratio,
        gap: taffy::Size {
            width: node.column_gap.into_length_percentage(available),
            height: node.row_gap.into_length_percentage(available),
        },
        ..Default::default()
    }
}

impl From<AlignItems> for Option<taffy::style::AlignItems> {
    fn from(value: AlignItems) -> Self {
        match value {
            AlignItems::Default => None,
            AlignItems::Start => taffy::style::AlignItems::START.into(),
            AlignItems::End => taffy::style::AlignItems::END.into(),
            AlignItems::FlexStart => taffy::style::AlignItems::FLEX_START.into(),
            AlignItems::FlexEnd => taffy::style::AlignItems::FLEX_END.into(),
            AlignItems::Center => taffy::style::AlignItems::CENTER.into(),
            AlignItems::Baseline => taffy::style::AlignItems::BASELINE.into(),
            AlignItems::Stretch => taffy::style::AlignItems::STRETCH.into(),
        }
    }
}

impl From<JustifyItems> for Option<taffy::style::JustifyItems> {
    fn from(value: JustifyItems) -> Self {
        match value {
            JustifyItems::Default => None,
            JustifyItems::Start => taffy::style::JustifyItems::START.into(),
            JustifyItems::End => taffy::style::JustifyItems::END.into(),
            JustifyItems::Center => taffy::style::JustifyItems::CENTER.into(),
            JustifyItems::Baseline => taffy::style::JustifyItems::BASELINE.into(),
            JustifyItems::Stretch => taffy::style::JustifyItems::STRETCH.into(),
        }
    }
}

impl From<AlignSelf> for Option<taffy::style::AlignSelf> {
    fn from(value: AlignSelf) -> Self {
        match value {
            AlignSelf::Auto => None,
            AlignSelf::Start => taffy::style::AlignSelf::START.into(),
            AlignSelf::End => taffy::style::AlignSelf::END.into(),
            AlignSelf::FlexStart => taffy::style::AlignSelf::FLEX_START.into(),
            AlignSelf::FlexEnd => taffy::style::AlignSelf::FLEX_END.into(),
            AlignSelf::Center => taffy::style::AlignSelf::CENTER.into(),
            AlignSelf::Baseline => taffy::style::AlignSelf::BASELINE.into(),
            AlignSelf::Stretch => taffy::style::AlignSelf::STRETCH.into(),
        }
    }
}

impl From<JustifySelf> for Option<taffy::style::JustifySelf> {
    fn from(value: JustifySelf) -> Self {
        match value {
            JustifySelf::Auto => None,
            JustifySelf::Start => taffy::style::JustifySelf::START.into(),
            JustifySelf::End => taffy::style::JustifySelf::END.into(),
            JustifySelf::Center => taffy::style::JustifySelf::CENTER.into(),
            JustifySelf::Baseline => taffy::style::JustifySelf::BASELINE.into(),
            JustifySelf::Stretch => taffy::style::JustifySelf::STRETCH.into(),
        }
    }
}

impl From<AlignContent> for Option<taffy::style::AlignContent> {
    fn from(value: AlignContent) -> Self {
        match value {
            AlignContent::Default => None,
            AlignContent::Start => taffy::style::AlignContent::START.into(),
            AlignContent::End => taffy::style::AlignContent::END.into(),
            AlignContent::FlexStart => taffy::style::AlignContent::FLEX_START.into(),
            AlignContent::FlexEnd => taffy::style::AlignContent::FLEX_END.into(),
            AlignContent::Center => taffy::style::AlignContent::CENTER.into(),
            AlignContent::Stretch => taffy::style::AlignContent::STRETCH.into(),
            AlignContent::SpaceBetween => taffy::style::AlignContent::SPACE_BETWEEN.into(),
            AlignContent::SpaceAround => taffy::style::AlignContent::SPACE_AROUND.into(),
            AlignContent::SpaceEvenly => taffy::style::AlignContent::SPACE_EVENLY.into(),
        }
    }
}

impl From<JustifyContent> for Option<taffy::style::JustifyContent> {
    fn from(value: JustifyContent) -> Self {
        match value {
            JustifyContent::Default => None,
            JustifyContent::Start => taffy::style::JustifyContent::START.into(),
            JustifyContent::End => taffy::style::JustifyContent::END.into(),
            JustifyContent::FlexStart => taffy::style::JustifyContent::FLEX_START.into(),
            JustifyContent::FlexEnd => taffy::style::JustifyContent::FLEX_END.into(),
            JustifyContent::Center => taffy::style::JustifyContent::CENTER.into(),
            JustifyContent::Stretch => taffy::style::JustifyContent::STRETCH.into(),
            JustifyContent::SpaceBetween => taffy::style::JustifyContent::SPACE_BETWEEN.into(),
            JustifyContent::SpaceAround => taffy::style::JustifyContent::SPACE_AROUND.into(),
            JustifyContent::SpaceEvenly => taffy::style::JustifyContent::SPACE_EVENLY.into(),
        }
    }
}

impl From<Display> for taffy::style::Display {
    fn from(value: Display) -> Self {
        match value {
            Display::Flex => taffy::style::Display::Flex,
            Display::Grid => taffy::style::Display::Grid,
            Display::Block => taffy::style::Display::Block,
            Display::None => taffy::style::Display::None,
        }
    }
}

impl From<BoxSizing> for taffy::style::BoxSizing {
    fn from(value: BoxSizing) -> Self {
        match value {
            BoxSizing::BorderBox => taffy::style::BoxSizing::BorderBox,
            BoxSizing::ContentBox => taffy::style::BoxSizing::ContentBox,
        }
    }
}

impl From<OverflowAxis> for taffy::style::Overflow {
    fn from(value: OverflowAxis) -> Self {
        match value {
            OverflowAxis::Visible => taffy::style::Overflow::Visible,
            OverflowAxis::Clip => taffy::style::Overflow::Clip,
            OverflowAxis::Hidden => taffy::style::Overflow::Hidden,
            OverflowAxis::Scroll => taffy::style::Overflow::Scroll,
        }
    }
}

impl From<FlexDirection> for taffy::style::FlexDirection {
    fn from(value: FlexDirection) -> Self {
        match value {
            FlexDirection::Row => taffy::style::FlexDirection::Row,
            FlexDirection::Column => taffy::style::FlexDirection::Column,
            FlexDirection::RowReverse => taffy::style::FlexDirection::RowReverse,
            FlexDirection::ColumnReverse => taffy::style::FlexDirection::ColumnReverse,
        }
    }
}

impl From<PositionType> for taffy::style::Position {
    fn from(value: PositionType) -> Self {
        match value {
            PositionType::Relative => taffy::style::Position::Relative,
            PositionType::Absolute => taffy::style::Position::Absolute,
        }
    }
}

impl From<FlexWrap> for taffy::style::FlexWrap {
    fn from(value: FlexWrap) -> Self {
        match value {
            FlexWrap::NoWrap => taffy::style::FlexWrap::NoWrap,
            FlexWrap::Wrap => taffy::style::FlexWrap::Wrap,
            FlexWrap::WrapReverse => taffy::style::FlexWrap::WrapReverse,
        }
    }
}

impl From<GridAutoFlow> for taffy::style::GridAutoFlow {
    fn from(value: GridAutoFlow) -> Self {
        match value {
            GridAutoFlow::Row => taffy::style::GridAutoFlow::Row,
            GridAutoFlow::RowDense => taffy::style::GridAutoFlow::RowDense,
            GridAutoFlow::Column => taffy::style::GridAutoFlow::Column,
            GridAutoFlow::ColumnDense => taffy::style::GridAutoFlow::ColumnDense,
        }
    }
}

impl Val {
    fn into_length_percentage_auto(
        self,
        available: &Available
    ) -> taffy::style::LengthPercentageAuto {
        match self {
            Val::Auto => style_helpers::auto(),
            Val::Percent(value) => style_helpers::percent(value / 100.),
            Val::Px(value) => style_helpers::length(value),
            Val::VMin(value) => {
                style_helpers::length(available.size.min_element() * value / 100.)
            }
            Val::VMax(value) => {
                style_helpers::length(available.size.max_element() * value / 100.)
            }
            Val::Vw(value) => style_helpers::length(available.size.x * value / 100.),
            Val::Vh(value) => style_helpers::length(available.size.y * value / 100.),
        }
    }

    fn into_length_percentage(
        self,
        available: &Available
    ) -> taffy::style::LengthPercentage {
        match self {
            Val::Auto => style_helpers::length(0.),
            Val::Percent(value) => style_helpers::percent(value / 100.),
            Val::Px(value) => style_helpers::length(value),
            Val::VMin(value) => {
                style_helpers::length(available.size.min_element() * value / 100.)
            }
            Val::VMax(value) => {
                style_helpers::length(available.size.max_element() * value / 100.)
            }
            Val::Vw(value) => style_helpers::length(available.size.x * value / 100.),
            Val::Vh(value) => style_helpers::length(available.size.y * value / 100.),
        }
    }

    fn into_dimension(self, available: &Available) -> taffy::style::Dimension {
        self.into_length_percentage_auto(available).into()
    }
}


impl UiRect {
    fn map_to_taffy_rect<T>(self, map_fn: impl Fn(Val) -> T) -> taffy::geometry::Rect<T> {
        taffy::geometry::Rect {
            left: map_fn(self.left),
            right: map_fn(self.right),
            top: map_fn(self.top),
            bottom: map_fn(self.bottom),
        }
    }
}