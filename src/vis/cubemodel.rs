use crate::cube::CubeNxN;
use crate::utils::Coordinate3;
use log::info;
use three_d::{
    ColorMaterial, Context, CpuMesh, Geometry, Gm, Mat4, Mesh, Object, RenderStates, Srgba, Vec3,
};

const CUBIE_WORLD_SIZE: f32 = 0.1;

/// An NxN twisty puzzle model which can be drawn on screen.
pub struct CubeObject {
    /// The in-memory representation of the cube to be drawn
    current_state: CubeNxN,

    // TODO: can be instanced maybe?
    cubie_models: Vec<Gm<Mesh, ColorMaterial>>,
}

impl CubeObject {
    pub fn new(context: &Context, cube_size: usize, world_size: f32) -> Self {
        let base = CubeNxN::default(cube_size);

        let cubie_models = (0..cube_size.pow(3))
            .map(|i| {
                let coord = Coordinate3::from_index(i, cube_size);

                let cube_mesh = Mesh::new(&context, &CpuMesh::cube());

                let colour = vec![Srgba::WHITE, Srgba::RED, Srgba::GREEN][i % 3];

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

                // TODO LOL this does not work with even numbers it's probably something easy and
                //   dumb I know but like not my main focus right now OK i'll figure it out in a sec
                let offset = -(cube_size as i32 / 2) as f32;
                info!("offset: {offset}");
                // TODO this is all a bad way of doing it and doesn't make sense refactor this later
                //  to actually use world size and not do the translation and scale calculations in
                //  different places
                let world_coord = coord.to_world_coordinates(offset, world_size);
                let translation = Mat4::from_translation(world_coord);
                model.set_transformation(translation * Mat4::from_scale(0.5));

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
