pub struct Coordinate3(pub [usize; 3]);

impl Coordinate3 {
    pub fn from_index(index: usize, cube_size: usize) -> Coordinate3 {
        let x = index % cube_size;
        let y = (index / cube_size) % cube_size;
        let z = (index / (cube_size * cube_size)) % cube_size;

        Coordinate3([x, y, z])
    }
}
