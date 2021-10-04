use std::cmp::min;

use decorum::R32;

use crate::math::Point;

// TASK: Document.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Triangle<const D: usize> {
    points: [nalgebra::Point<R32, D>; 3],
}

impl<const D: usize> Triangle<D> {
    /// Create a new `Triangle`
    pub fn new(
        a: impl Into<Point<D>>,
        b: impl Into<Point<D>>,
        c: impl Into<Point<D>>,
    ) -> Result<Self, Error> {
        let a = a.into();
        let b = b.into();
        let c = c.into();

        if a == b || a == c || b == c {
            return Err(Error::CollapsedPoints);
        }
        if (b - a).normalize() == (c - b).normalize() {
            return Err(Error::IsALineSegment);
        }

        let a = a.map(|coord| coord.into());
        let b = b.map(|coord| coord.into());
        let c = c.map(|coord| coord.into());

        let min = min(a.coords.data.0, min(b.coords.data.0, c.coords.data.0));
        let min = nalgebra::Point::from(min[0]);

        let (a, b, c) = if a == min {
            (a, b, c)
        } else if b == min {
            (b, c, a)
        } else {
            (c, a, b)
        };

        Ok(Self { points: [a, b, c] })
    }

    /// Return the points of the triangle
    pub fn points(&self) -> [Point<D>; 3] {
        self.points.map(|point| point.map(|coord| coord.into()))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    CollapsedPoints,
    IsALineSegment,
}

#[cfg(test)]
mod tests {
    use nalgebra::point;

    use crate::math::Point;

    use super::{Error, Triangle};

    #[test]
    fn validation() {
        let triangle =
            Triangle::new(point![0., 0.], point![0., 1.], point![1., 1.]);
        let points_on_a_line =
            Triangle::new(point![0., 0.], point![1., 1.], point![2., 2.]);
        let collapsed_points =
            Triangle::new(point![0., 0.], point![1., 1.], point![1., 1.]);

        assert!(triangle.is_ok());
        assert_eq!(points_on_a_line, Err(Error::IsALineSegment));
        assert_eq!(collapsed_points, Err(Error::CollapsedPoints));
    }

    #[test]
    fn normalization() {
        let a = point![0., 0.];
        let b = point![0., 1.];
        let c = point![1., 1.];

        // Test with triangles in both directions, to make sure the
        // normalization preserves direction.
        test(a, b, c);
        test(a, c, b);

        fn test<const D: usize>(a: Point<D>, b: Point<D>, c: Point<D>) {
            let abc = Triangle::new(a, b, c).unwrap();
            let bca = Triangle::new(b, c, a).unwrap();
            let cab = Triangle::new(c, a, b).unwrap();

            assert_eq!(abc.points(), bca.points());
            assert_eq!(abc.points(), cab.points());

            // But don't change order of triangle points.
            assert!(
                abc.points() == [a, b, c]
                    || abc.points() == [b, c, a]
                    || abc.points() == [c, b, a]
            );
        }
    }
}
