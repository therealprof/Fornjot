use parry3d_f64::bounding_volume::AABB;

use crate::{
    debug::DebugInfo,
    kernel::{
        topology::{edges::Edges, faces::Faces, vertices::Vertices},
        Shape,
    },
};

impl Shape for fj::Difference {
    fn bounding_volume(&self) -> AABB {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.a.bounding_volume()
    }

    fn faces(&self, _tolerance: f64, _: &mut DebugInfo) -> Faces {
        // TASK: Implement algorithm from "Boundary Representation Modelling
        //       Techniques", section 6.1.1 (pages 127 ff.).

        // TASK: Find interactions between objects by comparing each face in one
        //       with each face in the other.
        // TASK: Check for intersection between the surfaces of each face. This
        //       might result in a curve where they intersect.
        // TASK: Check that curve against the faces, to find curve sections that
        //       lie in the faces.
        // TASK: Find common curve sections that lie in both faces.
        // TASK: Add common curve sections to faces. (What does that mean
        //       specifically? Are we creating a new edge, and therefore new
        //       faces, there?)

        todo!()
    }

    fn edges(&self) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vertices {
        todo!()
    }
}
