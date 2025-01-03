use crate::cube::CubeNxN;
use crate::utils::Coordinate3;
use three_d::{
    ColorMaterial, Context, CpuMesh, Geometry, Gm, Mat4, Mesh, Object, RenderStates, Srgba, Vec3,
};

const CUBIE_WORLD_SIZE: f32 = 1.0;

/// An NxN twisty puzzle model which can be drawn on screen.
pub struct CubeObject {
    /// The in-memory representation of the cube to be drawn
    current_state: CubeNxN,

    // TODO: can be instanced maybe?
    cubie_models: Vec<Gm<Mesh, ColorMaterial>>,
}

impl CubeObject {
    pub fn new(context: &Context, size: usize) -> Self {
        let base = CubeNxN::default(size);
        let cubie_models = (0..size.pow(3))
            .map(|i| {
                let Coordinate3([x, y, z]) = Coordinate3::from_index(i, size);

                let cube_mesh = Mesh::new(&context, &CpuMesh::cube());

                let colour = vec![Srgba::WHITE, Srgba::RED, Srgba::GREEN, Srgba::BLUE][i % 4];

                // TODO the colours
                let mut model = Gm::new(
                    cube_mesh,
                    ColorMaterial {
                        color: colour,
                        is_transparent: false,
                        render_states: RenderStates::default(),
                        texture: None,
                    },
                );

                let offset = -(size as f32 / 2.0);
                let translation = Mat4::from_translation(Vec3::new(
                    (x as f32 + offset) * CUBIE_WORLD_SIZE,
                    (y as f32 + offset) * CUBIE_WORLD_SIZE,
                    (z as f32 + offset) * CUBIE_WORLD_SIZE,
                ));
                model.set_transformation(Mat4::from_scale(CUBIE_WORLD_SIZE) * translation);

                // model.set_animation(move |time| {
                //     Mat4::from_angle_x(Rad(time * 0.001)) * Mat4::from_angle_y(Rad(time * 0.002))
                // });

                model
            })
            .collect();

        CubeObject {
            current_state: base,
            cubie_models,
        }
    }

    pub fn animate(&mut self, time: f32) {
        self.cubie_models.iter_mut().for_each(|model| {
            model.animate(time);
        })
    }
}

impl<'a> IntoIterator for &'a CubeObject {
    type Item = &'a dyn Object;
    type IntoIter = <Vec<&'a dyn Object> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.cubie_models
            .iter()
            .map(|m| m as &'a dyn Object)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

// impl Geometry for CubeObject {
//     fn draw(&self, viewer: &dyn Viewer, program: &Program, render_states: RenderStates) {
//         self.cubie_models.iter().for_each(|model| {
//             model.draw(viewer, program, render_states);
//         })
//     }
//
//     fn vertex_shader_source(&self) -> String {
//         todo!()
//     }
//
//     fn id(&self) -> GeometryId {
//         todo!()
//     }
//
//     fn render_with_material(
//         &self,
//         material: &dyn Material,
//         viewer: &dyn Viewer,
//         lights: &[&dyn Light],
//     ) {
//         self.cubie_models
//             .iter()
//             .for_each(|model| model.render_with_material(material, viewer, lights))
//     }
//
//     fn render_with_effect(
//         &self,
//         material: &dyn Effect,
//         viewer: &dyn Viewer,
//         lights: &[&dyn Light],
//         color_texture: Option<ColorTexture>,
//         depth_texture: Option<DepthTexture>,
//     ) {
//         self.cubie_models.iter().for_each(|model| {
//             model.render_with_effect(material, viewer, lights, color_texture, depth_texture)
//         })
//     }
//
//     fn aabb(&self) -> AxisAlignedBoundingBox {
//         self.model.aabb()
//     }
//
//     fn animate(&mut self, time: f32) {
//         self.cubie_models
//             .iter_mut()
//             .for_each(|model| model.animate(time))
//     }
// }
//
// impl Object for CubeObject {
//     fn render(&self, viewer: &dyn Viewer, lights: &[&dyn Light]) {
//         self.cubie_models
//             .iter()
//             .for_each(|model| model.render(viewer, lights))
//     }
//
//     fn material_type(&self) -> MaterialType {
//         MaterialType::Opaque
//     }
// }
