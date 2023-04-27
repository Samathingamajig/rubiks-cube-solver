use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, PartialEq)]
struct RubiksCube {
    size: usize,
    faces: [Vec<Vec<Color>>; 6],
}

impl RubiksCube {
    fn new(size: usize) -> RubiksCube {
        RubiksCube {
            size,
            faces: [
                vec![vec![Color::Yellow; size]; size],
                vec![vec![Color::Orange; size]; size],
                vec![vec![Color::Blue; size]; size],
                vec![vec![Color::Red; size]; size],
                vec![vec![Color::Green; size]; size],
                vec![vec![Color::White; size]; size],
            ],
        }
    }
}

impl Display for RubiksCube {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fn output_single_row(fmt: &mut Formatter<'_>, row: &Vec<Color>) -> std::fmt::Result {
            for c in row {
                write!(fmt, "{c}")?
            }
            Ok(())
        }

        let leading_spaces = " ".repeat(self.faces[0].len() * 2);

        for row in &self.faces[0][..] {
            write!(fmt, "{leading_spaces}")?;
            output_single_row(fmt, row)?;
            writeln!(fmt)?;
        }

        for (((left, front), right), back) in self.faces[1][..]
            .iter()
            .zip(&self.faces[2][..])
            .zip(&self.faces[3][..])
            .zip(&self.faces[4][..])
        {
            output_single_row(fmt, left)?;
            output_single_row(fmt, front)?;
            output_single_row(fmt, right)?;
            output_single_row(fmt, back)?;
            writeln!(fmt)?;
        }

        for row in &self.faces[5][..] {
            write!(fmt, "{leading_spaces}")?;
            output_single_row(fmt, row)?;
            writeln!(fmt)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

#[derive(Clone, Copy, num_derive::FromPrimitive, Debug)]
enum Face {
    Up,
    Left,
    Front,
    Right,
    Back,
    Down,
}

#[derive(Clone, Copy)]
enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Clone, Copy, Debug)]
enum Movement {
    Clockwise,
    CounterClockwise,
    Half,
}

#[derive(Clone, Copy)]
struct Side(Face, Corner);

impl Display for Color {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Color::White => yansi::Paint::black("[]")
                    .bg(yansi::Color::Fixed(255))
                    .to_string(),
                Color::Yellow => yansi::Paint::black("[]")
                    .bg(yansi::Color::RGB(255, 255, 0))
                    .to_string(),
                Color::Red => yansi::Paint::black("[]")
                    .bg(yansi::Color::RGB(255, 0, 0))
                    .to_string(),
                Color::Orange => yansi::Paint::black("[]")
                    .bg(yansi::Color::RGB(255, 100, 0))
                    .to_string(),
                Color::Blue => yansi::Paint::black("[]")
                    .bg(yansi::Color::RGB(0, 0, 255))
                    .to_string(),
                Color::Green => yansi::Paint::black("[]")
                    .bg(yansi::Color::RGB(0, 140, 0))
                    .to_string(),
            }
        )
    }
}

fn get_sides(face: Face) -> [Side; 4] {
    use Corner::*;
    use Face::*;
    match face {
        Up => [
            Side(Back, TopRight),
            Side(Right, TopRight),
            Side(Front, TopRight),
            Side(Left, TopRight),
        ],
        Left => [
            Side(Up, TopLeft),
            Side(Front, TopLeft),
            Side(Down, TopLeft),
            Side(Back, BottomRight),
        ],
        Front => [
            Side(Up, BottomLeft),
            Side(Right, TopLeft),
            Side(Down, TopRight),
            Side(Left, BottomRight),
        ],
        Right => [
            Side(Up, BottomRight),
            Side(Back, TopLeft),
            Side(Down, BottomRight),
            Side(Front, BottomRight),
        ],
        Back => [
            Side(Up, TopRight),
            Side(Left, TopLeft),
            Side(Down, BottomLeft),
            Side(Right, BottomRight),
        ],
        Down => [
            Side(Front, BottomLeft),
            Side(Right, BottomLeft),
            Side(Back, BottomLeft),
            Side(Left, BottomLeft),
        ],
    }
}

fn position_based_off_corner_and_move_count(
    corner: Corner,
    move_count: usize,
    size: usize,
    depth: usize,
) -> (usize, usize) {
    match corner {
        Corner::TopLeft => (move_count, depth),
        Corner::TopRight => (depth, size - 1 - move_count),
        Corner::BottomRight => (size - 1 - move_count, size - 1 - depth),
        Corner::BottomLeft => (size - 1 - depth, move_count),
    }
}

macro_rules! cycle {
    ($a:expr, $b:expr) => {{
        let temp = $a;
        $a = $b;
        $b = temp;
    }};
    ($a:expr, $b:expr, $c:expr, $d:expr) => {{
        let temp = $a;
        $a = $b;
        $b = $c;
        $c = $d;
        $d = temp;
    }};
}

fn rotate_face(rc: &mut RubiksCube, face: Face, movement: Movement, depth: usize) {
    if depth == 0 {
        let main_face = &mut rc.faces[face as usize];
        let s = rc.size - 1;
        match movement {
            Movement::Clockwise => {
                for o in 0..(rc.size / 2) {
                    for i in o..(s - o) {
                        cycle!(
                            main_face[o][i],
                            main_face[s - i][o],
                            main_face[s - o][s - i],
                            main_face[i][s - o]
                        );
                    }
                }
            }
            Movement::CounterClockwise => {
                for o in 0..(rc.size / 2) {
                    for i in o..(s - o) {
                        cycle!(
                            main_face[o][i],
                            main_face[i][s - o],
                            main_face[s - o][s - i],
                            main_face[s - i][o]
                        );
                    }
                }
            }
            Movement::Half => {
                for o in 0..(rc.size / 2) {
                    for i in o..(s - o) {
                        cycle!(main_face[o][i], main_face[s - o][s - i]);
                        cycle!(main_face[s - i][o], main_face[i][s - o]);
                    }
                }
            }
        };
    }

    let sides = get_sides(face);
    match movement {
        Movement::Clockwise => {
            for i in 0..rc.size {
                let positions = [
                    position_based_off_corner_and_move_count(sides[0].1, i, rc.size, depth),
                    position_based_off_corner_and_move_count(sides[1].1, i, rc.size, depth),
                    position_based_off_corner_and_move_count(sides[2].1, i, rc.size, depth),
                    position_based_off_corner_and_move_count(sides[3].1, i, rc.size, depth),
                ];

                cycle!(
                    rc.faces[sides[0].0 as usize][positions[0].0][positions[0].1],
                    rc.faces[sides[3].0 as usize][positions[3].0][positions[3].1],
                    rc.faces[sides[2].0 as usize][positions[2].0][positions[2].1],
                    rc.faces[sides[1].0 as usize][positions[1].0][positions[1].1]
                );
            }
        }
        Movement::CounterClockwise => {
            for i in 0..rc.size {
                let positions = [
                    position_based_off_corner_and_move_count(sides[0].1, i, rc.size, depth),
                    position_based_off_corner_and_move_count(sides[1].1, i, rc.size, depth),
                    position_based_off_corner_and_move_count(sides[2].1, i, rc.size, depth),
                    position_based_off_corner_and_move_count(sides[3].1, i, rc.size, depth),
                ];

                cycle!(
                    rc.faces[sides[0].0 as usize][positions[0].0][positions[0].1],
                    rc.faces[sides[1].0 as usize][positions[1].0][positions[1].1],
                    rc.faces[sides[2].0 as usize][positions[2].0][positions[2].1],
                    rc.faces[sides[3].0 as usize][positions[3].0][positions[3].1]
                );
            }
        }
        Movement::Half => {
            for i in 0..rc.size {
                let positions = [
                    position_based_off_corner_and_move_count(sides[0].1, i, rc.size, depth),
                    position_based_off_corner_and_move_count(sides[1].1, i, rc.size, depth),
                    position_based_off_corner_and_move_count(sides[2].1, i, rc.size, depth),
                    position_based_off_corner_and_move_count(sides[3].1, i, rc.size, depth),
                ];

                cycle!(
                    rc.faces[sides[0].0 as usize][positions[0].0][positions[0].1],
                    rc.faces[sides[2].0 as usize][positions[2].0][positions[2].1]
                );
                cycle!(
                    rc.faces[sides[1].0 as usize][positions[1].0][positions[1].1],
                    rc.faces[sides[3].0 as usize][positions[3].0][positions[3].1]
                );
            }
        }
    }
}

fn checkerboard(rc: &mut RubiksCube, print_each_step: bool) {
    // dbg!(rc.size / 2);
    for face in [Face::Right, Face::Up, Face::Front] {
        for depth in (1..((rc.size + 1) / 2)).step_by(2) {
            // dbg!(depth, rc.size - depth - 1);
            rotate_face(rc, face, Movement::Half, depth);
            if depth != rc.size - depth - 1 {
                rotate_face(rc, face, Movement::Half, rc.size - depth - 1);
            }
        }
        if print_each_step {
            println!("{}", rc);
            std::io::stdin().read_line(&mut String::new()).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn random_3x3x3_clockwise_scramble() {
        let mut rc = crate::RubiksCube::new(3);

        use crate::Color::*;
        use crate::Face::*;

        let moves = [
            Down, Left, Up, Front, Left, Up, Down, Right, Down, Left, Front, Back, Left, Right,
            Back, Up, Down, Front, Up, Down, Back, Left, Front, Left, Right, Left, Down, Left,
            Front, Back,
        ];

        for m in moves {
            crate::rotate_face(&mut rc, m, crate::Movement::Clockwise, 0);
        }

        let expected = crate::RubiksCube {
            size: 3,
            faces: [
                vec![
                    vec![Green, White, Red],
                    vec![Yellow, Yellow, Red],
                    vec![Blue, Red, Red],
                ],
                vec![
                    vec![White, Blue, Orange],
                    vec![Blue, Orange, Orange],
                    vec![Green, Green, Orange],
                ],
                vec![
                    vec![White, White, Yellow],
                    vec![Yellow, Blue, Orange],
                    vec![White, White, Orange],
                ],
                vec![
                    vec![Blue, Yellow, Blue],
                    vec![Blue, Red, Red],
                    vec![Green, Yellow, Yellow],
                ],
                vec![
                    vec![White, Green, Red],
                    vec![Blue, Green, White],
                    vec![Blue, Orange, Yellow],
                ],
                vec![
                    vec![Green, Orange, Yellow],
                    vec![Red, White, Green],
                    vec![Red, Green, Orange],
                ],
            ],
        };

        assert_eq!(rc, expected);
    }

    #[test]
    fn random_3x3x3_mixed_scramble() {
        let mut rc = crate::RubiksCube::new(3);

        use crate::Color::*;
        use crate::Face::*;
        use crate::Movement::*;

        let moves = [
            (Up, Half),
            (Right, Half),
            (Down, Half),
            (Front, CounterClockwise),
            (Down, Clockwise),
            (Up, Half),
            (Back, CounterClockwise),
            (Left, Clockwise),
            (Front, Clockwise),
            (Up, Half),
            (Front, Half),
            (Down, Clockwise),
            (Right, CounterClockwise),
            (Down, Clockwise),
            (Left, CounterClockwise),
            (Back, Half),
            (Front, Half),
            (Back, CounterClockwise),
            (Right, Clockwise),
            (Down, CounterClockwise),
            (Left, CounterClockwise),
            (Front, Clockwise),
            (Left, Clockwise),
            (Down, Half),
            (Up, CounterClockwise),
            (Right, Half),
            (Back, Clockwise),
            (Front, CounterClockwise),
            (Left, CounterClockwise),
            (Up, Half),
        ];

        for (face, movement) in moves {
            crate::rotate_face(&mut rc, face, movement, 0);
        }

        let expected = crate::RubiksCube {
            size: 3,
            faces: [
                vec![
                    vec![White, Red, Orange],
                    vec![Orange, Yellow, Yellow],
                    vec![Yellow, Red, Red],
                ],
                vec![
                    vec![Orange, Yellow, Orange],
                    vec![Orange, Orange, Yellow],
                    vec![Yellow, Orange, Orange],
                ],
                vec![
                    vec![Blue, Blue, Blue],
                    vec![Red, Blue, Blue],
                    vec![Green, Orange, Blue],
                ],
                vec![
                    vec![White, Blue, Green],
                    vec![White, Red, Yellow],
                    vec![Yellow, White, Red],
                ],
                vec![
                    vec![White, Green, Blue],
                    vec![Green, Green, Green],
                    vec![Green, White, Red],
                ],
                vec![
                    vec![Yellow, White, Red],
                    vec![Blue, White, Green],
                    vec![Green, Red, White],
                ],
            ],
        };

        assert_eq!(rc, expected);
    }

    #[test]
    fn random_5x5x5_mixed_scramble() {
        let mut rc = crate::RubiksCube::new(5);

        use crate::Color::*;
        use crate::Face::*;
        use crate::Movement::*;

        let moves = [
            (Front, Half, 1),
            (Down, Half, 1),
            (Right, Clockwise, 1),
            (Up, CounterClockwise, 1),
            (Left, CounterClockwise, 0),
            (Down, Clockwise, 0),
            (Front, Clockwise, 0),
            (Right, CounterClockwise, 1),
            (Down, Clockwise, 0),
            (Back, CounterClockwise, 2),
            (Right, Clockwise, 1),
            (Up, Half, 2),
            (Front, CounterClockwise, 0),
            (Down, Half, 0),
            (Up, CounterClockwise, 1),
            (Down, Clockwise, 2),
            (Left, Clockwise, 0),
            (Up, Half, 0),
            (Front, Half, 2),
            (Right, Clockwise, 1),
            (Front, Clockwise, 1),
            (Left, CounterClockwise, 0),
            (Up, CounterClockwise, 1),
            (Front, Half, 2),
            (Left, CounterClockwise, 1),
            (Back, Clockwise, 1),
            (Up, Clockwise, 0),
            (Left, Half, 1),
            (Right, Half, 1),
            (Back, CounterClockwise, 0),
        ];

        for (face, movement, layer) in moves {
            crate::rotate_face(&mut rc, face, movement, layer);
        }

        let expected = crate::RubiksCube {
            size: 5,
            faces: [
                vec![
                    vec![White, Orange, Red, Blue, White],
                    vec![Orange, Blue, Orange, White, Yellow],
                    vec![Yellow, Red, Orange, Orange, Orange],
                    vec![White, Yellow, Orange, White, Green],
                    vec![Green, White, Red, Blue, Yellow],
                ],
                vec![
                    vec![Green, Yellow, Green, Green, Red],
                    vec![Red, Red, Green, Yellow, White],
                    vec![Red, White, Blue, Green, Orange],
                    vec![Red, Blue, Green, Orange, Orange],
                    vec![Green, Orange, Blue, Red, Blue],
                ],
                vec![
                    vec![White, Blue, Yellow, Yellow, Orange],
                    vec![Red, Green, Orange, White, Orange],
                    vec![Green, Green, Yellow, Yellow, Orange],
                    vec![White, Red, Red, Blue, Yellow],
                    vec![Yellow, Red, Blue, Blue, Blue],
                ],
                vec![
                    vec![Blue, Yellow, Yellow, Blue, Blue],
                    vec![Blue, Green, White, Blue, Red],
                    vec![White, Yellow, Green, White, Orange],
                    vec![Green, White, Yellow, Orange, Green],
                    vec![Red, Yellow, White, Orange, Green],
                ],
                vec![
                    vec![Orange, White, Blue, White, Orange],
                    vec![Blue, Yellow, Red, Orange, Yellow],
                    vec![Blue, Blue, White, Blue, Green],
                    vec![Red, Green, Blue, Yellow, White],
                    vec![Red, White, White, Blue, Orange],
                ],
                vec![
                    vec![Red, Green, White, Red, White],
                    vec![Yellow, Green, Blue, Red, Orange],
                    vec![Yellow, Red, Red, White, Red],
                    vec![Green, Orange, Yellow, Red, Green],
                    vec![Yellow, Orange, Green, Green, Yellow],
                ],
            ],
        };

        assert_eq!(rc, expected);
    }
}

fn main() {
    // let mut rc = RubiksCube::new(5);

    // rc.faces[Face::Front as usize][0][0] = Color::Yellow;
    // rc.faces[Face::Front as usize][0][1] = Color::Orange;
    // rc.faces[Face::Front as usize][0][2] = Color::Red;
    // rc.faces[Face::Front as usize][0][3] = Color::Green;
    // rc.faces[Face::Front as usize][0][4] = Color::White;

    // rc.faces[Face::Front as usize][3][1] = Color::Yellow;
    // rc.faces[Face::Front as usize][3][2] = Color::Orange;

    // println!("{rc}");

    // loop {
    //     rotate_face(&mut rc, Face::Front, Movement::Half, 0);
    //     std::io::stdin().read_line(&mut String::new()).unwrap();

    //     println!("{rc}");
    // }

    // let mut rc = RubiksCube::new(3);

    // println!("{rc:?}");

    // rc.faces[Face::Front as usize][0][0] = Color::Yellow;
    // rc.faces[Face::Front as usize][0][1] = Color::Blue;
    // rc.faces[Face::Front as usize][2][1] = Color::Red;

    // rc.faces[Face::Left as usize][0][2] = Color::White;
    // rc.faces[Face::Left as usize][2][2] = Color::Green;

    // println!("{rc:?}");

    // for _ in 0..8 {
    //     rotate_face(&mut rc, Face::Front, Movement::Clockwise, 1);
    //     std::io::stdin().read_line(&mut String::new()).unwrap();

    //     println!("{rc:?}");
    // }

    // let mut rc = RubiksCube::new(3);

    // println!("{rc}");

    // let mut rng = rand::thread_rng();
    // for _ in 0..20 {
    //     let face = num_traits::FromPrimitive::from_u32(rng.gen_range(0..6)).unwrap();
    //     rotate_face(&mut rc, face, Movement::Clockwise, 0);
    //     std::io::stdin().read_line(&mut String::new()).unwrap();

    //     println!("Moving face {face:?}");
    //     println!("{rc}");
    // }

    // let mut rc = RubiksCube::new(5);

    // // checkerboard pattern through rotations, printing on each rotation

    // println!("{rc}");
    // std::io::stdin().read_line(&mut String::new()).unwrap();
    // rotate_face(&mut rc, Face::Left, Movement::Clockwise, 1);
    // rotate_face(&mut rc, Face::Left, Movement::Clockwise, 1);
    // rotate_face(&mut rc, Face::Right, Movement::Clockwise, 1);
    // rotate_face(&mut rc, Face::Right, Movement::Clockwise, 1);
    // println!("{rc}");
    // std::io::stdin().read_line(&mut String::new()).unwrap();
    // rotate_face(&mut rc, Face::Front, Movement::Clockwise, 1);
    // rotate_face(&mut rc, Face::Front, Movement::Clockwise, 1);
    // rotate_face(&mut rc, Face::Back, Movement::Clockwise, 1);
    // rotate_face(&mut rc, Face::Back, Movement::Clockwise, 1);
    // println!("{rc}");
    // std::io::stdin().read_line(&mut String::new()).unwrap();
    // rotate_face(&mut rc, Face::Up, Movement::Clockwise, 1);
    // rotate_face(&mut rc, Face::Up, Movement::Clockwise, 1);
    // rotate_face(&mut rc, Face::Down, Movement::Clockwise, 1);
    // rotate_face(&mut rc, Face::Down, Movement::Clockwise, 1);
    // println!("{rc}");
    // std::io::stdin().read_line(&mut String::new()).unwrap();

    // let mut rc = RubiksCube::new(3);

    // println!("{rc}");
    // std::io::stdin().read_line(&mut String::new()).unwrap();

    // for _ in 0..4 {
    //     rotate_face(&mut rc, Face::Right, Movement::Clockwise, 1);
    //     rotate_face(&mut rc, Face::Up, Movement::Clockwise, 0)
    // }
    // print!("{rc}");
    // std::io::stdin().read_line(&mut String::new()).unwrap();

    // for _ in 0..4 {
    //     rotate_face(&mut rc, Face::Down, Movement::Clockwise, 1);
    //     rotate_face(&mut rc, Face::Front, Movement::Clockwise, 0)
    // }
    // print!("{rc}");
    // std::io::stdin().read_line(&mut String::new()).unwrap();

    // for _ in 0..4 {
    //     rotate_face(&mut rc, Face::Back, Movement::Clockwise, 1);
    //     rotate_face(&mut rc, Face::Left, Movement::Clockwise, 0)
    // }

    // print!("{rc}");

    let mut rc = RubiksCube::new(5);
    checkerboard(&mut rc, false);
    println!("{rc}");

    let mut rc = RubiksCube::new(3);
    checkerboard(&mut rc, false);
    println!("{rc}");

    let mut rc = RubiksCube::new(6);
    checkerboard(&mut rc, false);
    println!("{rc}");

    let mut rc = RubiksCube::new(7);
    checkerboard(&mut rc, false);
    println!("{rc}");
}
