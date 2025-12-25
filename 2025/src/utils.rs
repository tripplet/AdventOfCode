pub const NEIGHBORS_2D: &[(i32, i32)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2<T> {
    pub y: T,
    pub x: T,
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            y: value.0,
            x: value.1,
        }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add<Vec2<T>> for Vec2<T>
where
    T: Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            y: self.y + rhs.x,
            x: self.x + rhs.x,
        }
    }
}

/// Macro to create a new `Vec2<T>` instance, order is y, x
/// ```rust
/// vec!(2, 4);
/// ```
#[allow(unused_macros)]
macro_rules! vec2 {
    ($y:expr, $x:expr) => {
        crate::utils::Vec2 { y: $y, x: $x }
    };
}
