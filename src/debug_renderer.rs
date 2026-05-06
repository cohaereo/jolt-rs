use jolt_sys::{
    JPC_CastShadow_JPC_CAST_SHADOW_OFF, JPC_CastShadow_JPC_CAST_SHADOW_ON,
    JPC_CullMode_JPC_CULLING_OFF, JPC_CullMode_JPC_CULL_BACK_FACE,
    JPC_CullMode_JPC_CULL_FRONT_FACE, JPC_DrawMode_JPC_DRAW_MODE_SOLID,
    JPC_DrawMode_JPC_DRAW_MODE_WIREFRAME,
};
use mint::{ColumnMatrix4, Vector3};

use crate::Real;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CullMode {
    BackFace = JPC_CullMode_JPC_CULL_BACK_FACE as u32,
    FrontFace = JPC_CullMode_JPC_CULL_FRONT_FACE as u32,
    Off = JPC_CullMode_JPC_CULLING_OFF as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CastShadow {
    On = JPC_CastShadow_JPC_CAST_SHADOW_ON as u32,
    Off = JPC_CastShadow_JPC_CAST_SHADOW_OFF as u32,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawMode {
    Solid = JPC_DrawMode_JPC_DRAW_MODE_SOLID as u32,
    Wireframe = JPC_DrawMode_JPC_DRAW_MODE_WIREFRAME as u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub trait DebugRenderer {
    fn draw_line(&mut self, from: Vector3<Real>, to: Vector3<Real>, color: Color);

    fn draw_triangle(
        &mut self,
        v1: Vector3<Real>,
        v2: Vector3<Real>,
        v3: Vector3<Real>,
        color: Color,
    );

    fn create_triangle_batch(&mut self, triangles: &[!]) -> !;
    fn create_triangle_batch_indexed(&mut self, vertices: &[!], indices: &[u32]) -> !;

    fn draw_geometry(
        &mut self,
        model_matrix: ColumnMatrix4<f32>,
        worldspace_bounds: (Vector3<Real>, Vector3<Real>),
        lod_scale_squared: f32,
        color: Color,
        geometry: &!,
        cull_mode: CullMode,
        cast_shadow: CastShadow,
        draw_mode: DrawMode,
    );

    fn draw_text_3d(&mut self, position: Vector3<Real>, string: &str, color: Color, height: f32);
}
