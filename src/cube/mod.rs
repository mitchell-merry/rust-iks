mod mod_test;

#[derive(Debug, PartialEq)]
enum Colour {
    /// Each cubie contains data for all 6 of it's faces, but only 1/2/3 will
    ///   be shown at a time. The 'NONE' colour is for the faces that are always
    ///   facing into the cube (not shown on the outside). They're only visible
    ///   during a turn.
    NONE = 0,
    WHITE = 1,
    ORANGE = 2,
    GREEN = 3,
    RED = 4,
    BLUE = 5,
    YELLOW = 6,
}

impl Colour {
    pub fn otherwise_none(condition: bool, colour: Colour) -> Colour {
        if condition {
            colour
        } else {
            Colour::NONE
        }
    }
}

/// A "cubie" is each of the individual pieces on a cube.
/// It's worth making the distinction between each of the stickers on a cube,
///   and the actual physical pieces that make up the cube.
/// There are 26 physical cubies on a 3x3 cube (3x3x3 = 27, minus one for the middle),
///   and there are three types of cubies:
/// - those with 3 stickers (3 visible faces) - those are the corners
/// - those with 2 stickers (2 visible faces) - those are the edges
/// - those with 1 sticker (1 visible face) - those are the centres
///
/// On each rotation of a face on the puzzle, all the involved cubies will also individually
///   rotate (the sticker facing up is no longer the same cubie), so depending on the axis
///   of rotation, the direction of the rotation, and the number of times rotated, we cycle
///   the stickers on the cubie to reflect the new orientation.
/// The full rotation effect is a combination of changing the permutation of the cubies on
///   the face (actually moving them on the cube) and changing the cubies' individual
///   orientation
#[derive(Debug, PartialEq)]
pub struct Cubie {
    /// How the cubie is oriented.
    /// The order is [up, left, front, right, back, down].
    orientation: [Colour; 6],
}

pub struct CubeNxN {
    size: usize,
    /// Ordered list of all the cubies in the cube.
    ///
    /// Order of the cubies is in order of x/y/z, where, when
    ///   holding white up and green facing you
    /// - x is the axis "right to left" (red to orange)
    /// - y is the axis "up to down" (white to yellow)
    /// - z is the axis "front to back" (green to blue)
    /// Thus, 0/0/0 is the white/green/red cubie.
    ///
    /// Consult the following helpful diagram of a net of a 3x3 with the indices marked
    ///   and extrapolate to your favourite n.
    /// Notes:
    /// - the same index appears multiple times because a singular cubie can represent up to 3
    ///   visible stickers on a cube (see the definition of Cubie)
    /// - the faces with the following centers are normally coloured as such:
    ///   - 10: white
    ///   - 04: green
    ///   - 14: orange
    ///   - 12: red
    ///   - 22: blue
    ///   - 16: yellow
    /// ```txt
    ///           20 19 18
    ///           11 10 09
    ///           02 01 00
    /// 20 11 02  02 01 00  00 09 18  18 19 20
    /// 23 14 05  05 04 03  03 12 21  21 22 23
    /// 26 17 08  08 07 06  06 15 24  24 25 26
    ///           08 07 06
    ///           17 16 15
    ///           26 25 24
    /// ```
    cubies: Vec<Cubie>,
}

impl CubeNxN {
    pub fn default(size: usize) -> Self {
        let cubie_count = size * size * size;
        // TODO remove muts
        let mut cubies = Vec::with_capacity(cubie_count);

        for i in 0..cubie_count {
            let x = i % size;
            let y = (i / size) % size;
            let z = (i / (size * size)) % size;

            let up = Colour::otherwise_none(y == 0, Colour::WHITE);
            let left = Colour::otherwise_none(x == size - 1, Colour::ORANGE);
            let front = Colour::otherwise_none(z == 0, Colour::GREEN);
            let right = Colour::otherwise_none(x == 0, Colour::RED);
            let back = Colour::otherwise_none(z == size - 1, Colour::BLUE);
            let down = Colour::otherwise_none(y == size - 1, Colour::YELLOW);

            cubies.insert(
                i,
                Cubie {
                    orientation: [up, left, front, right, back, down],
                },
            )
        }

        Self { size, cubies }
    }
}
