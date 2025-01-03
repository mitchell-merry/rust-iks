use three_d::{Mat4, Vec3};

#[cfg(test)]
#[test]
pub fn t() {
    let offset = -(3 as f32 / 2.0);
    let translation = Mat4::from_translation(Vec3::new(
        (1 as f32 + offset),
        (1 as f32 + offset),
        (1 as f32 + offset),
    ));
    let scale = Mat4::from_scale(2.0);
    println!("{:?}", translation);
    println!("{:?}", scale);
}
