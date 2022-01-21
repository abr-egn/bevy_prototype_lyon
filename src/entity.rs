//! Custom Bevy ECS bundle for shapes.

use bevy::{
    ecs::{bundle::Bundle, component::Component},
    render::{
        color::Color,
        view::{ComputedVisibility, Visibility},
    },
    sprite::Mesh2dHandle,
    transform::components::{GlobalTransform, Transform},
};
use lyon_tessellation::{self as tess, FillOptions};

use crate::{
    draw::{DrawMode, FillMode},
    render::Shape,
};

/// A Bevy `Bundle` to represent a shape.
#[allow(missing_docs)]
#[derive(Bundle)]
pub struct ShapeBundle {
    pub path: Path,
    pub mode: DrawMode,
    pub shape: Shape,
    pub mesh2d: Mesh2dHandle,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for ShapeBundle {
    fn default() -> Self {
        Self {
            path: Path(tess::path::Path::new()),
            mode: DrawMode::Fill(FillMode {
                options: FillOptions::default(),
                color: Color::WHITE,
            }),
            shape: Shape::default(),
            mesh2d: Mesh2dHandle::default(),
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
        }
    }
}

#[allow(missing_docs)]
#[derive(Component)]
pub struct Path(pub tess::path::Path);

impl Path {
    /// Directly build a `Mesh` from this `Path`.
    pub fn build_mesh(
        &self,
        fill_tess: &mut tess::FillTessellator,
        stroke_tess: &mut tess::StrokeTessellator,
        tess_mode: &DrawMode,
    ) -> bevy::render::mesh::Mesh {
        crate::plugin::path_to_mesh(fill_tess, stroke_tess, tess_mode, self)
    }
}
