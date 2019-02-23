use crate::{Coordinate, CoordinateType};

/// A bounded 2D quadrilateral whose area is defined by minimum and maximum `Coordinates`.
#[derive(PartialEq, Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect<T>
where
    T: CoordinateType,
{
    pub min: Coordinate<T>,
    pub max: Coordinate<T>,
}

impl<T: CoordinateType> Rect<T> {
    /// Create a new `Rect` with this `min` & `max` point
    ///
    /// ```
    /// # use geo_types::Rect;
    /// let r = Rect::new((0., 0.), (10., 10.));
    /// ```
    pub fn new(min: impl Into<Coordinate<T>>, max: impl Into<Coordinate<T>>) -> Self {
        Rect {
            min: min.into(),
            max: max.into(),
        }
    }
    pub fn width(self) -> T {
        self.max.x - self.min.x
    }

    pub fn height(self) -> T {
        self.max.y - self.min.y
    }
}
