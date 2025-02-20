use std::ops::Add;
use std::str::FromStr;
use std::collections::HashSet;
use std::usize;

fn main() {
    let input_file_name = "input.txt";
    let contents = std::fs::read_to_string(input_file_name).expect("Failed to read the input file");
    println!("{}", contents.len());
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Add<Direction> for Pos {
    type Output = Self;

    fn add(self, direction: Direction) -> Self::Output {
        match direction {
            Direction::North => Self::new(self.x, self.y + 1),
            Direction::South => Self::new(self.x, self.y - 1),
            Direction::East => Self::new(self.x + 1, self.y),
            Direction::West => Self::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IllegalChar(char);

impl TryFrom<char> for Direction {
    type Error = IllegalChar;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '^' => Self::North,
            'v' => Self::South,
            '>' => Self::East,
            '<' => Self::West,
            _ => return Err(IllegalChar(c)),
        })
    }
}

pub struct Moves {
    moves: Vec<(usize, Direction)>,
}

impl FromStr for Moves {
    type Err = IllegalChar;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .chars()
            .map(Direction::try_from)
            .collect::<Result<Vec<Direction>, IllegalChar>>()?;
        let moves = moves.into_iter().enumerate().collect::<Vec<_>>();
        Ok(Self { moves })
    }
}

pub struct VisitedHouses {
    visited_houses: HashSet<Pos>,
    current_robo_pos: Pos,
    current_human_pos: Pos,
}

impl VisitedHouses {
    #[must_use]
    pub fn new() -> Self {
        let initial_position = Pos::new(0, 0);
        let mut visited_houses = HashSet::new();
        visited_houses.insert(initial_position);

        Self {
            visited_houses,
            current_robo_pos: initial_position,
            current_human_pos: initial_position,
        }
    }

    #[must_use]
    pub fn num_visited_houses(&self) -> usize {
        self.visited_houses.len()
    }

    #[must_use]
    pub const fn current_human_pos(&self) -> Pos {
        self.current_human_pos
    }

    #[must_use]
    pub const fn current_robo_pos(&self) -> Pos {
        self.current_robo_pos
    }

    pub fn perform_move(&mut self, (index, direction): (usize, Direction)) {
        if index % 2 == 0 {
            let new_position = self.current_human_pos + direction;
            self.current_human_pos = new_position;
            self.visited_houses.insert(new_position);
        } else {
            let new_position = self.current_robo_pos + direction;
            self.current_robo_pos = new_position;
            self.visited_houses.insert(new_position);
        }
    }

    pub fn perform_moves(&mut self, moves: Moves) {
        for m in moves.moves {
            self.perform_move(m);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visited_houses_new() {
        let visited_houses = VisitedHouses::new();
        assert_eq!(visited_houses.num_visited_houses(), 1);
        // What do you want to do about the current position?
        assert_eq!(visited_houses.current_human_pos, Pos::new(0, 0));
        assert_eq!(visited_houses.current_robo_pos, Pos::new(0, 0));
    }

    #[test]
    fn test_direction_try_from() {
        assert_eq!('^'.try_into(), Ok(Direction::North));
        assert_eq!('v'.try_into(), Ok(Direction::South));
        assert_eq!('<'.try_into(), Ok(Direction::West));
        assert_eq!('>'.try_into(), Ok(Direction::East));
        assert_eq!(Direction::try_from('x'), Err(IllegalChar('x')));
    }

    #[test]
    fn test_move_north_south() {
        let mut visited_houses = VisitedHouses::new();
        visited_houses.perform_move((0, Direction::North));
        visited_houses.perform_move((1, Direction::South));
        assert_eq!(visited_houses.num_visited_houses(), 3);
        assert_eq!(visited_houses.current_human_pos, Pos::new(0, 1));
        assert_eq!(visited_houses.current_robo_pos, Pos::new(0, -1));
    }

    #[test]
    fn test_square_moves() {
        let mut visited_houses = VisitedHouses::new();
        let moves = Moves::from_str("^>v<").unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 3);
        assert_eq!(visited_houses.current_human_pos, Pos::new(0, 0));
        assert_eq!(visited_houses.current_robo_pos, Pos::new(0, 0));
    }

    #[test]
    fn test_up_down_moves() {
        let mut visited_houses = VisitedHouses::new();
        let moves = Moves::from_str("^v^v^v^v^v").unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 11);
        assert_eq!(visited_houses.current_human_pos, Pos::new(0, 5));
        assert_eq!(visited_houses.current_robo_pos, Pos::new(0, -5));
    }

    #[test]
    fn test_aoc_input() {
        let mut visited_houses = VisitedHouses::new();
        let moves = Moves::from_str(include_str!("../../input.txt").trim()).unwrap();
        visited_houses.perform_moves(moves);
        assert_eq!(visited_houses.num_visited_houses(), 2639);
    }
}
