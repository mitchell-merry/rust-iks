use three_d::Vec3;

pub struct Coordinate3(pub [usize; 3]);

pub type World3 = Vec3;

impl Coordinate3 {
    pub fn from_index(index: usize, cube_size: usize) -> Coordinate3 {
        let x = index % cube_size;
        let y = (index / cube_size) % cube_size;
        let z = (index / (cube_size * cube_size)) % cube_size;

        Coordinate3([x, y, z])
    }

    pub fn to_world_coordinates(&self, offset: f32, world_size: f32) -> World3 {
        let to_individual_world_coord = |coord: f32| (coord) + offset;

        World3::new(
            to_individual_world_coord(self.0[0] as f32),
            to_individual_world_coord(self.0[1] as f32),
            to_individual_world_coord(self.0[2] as f32),
        )
    }
}
