/// API for creating a mesh
pub struct MeshMaker<V> {
    vertices: Vec<V>,
    indices: Vec<Index>,
}

impl<V> MeshMaker<V>
where
    V: Copy + Eq,
{
    /// Create a new instance of `MeshMaker`
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Add a vertex to the mesh
    pub fn push(&mut self, vertex: V) {
        let pos = self.vertices.iter().position(|&v| v == vertex);
        let index = pos.unwrap_or_else(|| {
            let index = self.vertices.len();
            self.vertices.push(vertex);
            index
        });

        self.indices.push(index as u32);
    }

    /// Access the vertices of the mesh
    pub fn vertices(&self) -> impl Iterator<Item = V> + '_ {
        self.vertices.iter().copied()
    }

    /// Access the indices of the mesh
    pub fn indices(&self) -> impl Iterator<Item = Index> + '_ {
        self.indices.iter().copied()
    }
}

/// An index that refers to a vertex in a mesh
pub type Index = u32;
