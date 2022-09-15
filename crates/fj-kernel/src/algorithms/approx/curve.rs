//! Curve approximation
//!
//! Since curves are infinite (even circles have an infinite coordinate space,
//! even though they connect to themselves in global coordinates), a range must
//! be provided to approximate them. The approximation then returns points
//! within that range.
//!
//! The boundaries of the range are not included in the approximation. This is
//! done, to give the caller (who knows the boundary anyway) more options on how
//! to further process the approximation.

use crate::objects::{Curve, GlobalCurve};

use super::{
    path::{GlobalPathApprox, RangeOnPath},
    Approx, ApproxCache, ApproxPoint, Tolerance,
};

impl Approx for (&Curve, RangeOnPath) {
    type Approximation = CurveApprox;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut ApproxCache,
    ) -> Self::Approximation {
        let (curve, range) = self;

        let approx =
            (curve.global_form(), range).approx_with_cache(tolerance, cache);
        let points = approx.points.into_iter().map(|point| {
            let point_surface =
                curve.path().point_from_path_coords(point.local_form);
            ApproxPoint::new(point_surface, point.global_form)
                .with_source((*curve, point.local_form))
        });

        CurveApprox::empty().with_points(points)
    }
}

impl Approx for (&GlobalCurve, RangeOnPath) {
    type Approximation = GlobalPathApprox;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut ApproxCache,
    ) -> Self::Approximation {
        let (curve, range) = self;

        if let Some(approx) = cache.global_curve(curve) {
            return approx;
        }

        let points = (curve.path(), range)
            .approx_with_cache(tolerance, cache)
            .into_iter()
            .map(|(point_curve, point_global)| {
                ApproxPoint::new(point_curve, point_global)
            })
            .collect();
        cache.insert_global_curve(curve, GlobalPathApprox { points })
    }
}

/// An approximation of a [`Curve`]
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CurveApprox {
    /// The points that approximate the curve
    pub points: Vec<ApproxPoint<2>>,
}

impl CurveApprox {
    /// Create an empty instance of `CurveApprox`
    pub fn empty() -> Self {
        Self { points: Vec::new() }
    }

    /// Add points to the approximation
    pub fn with_points(
        mut self,
        points: impl IntoIterator<Item = ApproxPoint<2>>,
    ) -> Self {
        self.points.extend(points);
        self
    }
}
