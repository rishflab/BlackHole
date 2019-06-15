pub struct Ray {
    pub origin: [f32; 4],
    pub direction: [f32; 4],
}

pub struct Intersection {
    pub color: [f32; 4],
}

pub struct Index(pub u32);

pub struct Vertex (pub [f32; 4]);

pub struct Aabb {
    pub min: [f32; 4],
    pub max: [f32; 4],
}

pub struct MeshHandle {
    pub min: [u32; 4],
    pub max: [u32; 4],
}
