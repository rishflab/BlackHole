use glm;

pub struct Ray {
    pub origin: [f32; 4],
    pub direction: [f32; 4],
}

pub struct Intersection {
    pub color: [f32; 4],
}