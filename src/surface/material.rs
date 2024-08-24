use crate::primitives::Color;

#[derive(Debug, Clone, Copy)]
pub enum MaterialKind {
    Glossy,
    Diffuse,
}

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub material_kind: MaterialKind,
    pub color: Color,
}