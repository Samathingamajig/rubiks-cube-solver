use std::fmt::{Debug, Formatter};

struct RubiksCube {
    faces: [Vec<Vec<Color>>; 6],
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

#[derive(Clone)]
enum Color {
    White,
    Yellow,
    Red,
    Orange,
    Blue,
    Green,
}

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

fn main() {
    let rc = RubiksCube {
        faces: [
            vec![vec![Color::White; 3]; 3],
            vec![vec![Color::Red; 3]; 3],
            vec![vec![Color::Blue; 3]; 3],
            vec![vec![Color::Orange; 3]; 3],
            vec![vec![Color::Green; 3]; 3],
            vec![vec![Color::Yellow; 3]; 3],
        ],
    };

    println!("{rc:?}");
}
