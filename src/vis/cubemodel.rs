use crate::cube::CubeNxN;
use std::ops::Deref;
use three_d::{
    AxisAlignedBoundingBox, ColorMaterial, ColorTexture, Context, CpuMesh, DepthTexture, Effect,
    Geometry, GeometryId, Gm, Light, Mat4, Material, MaterialType, Mesh, Object, Program,
    RenderStates, Viewer,
};

/// An NxN twisty puzzle model which can be drawn on screen.
pub struct CubeObject {
    /// The in-memory representation of the cube to be drawn
    current_state: CubeNxN,

    // TODO: can be instanced maybe?
    model: Gm<Mesh, ColorMaterial>,
}

impl CubeObject {
    pub fn new(context: &Context, size: usize) -> Self {
        let cube_mesh = Mesh::new(&context, &CpuMesh::cube());

        let mut model = Gm::new(cube_mesh, ColorMaterial::default());
        model.set_transformation(Mat4::from_scale(0.1));

        CubeObject {
            current_state: CubeNxN::default(size),
            model,
        }
    }
}

impl Deref for CubeObject {
    type Target = Gm<Mesh, ColorMaterial>;
    fn deref(&self) -> &Self::Target {
        &self.model
    }
}

impl<'a> IntoIterator for &'a CubeObject {
    type Item = &'a dyn Object;
    type IntoIter = std::iter::Once<&'a dyn Object>;

    fn into_iter(self) -> Self::IntoIter {
        std::iter::once(self)
    }
}

impl Geometry for CubeObject {
    fn draw(&self, viewer: &dyn Viewer, program: &Program, render_states: RenderStates) {
        self.model.draw(viewer, program, render_states);
    }

    fn vertex_shader_source(&self) -> String {
        self.model.vertex_shader_source()
    }

    fn id(&self) -> GeometryId {
        self.model.id()
    }

    fn render_with_material(
        &self,
        material: &dyn Material,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
    ) {
        self.model.render_with_material(material, viewer, lights)
    }

    fn render_with_effect(
        &self,
        material: &dyn Effect,
        viewer: &dyn Viewer,
        lights: &[&dyn Light],
        color_texture: Option<ColorTexture>,
        depth_texture: Option<DepthTexture>,
    ) {
        self.model
            .render_with_effect(material, viewer, lights, color_texture, depth_texture)
    }

    fn aabb(&self) -> AxisAlignedBoundingBox {
        self.model.aabb()
    }

    fn animate(&mut self, time: f32) {
        self.model.animate(time)
    }
}

impl Object for CubeObject {
    fn render(&self, viewer: &dyn Viewer, lights: &[&dyn Light]) {
        self.model.render(viewer, lights)
    }

    fn material_type(&self) -> MaterialType {
        MaterialType::Opaque
    }
}
