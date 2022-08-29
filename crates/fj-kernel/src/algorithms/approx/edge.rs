use fj_math::{Point, Scalar};

use crate::objects::Edge;

use super::{curve::RangeOnCurve, Approx};

impl Approx for Edge {
    type Approximation = Vec<(Point<2>, Point<3>)>;
    type Params = ();

    fn approx(
        &self,
        tolerance: super::Tolerance,
        (): Self::Params,
    ) -> Self::Approximation {
        // The range is only used for circles right now.
        let range = RangeOnCurve {
            boundary: [[Scalar::ZERO].into(), [Scalar::TAU].into()],
        };

        let mut points = self.curve().approx(tolerance, range);

        // Insert the exact vertices of this edge into the approximation. This
        // means we don't rely on the curve approximation to deliver accurate
        // representations of these vertices, which they might not be able to
        // do.
        //
        // If we used inaccurate representations of those vertices here, then
        // that would lead to bugs in the approximation, as points that should
        // refer to the same vertex would be understood to refer to very close,
        // but distinct vertices.
        let vertices = self
            .vertices()
            .convert(|vertex| (vertex.position(), vertex.global().position()));
        if let Some([(point_curve, point_global), _]) = vertices {
            let point_surface =
                self.curve().kind().point_from_curve_coords(point_curve);
            points.insert(0, (point_surface, point_global));
        }

        points
    }
}
