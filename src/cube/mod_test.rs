use crate::cube::{Colour, CubeNxN, Cubie};

#[test]
pub fn default_cube() {
    let cube = CubeNxN::default(2);

    assert_eq!(
        cube.cubies,
        vec![
            Cubie {
                orientation: [
                    Colour::WHITE,
                    Colour::NONE,
                    Colour::GREEN,
                    Colour::RED,
                    Colour::NONE,
                    Colour::NONE
                ]
            },
            Cubie {
                orientation: [
                    Colour::WHITE,
                    Colour::ORANGE,
                    Colour::GREEN,
                    Colour::NONE,
                    Colour::NONE,
                    Colour::NONE
                ]
            },
            Cubie {
                orientation: [
                    Colour::NONE,
                    Colour::NONE,
                    Colour::GREEN,
                    Colour::RED,
                    Colour::NONE,
                    Colour::YELLOW
                ]
            },
            Cubie {
                orientation: [
                    Colour::NONE,
                    Colour::ORANGE,
                    Colour::GREEN,
                    Colour::NONE,
                    Colour::NONE,
                    Colour::YELLOW
                ]
            },
            Cubie {
                orientation: [
                    Colour::WHITE,
                    Colour::NONE,
                    Colour::NONE,
                    Colour::RED,
                    Colour::BLUE,
                    Colour::NONE
                ]
            },
            Cubie {
                orientation: [
                    Colour::WHITE,
                    Colour::ORANGE,
                    Colour::NONE,
                    Colour::NONE,
                    Colour::BLUE,
                    Colour::NONE
                ]
            },
            Cubie {
                orientation: [
                    Colour::NONE,
                    Colour::NONE,
                    Colour::NONE,
                    Colour::RED,
                    Colour::BLUE,
                    Colour::YELLOW
                ]
            },
            Cubie {
                orientation: [
                    Colour::NONE,
                    Colour::ORANGE,
                    Colour::NONE,
                    Colour::NONE,
                    Colour::BLUE,
                    Colour::YELLOW
                ]
            },
        ]
    )
}
