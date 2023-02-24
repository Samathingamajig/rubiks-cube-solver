use std::fmt::{Debug, Formatter};

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

impl Debug for RubiksCube {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fn output_single_row(fmt: &mut Formatter<'_>, row: &Vec<Color>) -> std::fmt::Result {
            for c in row {
                write!(fmt, "{c:?}")?
            }
            Ok(())
        }

        let leading_spaces = " ".repeat(self.faces[0].len());

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

#[derive(Clone, Copy)]
enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
enum Movement {
    Clockwise,
    CounterClockwise,
    Half,
}

#[derive(Clone, Copy)]
struct Side(Face, Corner);

impl Debug for Color {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Color::White => "W",
                Color::Yellow => "Y",
                Color::Red => "R",
                Color::Orange => "O",
                Color::Blue => "B",
                Color::Green => "G",
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

fn rotate_face(rc: &mut RubiksCube, face: Face, movement: Movement, depth: usize) {
    if depth == 0 {
        let main_face = &mut rc.faces[face as usize];
        let s = rc.size - 1;
        match movement {
            Movement::Clockwise => {
                for i in 0..s {
                    let temp = main_face[0][i];
                    main_face[0][i] = main_face[s - i][0];
                    main_face[s - i][0] = main_face[s][s - i];
                    main_face[s][s - i] = main_face[i][s];
                    main_face[i][s] = temp;
                }
            }
            Movement::CounterClockwise => todo!(),
            Movement::Half => todo!(),
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

                let temp = rc.faces[sides[0].0 as usize][positions[0].0][positions[0].1];
                rc.faces[sides[0].0 as usize][positions[0].0][positions[0].1] =
                    rc.faces[sides[3].0 as usize][positions[3].0][positions[3].1];
                rc.faces[sides[3].0 as usize][positions[3].0][positions[3].1] =
                    rc.faces[sides[2].0 as usize][positions[2].0][positions[2].1];
                rc.faces[sides[2].0 as usize][positions[2].0][positions[2].1] =
                    rc.faces[sides[1].0 as usize][positions[1].0][positions[1].1];
                rc.faces[sides[1].0 as usize][positions[1].0][positions[1].1] = temp;
            }
        }
        Movement::CounterClockwise => todo!(),
        Movement::Half => todo!(),
    }
}

fn main() {
    // let mut rc = RubiksCube::new(5);

    // // println!("{rc:?}");

    // rc.faces[Face::Front as usize][0][0] = Color::Yellow;
    // rc.faces[Face::Front as usize][0][1] = Color::Orange;
    // rc.faces[Face::Front as usize][0][2] = Color::Red;
    // rc.faces[Face::Front as usize][0][3] = Color::Green;
    // rc.faces[Face::Front as usize][0][4] = Color::White;

    // println!("{rc:?}");

    // for _ in 0..8 {
    //     rotate_face(&mut rc, Face::Front, Movement::Clockwise);
    //     std::io::stdin().read_line(&mut String::new()).unwrap();

    //     println!("{rc:?}");
    // }

    let mut rc = RubiksCube::new(3);

    println!("{rc:?}");

    rc.faces[Face::Front as usize][0][0] = Color::Yellow;
    rc.faces[Face::Front as usize][0][1] = Color::Blue;
    rc.faces[Face::Front as usize][2][1] = Color::Red;

    rc.faces[Face::Left as usize][0][2] = Color::White;
    rc.faces[Face::Left as usize][2][2] = Color::Green;

    println!("{rc:?}");

    for _ in 0..8 {
        rotate_face(&mut rc, Face::Front, Movement::Clockwise, 1);
        std::io::stdin().read_line(&mut String::new()).unwrap();

        println!("{rc:?}");
    }
}
