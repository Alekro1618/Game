use std::ops::{Div, DivAssign, Mul, MulAssign, Neg};

use taffy::style_helpers;

use crate::{
    error::error::{Error, ErrorKind}, math::vec::Vec2,
};

#[derive(Copy, Clone, Debug)]

pub enum Val {
    /// Automatically determine the value based on the context and other [`Node`](crate::Node) properties.
    Auto,
    /// Set this value in logical pixels.
    Px(f32),
    /// Set the value as a percentage of its parent node's length along a specific axis.
    ///
    /// If the UI node has no parent, the percentage is calculated based on the window's length
    /// along the corresponding axis.
    ///
    /// The chosen axis depends on the [`Node`](crate::Node) field set:
    /// * For `flex_basis`, the percentage is relative to the main-axis length determined by the `flex_direction`.
    /// * For `gap`, `min_size`, `size`, and `max_size`:
    ///   - `width` is relative to the parent's width.
    ///   - `height` is relative to the parent's height.
    /// * For `margin`, `padding`, and `border` values: the percentage is relative to the parent node's width.
    /// * For positions, `left` and `right` are relative to the parent's width, while `bottom` and `top` are relative to the parent's height.
    Percent(f32),
    /// Set this value in percent of the viewport width
    Vw(f32),
    /// Set this value in percent of the viewport height
    Vh(f32),
    /// Set this value in percent of the viewport's smaller dimension.
    VMin(f32),
    /// Set this value in percent of the viewport's larger dimension.
    VMax(f32),
}

impl PartialEq for Val {
    fn eq(&self, other: &Self) -> bool {
        let same_unit = matches!(
            (self, other),
            (Self::Auto, Self::Auto)
                | (Self::Px(_), Self::Px(_))
                | (Self::Percent(_), Self::Percent(_))
                | (Self::Vw(_), Self::Vw(_))
                | (Self::Vh(_), Self::Vh(_))
                | (Self::VMin(_), Self::VMin(_))
                | (Self::VMax(_), Self::VMax(_))
        );

        let left = match self {
            Self::Auto => None,
            Self::Px(v)
            | Self::Percent(v)
            | Self::Vw(v)
            | Self::Vh(v)
            | Self::VMin(v)
            | Self::VMax(v) => Some(v),
        };

        let right = match other {
            Self::Auto => None,
            Self::Px(v)
            | Self::Percent(v)
            | Self::Vw(v)
            | Self::Vh(v)
            | Self::VMin(v)
            | Self::VMax(v) => Some(v),
        };

        match (same_unit, left, right) {
            (true, a, b) => a == b,
            // All zero-value variants are considered equal.
            (false, Some(&a), Some(&b)) => a == 0. && b == 0.,
            _ => false,
        }
    }
}

impl Val {
    pub const DEFAULT: Self = Self::Auto;
    pub const ZERO: Self = Self::Px(0.0);

    /// Returns a [`UiRect`] with its `left` equal to this value,
    /// and all other fields set to `Val::ZERO`.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_ui::{UiRect, Val};
    /// #
    /// let ui_rect = Val::Px(1.).left();
    ///
    /// assert_eq!(ui_rect.left, Val::Px(1.));
    /// assert_eq!(ui_rect.right, Val::ZERO);
    /// assert_eq!(ui_rect.top, Val::ZERO);
    /// assert_eq!(ui_rect.bottom, Val::ZERO);
    /// ```
    pub const fn left(self) -> UiRect {
        UiRect::left(self)
    }

    /// Returns a [`UiRect`] with its `right` equal to this value,
    /// and all other fields set to `Val::ZERO`.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_ui::{UiRect, Val};
    /// #
    /// let ui_rect = Val::Px(1.).right();
    ///
    /// assert_eq!(ui_rect.left, Val::ZERO);
    /// assert_eq!(ui_rect.right, Val::Px(1.));
    /// assert_eq!(ui_rect.top, Val::ZERO);
    /// assert_eq!(ui_rect.bottom, Val::ZERO);
    /// ```
    pub const fn right(self) -> UiRect {
        UiRect::right(self)
    }

    /// Returns a [`UiRect`] with its `top` equal to this value,
    /// and all other fields set to `Val::ZERO`.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_ui::{UiRect, Val};
    /// #
    /// let ui_rect = Val::Px(1.).top();
    ///
    /// assert_eq!(ui_rect.left, Val::ZERO);
    /// assert_eq!(ui_rect.right, Val::ZERO);
    /// assert_eq!(ui_rect.top, Val::Px(1.));
    /// assert_eq!(ui_rect.bottom, Val::ZERO);
    /// ```
    pub const fn top(self) -> UiRect {
        UiRect::top(self)
    }

    /// Returns a [`UiRect`] with its `bottom` equal to this value,
    /// and all other fields set to `Val::ZERO`.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_ui::{UiRect, Val};
    /// #
    /// let ui_rect = Val::Px(1.).bottom();
    ///
    /// assert_eq!(ui_rect.left, Val::ZERO);
    /// assert_eq!(ui_rect.right, Val::ZERO);
    /// assert_eq!(ui_rect.top, Val::ZERO);
    /// assert_eq!(ui_rect.bottom, Val::Px(1.));
    /// ```
    pub const fn bottom(self) -> UiRect {
        UiRect::bottom(self)
    }

    /// Returns a [`UiRect`] with all its fields equal to this value.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_ui::{UiRect, Val};
    /// #
    /// let ui_rect = Val::Px(1.).all();
    ///
    /// assert_eq!(ui_rect.left, Val::Px(1.));
    /// assert_eq!(ui_rect.right, Val::Px(1.));
    /// assert_eq!(ui_rect.top, Val::Px(1.));
    /// assert_eq!(ui_rect.bottom, Val::Px(1.));
    /// ```
    pub const fn all(self) -> UiRect {
        UiRect::all(self)
    }

    /// Returns a [`UiRect`] with all its `left` and `right` equal to this value,
    /// and its `top` and `bottom` set to `Val::ZERO`.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_ui::{UiRect, Val};
    /// #
    /// let ui_rect = Val::Px(1.).horizontal();
    ///
    /// assert_eq!(ui_rect.left, Val::Px(1.));
    /// assert_eq!(ui_rect.right, Val::Px(1.));
    /// assert_eq!(ui_rect.top, Val::ZERO);
    /// assert_eq!(ui_rect.bottom, Val::ZERO);
    /// ```
    pub const fn horizontal(self) -> UiRect {
        UiRect::horizontal(self)
    }

    /// Returns a [`UiRect`] with all its `top` and `bottom` equal to this value,
    /// and its `left` and `right` set to `Val::ZERO`.
    ///
    /// # Example
    ///
    /// ```
    /// # use bevy_ui::{UiRect, Val};
    /// #
    /// let ui_rect = Val::Px(1.).vertical();
    ///
    /// assert_eq!(ui_rect.left, Val::ZERO);
    /// assert_eq!(ui_rect.right, Val::ZERO);
    /// assert_eq!(ui_rect.top, Val::Px(1.));
    /// assert_eq!(ui_rect.bottom, Val::Px(1.));
    /// ```
    pub const fn vertical(self) -> UiRect {
        UiRect::vertical(self)
    }
}

impl Default for Val {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Mul<f32> for Val {
    type Output = Val;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Val::Auto => Val::Auto,
            Val::Px(value) => Val::Px(value * rhs),
            Val::Percent(value) => Val::Percent(value * rhs),
            Val::Vw(value) => Val::Vw(value * rhs),
            Val::Vh(value) => Val::Vh(value * rhs),
            Val::VMin(value) => Val::VMin(value * rhs),
            Val::VMax(value) => Val::VMax(value * rhs),
        }
    }
}

impl MulAssign<f32> for Val {
    fn mul_assign(&mut self, rhs: f32) {
        match self {
            Val::Auto => {}
            Val::Px(value)
            | Val::Percent(value)
            | Val::Vw(value)
            | Val::Vh(value)
            | Val::VMin(value)
            | Val::VMax(value) => *value *= rhs,
        }
    }
}

impl Div<f32> for Val {
    type Output = Val;

    fn div(self, rhs: f32) -> Self::Output {
        match self {
            Val::Auto => Val::Auto,
            Val::Px(value) => Val::Px(value / rhs),
            Val::Percent(value) => Val::Percent(value / rhs),
            Val::Vw(value) => Val::Vw(value / rhs),
            Val::Vh(value) => Val::Vh(value / rhs),
            Val::VMin(value) => Val::VMin(value / rhs),
            Val::VMax(value) => Val::VMax(value / rhs),
        }
    }
}

impl DivAssign<f32> for Val {
    fn div_assign(&mut self, rhs: f32) {
        match self {
            Val::Auto => {}
            Val::Px(value)
            | Val::Percent(value)
            | Val::Vw(value)
            | Val::Vh(value)
            | Val::VMin(value)
            | Val::VMax(value) => *value /= rhs,
        }
    }
}

impl Neg for Val {
    type Output = Val;

    fn neg(self) -> Self::Output {
        match self {
            Val::Px(value) => Val::Px(-value),
            Val::Percent(value) => Val::Percent(-value),
            Val::Vw(value) => Val::Vw(-value),
            Val::Vh(value) => Val::Vh(-value),
            Val::VMin(value) => Val::VMin(-value),
            Val::VMax(value) => Val::VMax(-value),
            _ => self,
        }
    }
}

impl Val {
    /// Resolves this [`Val`] to a value in physical pixels from the given `scale_factor`, `physical_base_value`,
    /// and `physical_target_size` context values.
    ///
    /// Returns a [`ValArithmeticError::NonEvaluable`] if the [`Val`] is impossible to resolve into a concrete value.
    pub fn resolve(
        self,
        physical_base_value: f32,
        physical_target_size: Vec2,
    ) -> Result<f32, Error> {
        match self {
            Val::Percent(value) => Ok(physical_base_value * value / 100.0),
            Val::Px(value) => Ok(value),
            Val::Vw(value) => Ok(physical_target_size.x * value / 100.0),
            Val::Vh(value) => Ok(physical_target_size.y * value / 100.0),
            Val::VMin(value) => {
                Ok(physical_target_size.x.min(physical_target_size.y) * value / 100.0)
            }
            Val::VMax(value) => {
                Ok(physical_target_size.x.max(physical_target_size.y) * value / 100.0)
            }
            Val::Auto => Err(Error::new(ErrorKind::UI, "Can't resolve value for auto")),
        }
    }
}



/// All the types that should be able to be used in the [`Val`] enum should implement this trait.
///
/// Instead of just implementing `Into<Val>` a custom trait is added.
/// This is done in order to prevent having to define a default unit, which could lead to confusion especially for newcomers.
pub trait ValNum {
    /// Called by the [`Val`] helper functions to convert the implementing type to an `f32` that can
    /// be used by [`Val`].
    fn val_num_f32(self) -> f32;
}

macro_rules! impl_to_val_num {
    ($($impl_type:ty),*$(,)?) => {
        $(
            impl ValNum for $impl_type {
                fn val_num_f32(self) -> f32 {
                    self as f32
                }
            }
        )*
    };
}

impl_to_val_num!(f32, f64, i8, i16, i32, i64, u8, u16, u32, u64, usize, isize);

/// Returns a [`Val::Auto`] where the value is automatically determined
/// based on the context and other [`Node`](crate::Node) properties.
pub const fn auto() -> Val {
    Val::Auto
}

/// Returns a [`Val::Px`] representing a value in logical pixels.
pub fn px<T: ValNum>(value: T) -> Val {
    Val::Px(value.val_num_f32())
}

/// Returns a [`Val::Percent`] representing a percentage of the parent node's length
/// along a specific axis.
///
/// If the UI node has no parent, the percentage is based on the window's length
/// along that axis.
///
/// Axis rules:
/// * For `flex_basis`, the percentage is relative to the main-axis length determined by the `flex_direction`.
/// * For `gap`, `min_size`, `size`, and `max_size`:
///   - `width` is relative to the parent's width.
///   - `height` is relative to the parent's height.
/// * For `margin`, `padding`, and `border` values: the percentage is relative to the parent's width.
/// * For positions, `left` and `right` are relative to the parent's width, while `bottom` and `top` are relative to the parent's height.
pub fn percent<T: ValNum>(value: T) -> Val {
    Val::Percent(value.val_num_f32())
}

/// Returns a [`Val::Vw`] representing a percentage of the viewport width.
pub fn vw<T: ValNum>(value: T) -> Val {
    Val::Vw(value.val_num_f32())
}

/// Returns a [`Val::Vh`] representing a percentage of the viewport height.
pub fn vh<T: ValNum>(value: T) -> Val {
    Val::Vh(value.val_num_f32())
}

/// Returns a [`Val::VMin`] representing a percentage of the viewport's smaller dimension.
pub fn vmin<T: ValNum>(value: T) -> Val {
    Val::VMin(value.val_num_f32())
}

/// Returns a [`Val::VMax`] representing a percentage of the viewport's larger dimension.
pub fn vmax<T: ValNum>(value: T) -> Val {
    Val::VMax(value.val_num_f32())
}

/// A type which is commonly used to define margins, paddings and borders.
///
/// # Examples
///
/// ## Margin
///
/// A margin is used to create space around UI elements, outside of any defined borders.
///
/// ```
/// # use bevy_ui::{UiRect, Val};
/// #
/// let margin = UiRect::all(Val::Auto); // Centers the UI element
/// ```
///
/// ## Padding
///
/// A padding is used to create space around UI elements, inside of any defined borders.
///
/// ```
/// # use bevy_ui::{UiRect, Val};
/// #
/// let padding = UiRect {
///     left: Val::Px(10.0),
///     right: Val::Px(20.0),
///     top: Val::Px(30.0),
///     bottom: Val::Px(40.0),
/// };
/// ```
///
/// ## Borders
///
/// A border is used to define the width of the border of a UI element.
///
/// ```
/// # use bevy_ui::{UiRect, Val};
/// #
/// let border = UiRect {
///     left: Val::Px(10.0),
///     right: Val::Px(20.0),
///     top: Val::Px(30.0),
///     bottom: Val::Px(40.0),
/// };
/// ```
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct UiRect {
    /// The value corresponding to the left side of the UI rect.
    pub left: Val,
    /// The value corresponding to the right side of the UI rect.
    pub right: Val,
    /// The value corresponding to the top side of the UI rect.
    pub top: Val,
    /// The value corresponding to the bottom side of the UI rect.
    pub bottom: Val,
}

impl UiRect {
    pub const DEFAULT: Self = Self::all(Val::ZERO);
    pub const ZERO: Self = Self::all(Val::ZERO);
    pub const AUTO: Self = Self::all(Val::Auto);

    pub const fn new(left: Val, right: Val, top: Val, bottom: Val) -> Self {
        UiRect {
            left,
            right,
            top,
            bottom,
        }
    }

    pub const fn all(value: Val) -> Self {
        UiRect {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    pub const fn px(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        UiRect {
            left: Val::Px(left),
            right: Val::Px(right),
            top: Val::Px(top),
            bottom: Val::Px(bottom),
        }
    }

    pub const fn percent(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        UiRect {
            left: Val::Percent(left),
            right: Val::Percent(right),
            top: Val::Percent(top),
            bottom: Val::Percent(bottom),
        }
    }

    pub const fn horizontal(value: Val) -> Self {
        Self {
            left: value,
            right: value,
            ..Self::DEFAULT
        }
    }

    pub const fn vertical(value: Val) -> Self {
        Self {
            top: value,
            bottom: value,
            ..Self::DEFAULT
        }
    }

    pub const fn axes(horizontal: Val, vertical: Val) -> Self {
        Self {
            left: horizontal,
            right: horizontal,
            top: vertical,
            bottom: vertical,
        }
    }

    pub const fn left(left: Val) -> Self {
        Self {
            left,
            ..Self::DEFAULT
        }
    }

    pub const fn right(right: Val) -> Self {
        Self {
            right,
            ..Self::DEFAULT
        }
    }

    pub const fn top(top: Val) -> Self {
        Self {
            top,
            ..Self::DEFAULT
        }
    }

    pub const fn bottom(bottom: Val) -> Self {
        Self {
            bottom,
            ..Self::DEFAULT
        }
    }

    #[inline]
    pub const fn with_left(mut self, left: Val) -> Self {
        self.left = left;
        self
    }

    #[inline]
    pub const fn with_right(mut self, right: Val) -> Self {
        self.right = right;
        self
    }

    #[inline]
    pub const fn with_top(mut self, top: Val) -> Self {
        self.top = top;
        self
    }

    #[inline]
    pub const fn with_bottom(mut self, bottom: Val) -> Self {
        self.bottom = bottom;
        self
    }
}

impl Default for UiRect {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl From<Val> for UiRect {
    fn from(value: Val) -> Self {
        UiRect::all(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Responsive position relative to a UI node.
pub struct UiPosition {
    /// Normalized anchor point
    pub anchor: Vec2,
    /// Responsive horizontal position relative to the anchor point
    pub x: Val,
    /// Responsive vertical position relative to the anchor point
    pub y: Val,
}

impl Default for UiPosition {
    fn default() -> Self {
        Self::CENTER
    }
}

impl UiPosition {
    /// Position at the given normalized anchor point
    pub const fn anchor(anchor: Vec2) -> Self {
        Self {
            anchor,
            x: Val::ZERO,
            y: Val::ZERO,
        }
    }

    /// Position at the top-left corner
    pub const TOP_LEFT: Self = Self::anchor(Vec2::new(-0.5, -0.5));

    /// Position at the center of the left edge
    pub const LEFT: Self = Self::anchor(Vec2::new(-0.5, 0.0));

    /// Position at the bottom-left corner
    pub const BOTTOM_LEFT: Self = Self::anchor(Vec2::new(-0.5, 0.5));

    /// Position at the center of the top edge
    pub const TOP: Self = Self::anchor(Vec2::new(0.0, -0.5));

    /// Position at the center of the element
    pub const CENTER: Self = Self::anchor(Vec2::new(0.0, 0.0));

    /// Position at the center of the bottom edge
    pub const BOTTOM: Self = Self::anchor(Vec2::new(0.0, 0.5));

    /// Position at the top-right corner
    pub const TOP_RIGHT: Self = Self::anchor(Vec2::new(0.5, -0.5));

    /// Position at the center of the right edge
    pub const RIGHT: Self = Self::anchor(Vec2::new(0.5, 0.0));

    /// Position at the bottom-right corner
    pub const BOTTOM_RIGHT: Self = Self::anchor(Vec2::new(0.5, 0.5));

    /// Create a new position
    pub const fn new(anchor: Vec2, x: Val, y: Val) -> Self {
        Self { anchor, x, y }
    }

    /// Creates a position from self with the given `x` and `y` coordinates
    pub const fn at(self, x: Val, y: Val) -> Self {
        Self { x, y, ..self }
    }

    /// Creates a position from self with the given `x` coordinate
    pub const fn at_x(self, x: Val) -> Self {
        Self { x, ..self }
    }

    /// Creates a position from self with the given `y` coordinate
    pub const fn at_y(self, y: Val) -> Self {
        Self { y, ..self }
    }

    /// Creates a position in logical pixels from self with the given `x` and `y` coordinates
    pub const fn at_px(self, x: f32, y: f32) -> Self {
        self.at(Val::Px(x), Val::Px(y))
    }

    /// Creates a percentage position from self with the given `x` and `y` coordinates
    pub const fn at_percent(self, x: f32, y: f32) -> Self {
        self.at(Val::Percent(x), Val::Percent(y))
    }

    /// Creates a position from self with the given `anchor` point
    pub const fn with_anchor(self, anchor: Vec2) -> Self {
        Self { anchor, ..self }
    }

    /// Position relative to the top-left corner
    pub const fn top_left(x: Val, y: Val) -> Self {
        Self::TOP_LEFT.at(x, y)
    }

    /// Position relative to the left edge
    pub const fn left(x: Val, y: Val) -> Self {
        Self::LEFT.at(x, y)
    }

    /// Position relative to the bottom-left corner
    pub const fn bottom_left(x: Val, y: Val) -> Self {
        Self::BOTTOM_LEFT.at(x, y)
    }

    /// Position relative to the top edge
    pub const fn top(x: Val, y: Val) -> Self {
        Self::TOP.at(x, y)
    }

    /// Position relative to the center
    pub const fn center(x: Val, y: Val) -> Self {
        Self::CENTER.at(x, y)
    }

    /// Position relative to the bottom edge
    pub const fn bottom(x: Val, y: Val) -> Self {
        Self::BOTTOM.at(x, y)
    }

    /// Position relative to the top-right corner
    pub const fn top_right(x: Val, y: Val) -> Self {
        Self::TOP_RIGHT.at(x, y)
    }

    /// Position relative to the right edge
    pub const fn right(x: Val, y: Val) -> Self {
        Self::RIGHT.at(x, y)
    }

    /// Position relative to the bottom-right corner
    pub const fn bottom_right(x: Val, y: Val) -> Self {
        Self::BOTTOM_RIGHT.at(x, y)
    }

    /// Resolves the `Position` into physical coordinates.
    pub fn resolve(self, physical_size: Vec2, physical_target_size: Vec2) -> Vec2 {
        let d = self.anchor.map(|p| if 0. < p { -1. } else { 1. });

        physical_size * self.anchor
            + d * Vec2::new(
                self.x
                    .resolve(physical_size.x, physical_target_size)
                    .unwrap_or(0.),
                self.y
                    .resolve(physical_size.y, physical_target_size)
                    .unwrap_or(0.),
            )
    }
}

impl From<Val> for UiPosition {
    fn from(x: Val) -> Self {
        Self {
            x,
            ..Self::default()
        }
    }
}

impl From<(Val, Val)> for UiPosition {
    fn from((x, y): (Val, Val)) -> Self {
        Self {
            x,
            y,
            ..Self::default()
        }
    }
}
