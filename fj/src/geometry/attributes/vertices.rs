use nalgebra::SVector;

use crate::geometry::{operations, shapes};

/// Implemented by shapes that can return the vertices that make them up
///
/// Since the vertices of a shape are going to have a position in space, `D`
/// defines the dimension of those vertices' positions.
pub trait Vertices<const D: usize> {
    /// Return the vertices of the shape
    fn vertices(&self) -> Vec<operations::Translate<shapes::Vertex, D>>;
}

impl<const D: usize> Vertices<D> for shapes::Vertex {
    fn vertices(&self) -> Vec<operations::Translate<shapes::Vertex, D>> {
        vec![operations::Translate {
            shape: *self,
            offset: SVector::zeros(),
        }]
    }
}

impl<T, const D: usize> Vertices<D> for operations::Translate<T, D>
where
    T: Vertices<D>,
{
    fn vertices(&self) -> Vec<operations::Translate<shapes::Vertex, D>> {
        let mut vertices = self.shape.vertices();

        for translate in &mut vertices {
            translate.offset += self.offset;
        }

        vertices
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::vector;

    use crate::{geometry::shapes::Vertex, prelude::*};

    use super::Vertices;

    #[test]
    fn test_vertices_for_vertex() {
        assert_eq!(
            <Vertex as Vertices<1>>::vertices(&Vertex),
            [Vertex.translate(vector![0.])],
        );
        assert_eq!(
            <Vertex as Vertices<2>>::vertices(&Vertex),
            [Vertex.translate(vector![0., 0.])],
        );
        assert_eq!(
            <Vertex as Vertices<3>>::vertices(&Vertex),
            [Vertex.translate(vector![0., 0., 0.])],
        );
    }

    #[test]
    fn test_vertices_for_translate_vertices() {
        let vertex_1d = Vertex.translate(vector![2.]);
        let vertex_2d = Vertex.translate(vector![2., 3.]);
        let vertex_3d = Vertex.translate(vector![2., 3., 4.]);

        assert_eq!(vertex_1d.vertices(), [vertex_1d]);
        assert_eq!(vertex_2d.vertices(), [vertex_2d]);
        assert_eq!(vertex_3d.vertices(), [vertex_3d]);
    }
}
