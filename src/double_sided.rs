use bevy::prelude::Mesh;

pub trait DoubleSidedExt {
    fn with_double_sided(self) -> Self;
}

impl DoubleSidedExt for Mesh {
    fn with_double_sided(mut self) -> Self {
        self.merge(&self.clone().with_inverted_winding().unwrap())
            .unwrap();
        self
    }
}
