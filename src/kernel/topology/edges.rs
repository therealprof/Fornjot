use crate::{
    kernel::geometry::{Circle, Curve, Line},
    math::{Point, Transform, Vector},
};

use super::vertices::Vertex;

/// The edges of a shape
#[derive(Clone)]
pub struct Edges {
    /// The cycles that the edges of the shape form
    ///
    /// Code reading this field generally assumes that cycles do not overlap.
    /// This precondition is currently not checked, and must be upheld by all
    /// code writing to this field.
    pub cycles: Vec<Cycle>,
}

impl Edges {
    /// Construct a new instance of `Edges`, with a single cycle
    pub fn single_cycle(edges: impl IntoIterator<Item = Edge>) -> Self {
        let cycle = Cycle {
            edges: edges.into_iter().collect(),
        };

        Self {
            cycles: vec![cycle],
        }
    }

    /// Transform the edges
    #[must_use]
    pub fn transform(mut self, transform: &Transform) -> Self {
        for cycle in &mut self.cycles {
            for edge in &mut cycle.edges {
                *edge = edge.clone().transform(transform);
            }
        }

        self
    }
}

/// A cycle of connected edges
///
/// The end of each edge in the cycle must connect to the beginning of the next
/// edge. The end of the last edge must connect to the beginning of the first
/// one.
#[derive(Clone)]
pub struct Cycle {
    pub edges: Vec<Edge>,
}

/// An edge of a shape
#[derive(Clone, Debug)]
pub struct Edge {
    /// The curve that defines the edge's geometry
    ///
    /// The edge is a segment of the curve that is bounded by two vertices.
    pub curve: Curve,

    /// The vertices that bound this edge on the curve, in curve coordinates
    ///
    /// If there are no such vertices, that means the edge is connected to
    /// itself (like a full circle, for example).
    pub vertices: Option<[Vertex<1>; 2]>,

    /// Indicates whether the curve's direction is reversed
    ///
    /// Once this struct keeps track of the vertices that bound the edge, this
    /// field can probably be made redundant. The order of the bounding points
    /// will simply define the direction of the curve.
    pub reverse: bool,
}

impl Edge {
    /// Construct an edge
    ///
    /// If vertices are provided in `vertices`, they must be on `curve`.
    ///
    /// This constructor will convert the vertices into curve coordinates. If
    /// they are not on the curve, this will result in their projection being
    /// converted into curve coordinates, which is likely not the caller's
    /// intention.
    pub fn new(curve: Curve, vertices: Option<[Vertex<3>; 2]>) -> Self {
        let vertices = vertices
            .map(|vertices| vertices.map(|vertex| vertex.to_1d(&curve)));

        Self {
            curve,
            vertices,
            reverse: false,
        }
    }

    /// Construct an edge by sweeping a vertex
    ///
    /// Only sweeps along the positive direction of the z axis are supported.
    ///
    /// You **MUST NOT** use this method to construct an instance of `Edge` that
    /// represents and already existing edge. If you need an `Edge` instance
    /// that refers to an existing edge, copy an existing `Edge` instance.
    ///
    /// This method creates a second vertex by calling [`Vertex::create_at`]
    /// internally. You **MUST NOT** use this method to indirectly create a
    /// `Vertex` instance that refers to an already existing vertex. If you have
    /// two vertices and need an edge to connect them, use [`Edge::new`].
    ///
    /// Please refer to [`Vertex::create_at`] for an explanation of these
    /// limitations.
    #[allow(unused)]
    pub fn sweep_vertex(vertex: Vertex<3>, path: Vector<3>) -> Self {
        let line = Line {
            origin: *vertex.location(),
            direction: path,
        };

        let a = vertex;
        let b = Vertex::create_at(line.origin + line.direction);

        Self::new(Curve::Line(line), Some([a, b]))
    }

    /// Create a circle
    pub fn circle(radius: f64) -> Self {
        Self {
            curve: Curve::Circle(Circle {
                center: Point::origin(),
                radius: Vector::from([radius, 0.]),
            }),
            vertices: None,
            reverse: false,
        }
    }

    /// Reverse the edge
    pub fn reverse(&mut self) {
        self.reverse = !self.reverse;
    }

    /// Transform the edge
    #[must_use]
    pub fn transform(mut self, transform: &Transform) -> Self {
        self.curve = self.curve.transform(transform);
        self.vertices = self
            .vertices
            .map(|vertices| vertices.map(|vertex| vertex.transform(transform)));

        self
    }
}
