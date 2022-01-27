use std::f64::consts::PI;

use nalgebra::vector;
use parry3d_f64::{bounding_volume::AABB, math::Isometry};

use crate::{
    debug::DebugInfo,
    kernel::{
        approximation::Approximation,
        geometry::Surface,
        topology::{
            edges::{self, Edge, Edges},
            faces::{Face, Faces},
            vertices::Vertices,
        },
        Shape,
    },
};

impl Shape for fj::Sweep {
    fn bounding_volume(&self) -> AABB {
        let mut aabb = self.shape.bounding_volume();
        aabb.maxs.z = self.length;
        aabb
    }

    fn faces(&self, tolerance: f64, debug_info: &mut DebugInfo) -> Faces {
        let original_faces = self.shape.faces(tolerance, debug_info);

        let bottom_faces = original_faces
            .clone()
            .transform(&Isometry::rotation(vector![PI, 0., 0.]));

        let top_faces = original_faces.transform(&Isometry::translation(
            0.0,
            0.0,
            self.length,
        ));

        let mut side_faces = Vec::new();
        for cycle in self.shape.edges().cycles {
            for edge in cycle.edges {
                let top_edge = edge.clone().transform(&Isometry::translation(
                    0.0,
                    0.0,
                    self.length,
                ));

                let path = vector![self.length];

                // TASK: If we can sweep lines into planes, then `Plane` becomes
                //       redundant.
                // TASK: I think we need `Face::sweep_edge` here and call that.
                let surface = Surface::sweep_from(edge.curve.clone(), path);

                // TASK: This is problematic. The `Edge::sweep_vertex` calls
                //       create new vertices (using `Vertex::create_at`)
                //       internally. This is not good, as these edges share
                //       vertices, which means we're creating the same vertices
                //       multiple times. This is _forbidden_.
                let edges = match edge.vertices {
                    Some([a, b]) => {
                        let a = Edge::sweep_vertex(a.to_canonical(), path);
                        let b = Edge::sweep_vertex(b.to_canonical(), path);

                        Edges::single_cycle([edge, a, top_edge, b])
                    }
                    None => Edges {
                        // TASK: What we have here is a continuous edge that is
                        //       connected to itself. Hence the side wall that
                        //       it forms is itself continuous and connected to
                        //       itself.
                        //
                        //       Such a construct is not supported by by the
                        //       triangulation algorithm. I believe the usual
                        //       way to address this is to disallow these kinds
                        //       of constructs and always require at least one
                        //       vertex/edge that marks the start/end.
                        //
                        //       Maybe this is hubris, but I think it would be
                        //       better to not require such a fake vertex/edge,
                        //       as that would be confusing and might have other
                        //       unforeseen consequences down the line.
                        //
                        //       Instead, the approximation logic can handle
                        //       this. Continuous edges are already tracked (via
                        //       their `vertices` field), and are trivial to
                        //       handle anyway. Faces would need equivalent
                        //       tracking as well.
                        //
                        //       Face approximation logic could then easily
                        //       generate the fake edge required to make the
                        //       triangulation work. The existing infrastructure
                        //       to convert surface points back to 3D points
                        //       would neatly make sure that anything that's
                        //       supposed to be connected in 3D ends up being
                        //       connected.
                        cycles: vec![
                            edges::Cycle { edges: vec![edge] },
                            edges::Cycle {
                                edges: vec![top_edge],
                            },
                        ],
                    },
                };

                side_faces.push(Face::Face { edges, surface });
            }
        }

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
                let segment = segment.transformed(&Isometry::translation(
                    0.0,
                    0.0,
                    self.length,
                ));
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
