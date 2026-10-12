use engine_macros::Component;

use crate::{
    ecs::components::{Component, Mutable},
    render::color::Color,
    ui::geometry::{UiRect, Val},
};

pub struct Root;

impl Component for Root {
    type Mutability = Mutable;
}

#[derive(Clone, Component, PartialEq, Debug)]
pub struct Node {
    /// Which layout algorithm to use when laying out this node's contents:
    ///   - [`Display::Flex`]: Use the Flexbox layout algorithm
    ///   - [`Display::Grid`]: Use the CSS Grid layout algorithm
    ///   - [`Display::None`]: Hide this node and perform layout as if it does not exist.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/display>
    pub display: Display,

    /// Which part of a Node's box length styles like width and height control
    ///   - [`BoxSizing::BorderBox`]: They refer to the "border box" size (size including padding and border)
    ///   - [`BoxSizing::ContentBox`]: They refer to the "content box" size (size excluding padding and border)
    ///
    /// `BoxSizing::BorderBox` is generally considered more intuitive and is the default in Bevy even though it is not on the web.
    ///
    /// See: <https://developer.mozilla.org/en-US/docs/Web/CSS/box-sizing>
    pub box_sizing: BoxSizing,

    /// Whether a node should be laid out in-flow with, or independently of its siblings:
    ///  - [`PositionType::Relative`]: Layout this node in-flow with other nodes using the usual (flexbox/grid) layout algorithm.
    ///  - [`PositionType::Absolute`]: Layout this node on top and independently of other nodes.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/position>
    pub position_type: PositionType,

    /// Whether overflowing content should be displayed or clipped.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow>
    pub overflow: Overflow,

    /// How much space in logical pixels should be reserved for scrollbars when overflow is set to scroll or auto on an axis.
    pub scrollbar_width: f32,

    /// How the bounds of clipped content should be determined
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-clip-margin>
    pub overflow_clip_margin: OverflowClipMargin,

    /// The horizontal position of the left edge of the node.
    ///  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
    ///  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/left>
    pub left: Val,

    /// The horizontal position of the right edge of the node.
    ///  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
    ///  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/right>
    pub right: Val,

    /// The vertical position of the top edge of the node.
    ///  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
    ///  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/top>
    pub top: Val,

    /// The vertical position of the bottom edge of the node.
    ///  - For relatively positioned nodes, this is relative to the node's position as computed during regular layout.
    ///  - For absolutely positioned nodes, this is relative to the *parent* node's bounding box.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/bottom>
    pub bottom: Val,

    /// The ideal width of the node. `width` is used when it is within the bounds defined by `min_width` and `max_width`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/width>
    pub width: Val,

    /// The ideal height of the node. `height` is used when it is within the bounds defined by `min_height` and `max_height`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/height>
    pub height: Val,

    /// The minimum width of the node. `min_width` is used if it is greater than `width` and/or `max_width`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/min-width>
    pub min_width: Val,

    /// The minimum height of the node. `min_height` is used if it is greater than `height` and/or `max_height`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/min-height>
    pub min_height: Val,

    /// The maximum width of the node. `max_width` is used if it is within the bounds defined by `min_width` and `width`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/max-width>
    pub max_width: Val,

    /// The maximum height of the node. `max_height` is used if it is within the bounds defined by `min_height` and `height`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/max-height>
    pub max_height: Val,

    /// The aspect ratio of the node (defined as `width / height`)
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio>
    pub aspect_ratio: Option<f32>,

    /// Used to control how each individual item is aligned by default within the space they're given.
    /// - For Flexbox containers, sets default cross axis alignment of the child items.
    /// - For CSS Grid containers, controls block (vertical) axis alignment of children of this grid container within their grid areas.
    ///
    /// This value is overridden if [`AlignSelf`] on the child node is set.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-items>
    pub align_items: AlignItems,

    /// Used to control how each individual item is aligned by default within the space they're given.
    /// - For Flexbox containers, this property has no effect. See `justify_content` for main axis alignment of flex items.
    /// - For CSS Grid containers, sets default inline (horizontal) axis alignment of child items within their grid areas.
    ///
    /// This value is overridden if [`JustifySelf`] on the child node is set.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-items>
    pub justify_items: JustifyItems,

    /// Used to control how the specified item is aligned within the space it's given.
    /// - For Flexbox items, controls cross axis alignment of the item.
    /// - For CSS Grid items, controls block (vertical) axis alignment of a grid item within its grid area.
    ///
    /// If set to `Auto`, alignment is inherited from the value of [`AlignItems`] set on the parent node.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-self>
    pub align_self: AlignSelf,

    /// Used to control how the specified item is aligned within the space it's given.
    /// - For Flexbox items, this property has no effect. See `justify_content` for main axis alignment of flex items.
    /// - For CSS Grid items, controls inline (horizontal) axis alignment of a grid item within its grid area.
    ///
    /// If set to `Auto`, alignment is inherited from the value of [`JustifyItems`] set on the parent node.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>
    pub justify_self: JustifySelf,

    /// Used to control how items are distributed.
    /// - For Flexbox containers, controls alignment of lines if `flex_wrap` is set to [`FlexWrap::Wrap`] and there are multiple lines of items.
    /// - For CSS Grid containers, controls alignment of grid rows.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/align-content>
    pub align_content: AlignContent,

    /// Used to control how items are distributed.
    /// - For Flexbox containers, controls alignment of items in the main axis.
    /// - For CSS Grid containers, controls alignment of grid columns.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
    pub justify_content: JustifyContent,

    /// The amount of space around a node outside its border.
    ///
    /// If a percentage value is used, the percentage is calculated based on the width of the parent node.
    ///
    /// # Example
    /// ```
    /// # use bevy_ui::{Node, UiRect, Val};
    /// let node = Node {
    ///     margin: UiRect {
    ///         left: Val::Percent(10.),
    ///         right: Val::Percent(10.),
    ///         top: Val::Percent(15.),
    ///         bottom: Val::Percent(15.)
    ///     },
    ///     ..Default::default()
    /// };
    /// ```
    /// A node with this style and a parent with dimensions of 100px by 300px will have calculated margins of 10px on both left and right edges, and 15px on both top and bottom edges.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/margin>
    pub margin: UiRect,

    /// The amount of space between the edges of a node and its contents.
    ///
    /// If a percentage value is used, the percentage is calculated based on the width of the parent node.
    ///
    /// # Example
    /// ```
    /// # use bevy_ui::{Node, UiRect, Val};
    /// let node = Node {
    ///     padding: UiRect {
    ///         left: Val::Percent(1.),
    ///         right: Val::Percent(2.),
    ///         top: Val::Percent(3.),
    ///         bottom: Val::Percent(4.)
    ///     },
    ///     ..Default::default()
    /// };
    /// ```
    /// A node with this style and a parent with dimensions of 300px by 100px will have calculated padding of 3px on the left, 6px on the right, 9px on the top and 12px on the bottom.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/padding>
    pub padding: UiRect,

    /// The amount of space between the margins of a node and its padding.
    ///
    /// If a percentage value is used, the percentage is calculated based on the width of the parent node.
    ///
    /// The size of the node will be expanded if there are constraints that prevent the layout algorithm from placing the border within the existing node boundary.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-width>
    pub border: UiRect,

    /// Used to add rounded corners to a UI node. You can set a UI node to have uniformly
    /// rounded corners or specify different radii for each corner. If a given radius exceeds half
    /// the length of the smallest dimension between the node's height or width, the radius will
    /// calculated as half the smallest dimension.
    ///
    /// Elliptical nodes are not supported yet. Percentage values are based on the node's smallest
    /// dimension, either width or height.
    ///
    /// # Example
    /// ```rust
    /// # use bevy_ecs::prelude::*;
    /// # use bevy_ui::prelude::*;
    /// # use bevy_color::palettes::basic::{BLUE};
    /// fn setup_ui(mut commands: Commands) {
    ///     commands.spawn((
    ///         Node {
    ///             width: Val::Px(100.),
    ///             height: Val::Px(100.),
    ///             border: UiRect::all(Val::Px(2.)),
    ///             border_radius: BorderRadius::new(
    ///                 // top left
    ///                 Val::Px(10.),
    ///                 // top right
    ///                 Val::Px(20.),
    ///                 // bottom right
    ///                 Val::Px(30.),
    ///                 // bottom left
    ///                 Val::Px(40.),
    ///             ),
    ///             ..Default::default()
    ///         },
    ///         BackgroundColor(BLUE.into()),
    ///     ));
    /// }
    /// ```
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius>
    pub border_radius: BorderRadius,

    /// Whether a Flexbox container should be a row or a column. This property has no effect on Grid nodes.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-direction>
    pub flex_direction: FlexDirection,

    /// Whether a Flexbox container should wrap its contents onto multiple lines if they overflow. This property has no effect on Grid nodes.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-wrap>
    pub flex_wrap: FlexWrap,

    /// Defines how much a flexbox item should grow if there's space available. Defaults to 0 (don't grow at all).
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-grow>
    pub flex_grow: f32,

    /// Defines how much a flexbox item should shrink if there's not enough space available. Defaults to 1.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-shrink>
    pub flex_shrink: f32,

    /// The initial length of a flexbox in the main axis, before flex growing/shrinking properties are applied.
    ///
    /// `flex_basis` overrides `width` (if the main axis is horizontal) or `height` (if the main axis is vertical) when both are set, but it obeys the constraints defined by `min_width`/`min_height` and `max_width`/`max_height`.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/flex-basis>
    pub flex_basis: Val,

    /// The size of the gutters between items in a vertical flexbox layout or between rows in a grid layout.
    ///
    /// Note: Values of `Val::Auto` are not valid and are treated as zero.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/row-gap>
    pub row_gap: Val,

    /// The size of the gutters between items in a horizontal flexbox layout or between column in a grid layout.
    ///
    /// Note: Values of `Val::Auto` are not valid and are treated as zero.
    ///
    /// <https://developer.mozilla.org/en-US/docs/Web/CSS/column-gap>
    pub column_gap: Val,
}

impl Node {
    pub const DEFAULT: Self = Self {
        display: Display::DEFAULT,
        box_sizing: BoxSizing::DEFAULT,
        position_type: PositionType::DEFAULT,
        left: Val::Auto,
        right: Val::Auto,
        top: Val::Auto,
        bottom: Val::Auto,
        flex_direction: FlexDirection::DEFAULT,
        flex_wrap: FlexWrap::DEFAULT,
        align_items: AlignItems::DEFAULT,
        justify_items: JustifyItems::DEFAULT,
        align_self: AlignSelf::DEFAULT,
        justify_self: JustifySelf::DEFAULT,
        align_content: AlignContent::DEFAULT,
        justify_content: JustifyContent::DEFAULT,
        margin: UiRect::DEFAULT,
        padding: UiRect::DEFAULT,
        border: UiRect::DEFAULT,
        border_radius: BorderRadius::DEFAULT,
        flex_grow: 0.0,
        flex_shrink: 1.0,
        flex_basis: Val::Auto,
        width: Val::Auto,
        height: Val::Auto,
        min_width: Val::Auto,
        min_height: Val::Auto,
        max_width: Val::Auto,
        max_height: Val::Auto,
        aspect_ratio: None,
        overflow: Overflow::DEFAULT,
        overflow_clip_margin: OverflowClipMargin::DEFAULT,
        scrollbar_width: 0.,
        row_gap: Val::ZERO,
        column_gap: Val::ZERO,
    };

    pub const VIEWPORT: Self = Self {
        display: Display::DEFAULT,
        box_sizing: BoxSizing::DEFAULT,
        position_type: PositionType::DEFAULT,
        left: Val::Auto,
        right: Val::Auto,
        top: Val::Auto,
        bottom: Val::Auto,
        flex_direction: FlexDirection::DEFAULT,
        flex_wrap: FlexWrap::DEFAULT,
        align_items: AlignItems::DEFAULT,
        justify_items: JustifyItems::DEFAULT,
        align_self: AlignSelf::DEFAULT,
        justify_self: JustifySelf::DEFAULT,
        align_content: AlignContent::DEFAULT,
        justify_content: JustifyContent::DEFAULT,
        margin: UiRect::DEFAULT,
        padding: UiRect::DEFAULT,
        border: UiRect::DEFAULT,
        border_radius: BorderRadius::DEFAULT,
        flex_grow: 0.0,
        flex_shrink: 1.0,
        flex_basis: Val::Auto,
        width: Val::Vw(100.0),
        height: Val::Vh(100.0),
        min_width: Val::Vw(100.0),
        min_height: Val::Vh(100.0),
        max_width: Val::Vw(100.0),
        max_height: Val::Vh(100.0),
        aspect_ratio: None,
        overflow: Overflow::DEFAULT,
        overflow_clip_margin: OverflowClipMargin::DEFAULT,
        scrollbar_width: 0.,
        row_gap: Val::ZERO,
        column_gap: Val::ZERO,
    };
}

impl Default for Node {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AlignItems {
    /// The items are packed in their default position as if no alignment was applied.
    Default,
    /// The items are packed towards the start of the axis.
    Start,
    /// The items are packed towards the end of the axis.
    End,
    /// The items are packed towards the start of the axis, unless the flex direction is reversed;
    /// then they are packed towards the end of the axis.
    FlexStart,
    /// The items are packed towards the end of the axis, unless the flex direction is reversed;
    /// then they are packed towards the start of the axis.
    FlexEnd,
    /// The items are packed along the center of the axis.
    Center,
    /// The items are packed such that their baselines align.
    Baseline,
    /// The items are stretched to fill the space they're given.
    Stretch,
}

impl AlignItems {
    pub const DEFAULT: Self = Self::Default;
}

impl Default for AlignItems {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum JustifyItems {
    /// The items are packed in their default position as if no alignment was applied.
    Default,
    /// The items are packed towards the start of the axis.
    Start,
    /// The items are packed towards the end of the axis.
    End,
    /// The items are packed along the center of the axis
    Center,
    /// The items are packed such that their baselines align.
    Baseline,
    /// The items are stretched to fill the space they're given.
    Stretch,
}

impl JustifyItems {
    pub const DEFAULT: Self = Self::Default;
}

impl Default for JustifyItems {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AlignSelf {
    /// Use the parent node's [`AlignItems`] value to determine how this item should be aligned.
    Auto,
    /// This item will be aligned with the start of the axis.
    Start,
    /// This item will be aligned with the end of the axis.
    End,
    /// This item will be aligned with the start of the axis, unless the flex direction is reversed;
    /// then it will be aligned with the end of the axis.
    FlexStart,
    /// This item will be aligned with the end of the axis, unless the flex direction is reversed;
    /// then it will be aligned with the start of the axis.
    FlexEnd,
    /// This item will be aligned along the center of the axis.
    Center,
    /// This item will be aligned at the baseline.
    Baseline,
    /// This item will be stretched to fill the container.
    Stretch,
}

impl AlignSelf {
    pub const DEFAULT: Self = Self::Auto;
}

impl Default for AlignSelf {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum JustifySelf {
    /// Use the parent node's [`JustifyItems`] value to determine how this item should be aligned.
    Auto,
    /// This item will be aligned with the start of the axis.
    Start,
    /// This item will be aligned with the end of the axis.
    End,
    /// This item will be aligned along the center of the axis.
    Center,
    /// This item will be aligned at the baseline.
    Baseline,
    /// This item will be stretched to fill the space it's given.
    Stretch,
}

impl JustifySelf {
    pub const DEFAULT: Self = Self::Auto;
}

impl Default for JustifySelf {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum AlignContent {
    /// The items are packed in their default position as if no alignment was applied.
    Default,
    /// The items are packed towards the start of the axis.
    Start,
    /// The items are packed towards the end of the axis.
    End,
    /// The items are packed towards the start of the axis, unless the flex direction is reversed;
    /// then the items are packed towards the end of the axis.
    FlexStart,
    /// The items are packed towards the end of the axis, unless the flex direction is reversed;
    /// then the items are packed towards the start of the axis.
    FlexEnd,
    /// The items are packed along the center of the axis.
    Center,
    /// The items are stretched to fill the container along the axis.
    Stretch,
    /// The items are distributed such that the gap between any two items is equal.
    SpaceBetween,
    /// The items are distributed such that the gap between and around any two items is equal.
    SpaceEvenly,
    /// The items are distributed such that the gap between and around any two items is equal, with half-size gaps on either end.
    SpaceAround,
}

impl AlignContent {
    pub const DEFAULT: Self = Self::Default;
}

impl Default for AlignContent {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Used to control how items are distributed.
/// - For Flexbox containers, controls alignment of items in the main axis.
/// - For CSS Grid containers, controls alignment of grid columns.
///
/// <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-content>
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum JustifyContent {
    /// The items are packed in their default position as if no alignment was applied.
    Default,
    /// The items are packed towards the start of the axis.
    Start,
    /// The items are packed towards the end of the axis.
    End,
    /// The items are packed towards the start of the axis, unless the flex direction is reversed;
    /// then the items are packed towards the end of the axis.
    FlexStart,
    /// The items are packed towards the end of the axis, unless the flex direction is reversed;
    /// then the items are packed towards the start of the axis.
    FlexEnd,
    /// The items are packed along the center of the axis.
    Center,
    /// The items are stretched to fill the container along the axis.
    Stretch,
    /// The items are distributed such that the gap between any two items is equal.
    SpaceBetween,
    /// The items are distributed such that the gap between and around any two items is equal.
    SpaceEvenly,
    /// The items are distributed such that the gap between and around any two items is equal, with half-size gaps on either end.
    SpaceAround,
}

impl JustifyContent {
    pub const DEFAULT: Self = Self::Default;
}

impl Default for JustifyContent {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Display {
    Flex,
    Grid,
    Block,
    None,
}

impl Display {
    pub const DEFAULT: Self = Self::Block;
}

impl Default for Display {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BoxSizing {
    /// Length styles like width and height refer to the "border box" size (size including padding and border)
    BorderBox,
    /// Length styles like width and height refer to the "content box" size (size excluding padding and border)
    ContentBox,
}

impl BoxSizing {
    pub const DEFAULT: Self = Self::BorderBox;
}

impl Default for BoxSizing {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Defines how flexbox items are ordered within a flexbox
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FlexDirection {
    /// Same way as text direction along the main axis.
    Row,
    /// Flex from top to bottom.
    Column,
    /// Opposite way as text direction along the main axis.
    RowReverse,
    /// Flex from bottom to top.
    ColumnReverse,
}

impl FlexDirection {
    pub const DEFAULT: Self = Self::Row;
}

impl Default for FlexDirection {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Whether to show or hide overflowing items
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Overflow {
    /// Whether to show or clip overflowing items on the x axis
    pub x: OverflowAxis,
    /// Whether to show or clip overflowing items on the y axis
    pub y: OverflowAxis,
}

impl Overflow {
    pub const DEFAULT: Self = Self {
        x: OverflowAxis::DEFAULT,
        y: OverflowAxis::DEFAULT,
    };

    /// Show overflowing items on both axes
    pub const fn visible() -> Self {
        Self {
            x: OverflowAxis::Visible,
            y: OverflowAxis::Visible,
        }
    }

    /// Clip overflowing items on both axes
    pub const fn clip() -> Self {
        Self {
            x: OverflowAxis::Clip,
            y: OverflowAxis::Clip,
        }
    }

    /// Clip overflowing items on the x axis
    pub const fn clip_x() -> Self {
        Self {
            x: OverflowAxis::Clip,
            y: OverflowAxis::Visible,
        }
    }

    /// Clip overflowing items on the y axis
    pub const fn clip_y() -> Self {
        Self {
            x: OverflowAxis::Visible,
            y: OverflowAxis::Clip,
        }
    }

    /// Hide overflowing items on both axes by influencing layout and then clipping
    pub const fn hidden() -> Self {
        Self {
            x: OverflowAxis::Hidden,
            y: OverflowAxis::Hidden,
        }
    }

    /// Hide overflowing items on the x axis by influencing layout and then clipping
    pub const fn hidden_x() -> Self {
        Self {
            x: OverflowAxis::Hidden,
            y: OverflowAxis::Visible,
        }
    }

    /// Hide overflowing items on the y axis by influencing layout and then clipping
    pub const fn hidden_y() -> Self {
        Self {
            x: OverflowAxis::Visible,
            y: OverflowAxis::Hidden,
        }
    }

    /// Overflow is visible on both axes
    pub const fn is_visible(&self) -> bool {
        self.x.is_visible() && self.y.is_visible()
    }

    pub const fn scroll() -> Self {
        Self {
            x: OverflowAxis::Scroll,
            y: OverflowAxis::Scroll,
        }
    }

    /// Scroll overflowing items on the x axis
    pub const fn scroll_x() -> Self {
        Self {
            x: OverflowAxis::Scroll,
            y: OverflowAxis::Visible,
        }
    }

    /// Scroll overflowing items on the y axis
    pub const fn scroll_y() -> Self {
        Self {
            x: OverflowAxis::Visible,
            y: OverflowAxis::Scroll,
        }
    }
}

impl Default for Overflow {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// Whether to show or hide overflowing items
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum OverflowAxis {
    /// Show overflowing items.
    Visible,
    /// Hide overflowing items by clipping.
    Clip,
    /// Hide overflowing items by influencing layout and then clipping.
    Hidden,
    /// Scroll overflowing items.
    Scroll,
}

impl OverflowAxis {
    pub const DEFAULT: Self = Self::Visible;

    /// Overflow is visible on this axis
    pub const fn is_visible(&self) -> bool {
        matches!(self, Self::Visible)
    }
}

impl Default for OverflowAxis {
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// The bounds of the visible area when a UI node is clipped.
#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct OverflowClipMargin {
    /// Visible unclipped area
    pub visual_box: OverflowClipBox,
    /// Width of the margin on each edge of the visual box in logical pixels.
    /// The width of the margin will be zero if a negative value is set.
    pub margin: f32,
}

impl OverflowClipMargin {
    pub const DEFAULT: Self = Self {
        visual_box: OverflowClipBox::PaddingBox,
        margin: 0.,
    };

    /// Clip any content that overflows outside the content box
    pub const fn content_box() -> Self {
        Self {
            visual_box: OverflowClipBox::ContentBox,
            ..Self::DEFAULT
        }
    }

    /// Clip any content that overflows outside the padding box
    pub const fn padding_box() -> Self {
        Self {
            visual_box: OverflowClipBox::PaddingBox,
            ..Self::DEFAULT
        }
    }

    /// Clip any content that overflows outside the border box
    pub const fn border_box() -> Self {
        Self {
            visual_box: OverflowClipBox::BorderBox,
            ..Self::DEFAULT
        }
    }

    /// Add a margin on each edge of the visual box in logical pixels.
    /// The width of the margin will be zero if a negative value is set.
    pub const fn with_margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
pub enum OverflowClipBox {
    /// Clip any content that overflows outside the content box
    ContentBox,
    /// Clip any content that overflows outside the padding box
    #[default]
    PaddingBox,
    /// Clip any content that overflows outside the border box
    BorderBox,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum PositionType {
    /// Relative to all other nodes with the [`PositionType::Relative`] value.
    Relative,
    /// Independent of all other nodes, but relative to its parent node.
    Absolute,
}

impl PositionType {
    pub const DEFAULT: Self = Self::Relative;
}

impl Default for PositionType {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FlexWrap {
    /// Single line, will overflow if needed.
    NoWrap,
    /// Multiple lines, if needed.
    Wrap,
    /// Same as [`FlexWrap::Wrap`] but new lines will appear before the previous one.
    WrapReverse,
}

impl FlexWrap {
    pub const DEFAULT: Self = Self::NoWrap;
}

impl Default for FlexWrap {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GridAutoFlow {
    /// Items are placed by filling each row in turn, adding new rows as necessary.
    Row,
    /// Items are placed by filling each column in turn, adding new columns as necessary.
    Column,
    /// Combines `Row` with the dense packing algorithm.
    RowDense,
    /// Combines `Column` with the dense packing algorithm.
    ColumnDense,
}

impl GridAutoFlow {
    pub const DEFAULT: Self = Self::Row;
}

impl Default for GridAutoFlow {
    fn default() -> Self {
        Self::DEFAULT
    }
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub enum MinTrackSizingFunction {
    /// Track minimum size should be a fixed pixel value
    Px(f32),
    /// Track minimum size should be a percentage value
    Percent(f32),
    /// Track minimum size should be content sized under a min-content constraint
    MinContent,
    /// Track minimum size should be content sized under a max-content constraint
    MaxContent,
    /// Track minimum size should be automatically sized
    #[default]
    Auto,
    /// Track minimum size should be a percent of the viewport's smaller dimension.
    VMin(f32),
    /// Track minimum size should be a percent of the viewport's larger dimension.
    VMax(f32),
    /// Track minimum size should be a percent of the viewport's height dimension.
    Vh(f32),
    /// Track minimum size should be a percent of the viewport's width dimension.
    Vw(f32),
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub enum MaxTrackSizingFunction {
    /// Track maximum size should be a fixed pixel value
    Px(f32),
    /// Track maximum size should be a percentage value
    Percent(f32),
    /// Track maximum size should be content sized under a min-content constraint
    MinContent,
    /// Track maximum size should be content sized under a max-content constraint
    MaxContent,
    /// Track maximum size should be sized according to the fit-content formula with a fixed pixel limit
    FitContentPx(f32),
    /// Track maximum size should be sized according to the fit-content formula with a percentage limit
    FitContentPercent(f32),
    /// Track maximum size should be automatically sized
    #[default]
    Auto,
    /// The dimension as a fraction of the total available grid space (`fr` units in CSS)
    /// Specified value is the numerator of the fraction. Denominator is the sum of all fractions specified in that grid dimension.
    ///
    /// Spec: <https://www.w3.org/TR/css3-grid-layout/#fr-unit>
    Fraction(f32),
    /// Track maximum size should be a percent of the viewport's smaller dimension.
    VMin(f32),
    /// Track maximum size should be a percent of the viewport's smaller dimension.
    VMax(f32),
    /// Track maximum size should be a percent of the viewport's height dimension.
    Vh(f32),
    /// Track maximum size should be a percent of the viewport's width dimension.
    Vw(f32),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorderRadius {
    pub top_left: Val,
    pub top_right: Val,
    pub bottom_right: Val,
    pub bottom_left: Val,
}

impl Default for BorderRadius {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl BorderRadius {
    pub const DEFAULT: Self = Self::ZERO;

    /// Zero curvature. All the corners will be right-angled.
    pub const ZERO: Self = Self::all(Val::Px(0.));

    /// Maximum curvature. The UI Node will take a capsule shape or circular if width and height are equal.
    pub const MAX: Self = Self::all(Val::Px(f32::MAX));

    #[inline]
    /// Set all four corners to the same curvature.
    pub const fn all(radius: Val) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_left: radius,
            bottom_right: radius,
        }
    }

    #[inline]
    pub const fn new(top_left: Val, top_right: Val, bottom_right: Val, bottom_left: Val) -> Self {
        Self {
            top_left,
            top_right,
            bottom_right,
            bottom_left,
        }
    }

    #[inline]
    /// Sets the radii to logical pixel values.
    pub const fn px(top_left: f32, top_right: f32, bottom_right: f32, bottom_left: f32) -> Self {
        Self {
            top_left: Val::Px(top_left),
            top_right: Val::Px(top_right),
            bottom_right: Val::Px(bottom_right),
            bottom_left: Val::Px(bottom_left),
        }
    }

    #[inline]
    /// Sets the radii to percentage values.
    pub const fn percent(
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32,
    ) -> Self {
        Self {
            top_left: Val::Percent(top_left),
            top_right: Val::Percent(top_right),
            bottom_right: Val::Percent(bottom_right),
            bottom_left: Val::Percent(bottom_left),
        }
    }

    #[inline]
    /// Sets the radius for the top left corner.
    /// Remaining corners will be right-angled.
    pub const fn top_left(radius: Val) -> Self {
        Self {
            top_left: radius,
            ..Self::DEFAULT
        }
    }

    #[inline]
    /// Sets the radius for the top right corner.
    /// Remaining corners will be right-angled.
    pub const fn top_right(radius: Val) -> Self {
        Self {
            top_right: radius,
            ..Self::DEFAULT
        }
    }

    #[inline]
    /// Sets the radius for the bottom right corner.
    /// Remaining corners will be right-angled.
    pub const fn bottom_right(radius: Val) -> Self {
        Self {
            bottom_right: radius,
            ..Self::DEFAULT
        }
    }

    #[inline]
    /// Sets the radius for the bottom left corner.
    /// Remaining corners will be right-angled.
    pub const fn bottom_left(radius: Val) -> Self {
        Self {
            bottom_left: radius,
            ..Self::DEFAULT
        }
    }

    #[inline]
    /// Sets the radii for the top left and bottom left corners.
    /// Remaining corners will be right-angled.
    pub const fn left(radius: Val) -> Self {
        Self {
            top_left: radius,
            bottom_left: radius,
            ..Self::DEFAULT
        }
    }

    #[inline]
    /// Sets the radii for the top right and bottom right corners.
    /// Remaining corners will be right-angled.
    pub const fn right(radius: Val) -> Self {
        Self {
            top_right: radius,
            bottom_right: radius,
            ..Self::DEFAULT
        }
    }

    #[inline]
    /// Sets the radii for the top left and top right corners.
    /// Remaining corners will be right-angled.
    pub const fn top(radius: Val) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            ..Self::DEFAULT
        }
    }

    #[inline]
    /// Sets the radii for the bottom left and bottom right corners.
    /// Remaining corners will be right-angled.
    pub const fn bottom(radius: Val) -> Self {
        Self {
            bottom_left: radius,
            bottom_right: radius,
            ..Self::DEFAULT
        }
    }

    /// Returns the [`BorderRadius`] with its `top_left` field set to the given value.
    #[inline]
    pub const fn with_top_left(mut self, radius: Val) -> Self {
        self.top_left = radius;
        self
    }

    /// Returns the [`BorderRadius`] with its `top_right` field set to the given value.
    #[inline]
    pub const fn with_top_right(mut self, radius: Val) -> Self {
        self.top_right = radius;
        self
    }

    /// Returns the [`BorderRadius`] with its `bottom_right` field set to the given value.
    #[inline]
    pub const fn with_bottom_right(mut self, radius: Val) -> Self {
        self.bottom_right = radius;
        self
    }

    /// Returns the [`BorderRadius`] with its `bottom_left` field set to the given value.
    #[inline]
    pub const fn with_bottom_left(mut self, radius: Val) -> Self {
        self.bottom_left = radius;
        self
    }

    /// Returns the [`BorderRadius`] with its `top_left` and `bottom_left` fields set to the given value.
    #[inline]
    pub const fn with_left(mut self, radius: Val) -> Self {
        self.top_left = radius;
        self.bottom_left = radius;
        self
    }

    /// Returns the [`BorderRadius`] with its `top_right` and `bottom_right` fields set to the given value.
    #[inline]
    pub const fn with_right(mut self, radius: Val) -> Self {
        self.top_right = radius;
        self.bottom_right = radius;
        self
    }

    /// Returns the [`BorderRadius`] with its `top_left` and `top_right` fields set to the given value.
    #[inline]
    pub const fn with_top(mut self, radius: Val) -> Self {
        self.top_left = radius;
        self.top_right = radius;
        self
    }

    /// Returns the [`BorderRadius`] with its `bottom_left` and `bottom_right` fields set to the given value.
    #[inline]
    pub const fn with_bottom(mut self, radius: Val) -> Self {
        self.bottom_left = radius;
        self.bottom_right = radius;
        self
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BackgroundColor(pub Color);

impl Component for BackgroundColor {
    type Mutability = Mutable;
}

impl BackgroundColor {
    pub const DEFAULT: Self = Self(Color::NONE);
}

impl Default for BackgroundColor {
    fn default() -> Self {
        Self::DEFAULT
    }
}
