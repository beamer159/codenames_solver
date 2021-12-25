pub mod solver {
    use std::fmt::{Display, Formatter};

    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum Color {
        Red,
        Blue,
    }

    impl Color {
        pub fn from_char(c: char) -> Option<Color> {
            match c.to_ascii_lowercase() {
                'r' => Some(Color::Red),
                'b' => Some(Color::Blue),
                _ => None,
            }
        }
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Color::Red => "Red".fmt(f),
                Color::Blue => "Blue".fmt(f),
            }
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Hash)]
    pub enum Space {
        Color(Color),
        Neutral,
        Assassin,
    }

    impl Space {
        pub fn from_char(c: char) -> Option<Space> {
            match c {
                'r' | 'b' => Color::from_char(c).map(Space::Color),
                'n' => Some(Space::Neutral),
                'a' => Some(Space::Assassin),
                _ => None,
            }
        }
    }

    impl Display for Space {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Space::Color(c) => c.fmt(f),
                Space::Neutral => "Neutral".fmt(f),
                Space::Assassin => "Assassin".fmt(f),
            }
        }
    }

    pub struct Board {
        first: Color,
        spaces: Vec<Option<Space>>,
    }

    impl Board {
        pub fn new(first: Color) -> Board {
            Board {
                first,
                spaces: [None; 25].to_vec(),
            }
        }

        pub fn from_str(string: &str) -> Board {
            let mut iter = string.chars();
            let first = Color::from_char(iter.next().unwrap()).unwrap();
            let spaces: Vec<_> = iter.map(|c| Some(Space::from_char(c).unwrap())).collect();
            Board::from_vec(first, spaces)
        }

        pub fn from_vec(first: Color, spaces: Vec<Option<Space>>) -> Board {
            Board { first, spaces }
        }

        pub fn rotate_90(&self) -> Board {
            let rows: Vec<_> = self.spaces.chunks(5).collect();
            let mut spaces_90 = Vec::with_capacity(25);
            for x in 0..5 {
                for row in rows.iter().rev() {
                    spaces_90.push(row[x]);
                }
            }
            Board {
                first: self.first,
                spaces: spaces_90
            }
        }

        pub fn rotate_180(&self) -> Board {
            Board {
                first: self.first,
                spaces: self.spaces.iter().copied().rev().collect()
            }
        }

        pub fn first(&self) -> Color {
            self.first
        }

        pub fn get(&self, position: usize) -> &Option<Space> {
            self.spaces.get(position).unwrap()
        }

        pub fn set(&mut self, position: usize, space: Option<Space>) {
            self.spaces[position] = space;
        }
    }

    impl Display for Board {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            writeln!(f, "{} first", self.first)?;
            for i in 0..5 {
                for j in 0..5 {
                    match self.spaces[i * 5 + j] {
                        None => write!(f, "[?]")?,
                        Some(s) => write!(f, "[{:.1}]", s)?,
                    }
                }
                writeln!(f)?;
            }
            Ok(())
        }
    }
}
