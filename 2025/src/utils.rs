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

#[derive(Debug, Copy, Clone)]
pub struct Pos2<T> {
    pub y: T,
    pub x: T,
}

impl<T> From<(T, T)> for Pos2<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            y: value.0,
            x: value.1,
        }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add<Pos2<T>> for Pos2<T>
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

macro_rules! pos2 {
    ($y:expr, $x:expr) => {
        crate::utils::Pos2 { y: $y, x: $x }
    };
}
