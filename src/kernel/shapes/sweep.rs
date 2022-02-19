use std::f64::consts::PI;

use nalgebra::vector;
use parry3d_f64::math::Isometry;

use crate::{
    debug::DebugInfo,
    kernel::{
        approximation::Approximation,
        topology::{
            edges::{Edge, Edges},
            faces::{Face, Faces},
            vertices::Vertices,
        },
        Shape,
    },
    math::{Aabb, Scalar, Transform, Vector},
};

impl Shape for fj::Sweep {
    fn bounding_volume(&self) -> Aabb<3> {
        let mut aabb = self.shape.bounding_volume();
        *aabb.max.z_mut() = self.length.into();
        aabb
    }

    fn faces(&self, tolerance: Scalar, debug_info: &mut DebugInfo) -> Faces {
        let original_faces = self.shape.faces(tolerance, debug_info);

        let bottom_faces = original_faces
            .clone()
            .transform(&Isometry::rotation(vector![PI, 0., 0.]).into());

        let top_faces = original_faces
            .transform(&Isometry::translation(0.0, 0.0, self.length).into());

        // Create edges of side walls.
        let mut side_edges = Vec::new();
        for vertex in self.shape.vertices().0 {
            let edge =
                Edge::sweep_vertex(vertex, Vector::from([0., 0., self.length]));
            side_edges.push(edge);
        }

        // TASK: Iterate through `original_faces.edges()`, sweep each one into
        //       a face. The previously created edges must be provided to the
        //       edge-to-face-sweep operation.

        // This will only work correctly, if the original shape consists of one
        // edge. If there are more, this will create some kind of weird face
        // chimera, a single face to represent all the side faces.
        //
        // It'll be even worse, if the original shape consists of multiple
        // faces.
        let approx = Approximation::for_edges(&self.shape.edges(), tolerance);

        let mut quads = Vec::new();
        for segment in approx.segments {
            let [v0, v1] = [segment.a, segment.b];
            let [v3, v2] = {
                let segment = Transform::translation(0., 0., self.length)
                    .transform_segment(&segment);
                [segment.a, segment.b]
            };

            quads.push([v0, v1, v2, v3]);
        }

        let mut side_face = Vec::new();
        for [v0, v1, v2, v3] in quads {
            side_face.push([v0, v1, v2].into());
            side_face.push([v0, v2, v3].into());
        }

        let mut faces = Vec::new();
        faces.extend(bottom_faces.0);
        faces.extend(top_faces.0);
        faces.push(Face::Triangles(side_face));

        Faces(faces)
    }

    fn edges(&self) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vertices {
        todo!()
    }
}
