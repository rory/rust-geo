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

    /// Width of this Rect
    pub fn width(self) -> T {
        self.max.x - self.min.x
    }

    /// Height of this Rect
    pub fn height(self) -> T {
        self.max.y - self.min.y
    }

    /// Return a new Rect which covers both `self` and `other`
    ///
    /// It's possible that the returned Rect will include points not covered by either of the
    /// original Rects.
    ///
    /// ```
    /// # use geo_types::Rect;
    /// let r1 = Rect::new((0, 0), (10, 10));
    /// // `other` overlaps the Rect, no change
    /// assert_eq!(r1.combined(&Rect::new((1, 1), (5, 5))), r1);
    /// // Here a larger Rect is returned
    /// assert_eq!(r1.combined(&Rect::new((1, 1), (50, 50))), Rect::new((0, 0), (50, 50)));
    /// assert_eq!(r1.combined(&Rect::new((5, 1), (15, 2))), Rect::new((0, 0), (15, 10)));
    /// ```
    pub fn combined(&self, other: &Rect<T>) -> Rect<T> {
        Rect {
            min: Coordinate {
                x: simple_min(self.min.x, other.min.x),
                y: simple_min(self.min.y, other.min.y),
            },
            max: Coordinate {
                x: simple_max(self.max.x, other.max.x),
                y: simple_max(self.max.y, other.max.y),
            },
        }
    }
}

/// Can't use std::cmp::min on Float etc.
fn simple_min<T: CoordinateType>(a: T, b: T) -> T {
    if a <= b {
        a
    } else {
        b
    }
}

/// Can't use std::cmp::max on Float etc.
fn simple_max<T: CoordinateType>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}
