use std::mem;

use crate::Shape;

/// A 2-dimensional shape
#[derive(Clone, Debug)]
#[repr(C)]
pub enum Shape2d {
    /// A circle
    Circle(Circle),

    /// A difference between two shapes
    Difference(Box<Difference2d>),

    /// A sketch
    Sketch(Sketch),
}

/// A circle
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Circle {
    /// The radius of the circle
    radius: f64,
}

impl Circle {
    pub fn from_radius (radius: f64) -> Self {
        Self { radius }
    }

    pub fn radius (&self) -> f64 {
        self.radius
    }
}

impl From<Circle> for Shape {
    fn from(shape: Circle) -> Self {
        Self::Shape2d(Shape2d::Circle(shape))
    }
}

impl From<Circle> for Shape2d {
    fn from(shape: Circle) -> Self {
        Self::Circle(shape)
    }
}

/// A difference between two shapes
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Difference2d {
    /// The original shape
    pub a: Shape2d,

    /// The shape being subtracted
    pub b: Shape2d,
}

impl From<Difference2d> for Shape {
    fn from(shape: Difference2d) -> Self {
        Self::Shape2d(Shape2d::Difference(Box::new(shape)))
    }
}

impl From<Difference2d> for Shape2d {
    fn from(shape: Difference2d) -> Self {
        Self::Difference(Box::new(shape))
    }
}

/// A sketch
///
/// Sketches are currently limited to a single cycle of straight lines,
/// represented by a number of points. For example, if the points a, b, and c
/// are provided, the edges ab, bc, and ca are assumed.
///
/// Nothing about these edges is checked right now, but algorithms might assume
/// that the edges are non-overlapping. If you create a `Sketch` with
/// overlapping edges, you're on your own.
#[derive(Clone, Debug)]
#[repr(C)]
pub struct Sketch {
    // The fields are the raw parts of a `Vec`. `Sketch` needs to be FFI-safe,
    // meaning it can't store a `Vec` directly. It needs to take this detour.
    ptr: *mut [f64; 2],
    length: usize,
    capacity: usize,
}

impl Sketch {
    /// Create a sketch from a bunch of points
    pub fn from_points(mut points: Vec<[f64; 2]>) -> Self {
        // This can be cleaned up, once `Vec::into_raw_parts` is stable.
        let ptr = points.as_mut_ptr();
        let length = points.len();
        let capacity = points.capacity();

        // We're taking ownership of the memory here, so we can't allow `points`
        // to deallocate it.
        mem::forget(points);

        Self {
            ptr,
            length,
            capacity,
        }
    }

    /// Return the points of the sketch
    pub fn to_points(&self) -> Vec<[f64; 2]> {
        // This is sound. All invariants are automatically kept, as the raw
        // parts come from an original `Vec` that is identical to the new one we
        // create here, and aren't being modified anywhere.
        let points = unsafe {
            Vec::from_raw_parts(self.ptr, self.length, self.capacity)
        };

        // Ownership of the pointer in `self.raw_parts` transferred to `points`.
        // We work around that, by returning a clone of `points` (hence not
        // giving ownership to the caller).
        let ret = points.clone();

        // Now we just need to forget that `points` ever existed, and we keep
        // ownership of the pointer.
        mem::forget(points);

        ret
    }
}

impl From<Sketch> for Shape {
    fn from(shape: Sketch) -> Self {
        Self::Shape2d(Shape2d::Sketch(shape))
    }
}

impl From<Sketch> for Shape2d {
    fn from(shape: Sketch) -> Self {
        Self::Sketch(shape)
    }
}

// `Sketch` can be `Send`, because it encapsulates the raw pointer it contains,
// making sure memory ownership rules are observed.
unsafe impl Send for Sketch {}
