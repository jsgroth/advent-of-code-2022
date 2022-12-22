//! Day 22: Monkey Map
//! https://adventofcode.com/2022/day/22

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Empty,
    Wall,
    Void,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Move(usize),
    RotateRight,
    RotateLeft,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rotation {
    None,
    Left,
    Right,
    Invert,
}

impl Rotation {
    fn flip(&self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            _ => *self,
        }
    }

    fn rotate(&self, other: &Self) -> Self {
        match self {
            Self::None => *other,
            Self::Invert => match other {
                Self::None => Self::Invert,
                Self::Invert => Self::None,
                Self::Left => Self::Right,
                Self::Right => Self::Left,
            }
            Self::Left => match other {
                Self::None => Self::Left,
                Self::Left => Self::Invert,
                Self::Invert => Self::Right,
                Self::Right => Self::None,
            }
            Self::Right => match other {
                Self::None => Self::Right,
                Self::Right => Self::Invert,
                Self::Invert => Self::Left,
                Self::Left => Self::None,
            }
        }
    }

    fn rotate_direction(&self, direction: &Direction) -> Direction {
        match self {
            Self::None => *direction,
            Self::Invert => direction.invert(),
            Self::Left => match direction {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            }
            Self::Right => match direction {
                Direction::Up => Direction::Right,
                Direction::Left => Direction::Up,
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Down,
            }
        }
    }

    fn rotate_coordinates(&self, i: usize, j: usize, side_len: usize) -> (usize, usize) {
        match self {
            Self::None => (i, j),
            Self::Invert => (side_len - 1 - i, side_len - 1 - j),
            Self::Left => (side_len - 1 - j, i),
            Self::Right => (j, side_len - 1 - i),
        }
    }

    fn rotate_facing(&self, dx: i32, dy: i32) -> (i32, i32) {
        let rotate_matrix = match self {
            Self::None => vec![vec![1, 0], vec![0, 1]],
            Self::Invert => vec![vec![-1, 0], vec![0, -1]],
            Self::Left => vec![vec![0, -1], vec![1, 0]],
            Self::Right => vec![vec![0, 1], vec![-1, 0]],
        };
        (dx * rotate_matrix[0][0] + dy * rotate_matrix[1][0], dx * rotate_matrix[0][1] + dy * rotate_matrix[1][1])
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn invert(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct CubeConnection {
    index: usize,
    rotation: Rotation,
}

impl CubeConnection {
    fn new(index: usize, rotation: Rotation) -> Self {
        Self { index, rotation }
    }
}

#[derive(Debug)]
struct CubeFaceInProgress {
    top_left: (usize, usize),
    map: Vec<Vec<Space>>,
    up: Option<CubeConnection>,
    left: Option<CubeConnection>,
    down: Option<CubeConnection>,
    right: Option<CubeConnection>,
}

impl CubeFaceInProgress {
    fn new(top_left: (usize, usize), map: Vec<Vec<Space>>) -> Self {
        Self {
            top_left,
            map,
            up: None,
            left: None,
            down: None,
            right: None,
        }
    }

    fn all_connections(&self) -> Vec<(Direction, CubeConnection)> {
        vec![
            (Direction::Up, self.up),
            (Direction::Left, self.left),
            (Direction::Down, self.down),
            (Direction::Right, self.right),
        ]
            .into_iter()
            .filter_map(|(direction, connection)| {
                if let Some(connection) = connection {
                    Some((direction, connection))
                } else {
                    None
                }
            })
            .collect()
    }

    fn set_if_none(&mut self, direction: &Direction, connection: CubeConnection) {
        match direction {
            Direction::Up => {
                self.up.get_or_insert(connection);
            }
            Direction::Left => {
                self.left.get_or_insert(connection);
            }
            Direction::Down => {
                self.down.get_or_insert(connection);
            }
            Direction::Right => {
                self.right.get_or_insert(connection);
            }
        }
    }

    fn is_fully_populated(&self) -> bool {
        self.up.is_some() && self.left.is_some() && self.down.is_some() && self.right.is_some()
    }

    fn to_cube_face(self) -> CubeFace {
        CubeFace {
            top_left: self.top_left,
            map: self.map,
            up: self.up.unwrap(),
            left: self.left.unwrap(),
            down: self.down.unwrap(),
            right: self.right.unwrap(),
        }
    }
}

#[derive(Debug)]
struct CubeFace {
    top_left: (usize, usize),
    map: Vec<Vec<Space>>,
    up: CubeConnection,
    left: CubeConnection,
    down: CubeConnection,
    right: CubeConnection,
}

fn solve(input: &str) -> usize {
    let (map, instructions) = parse_input(input);

    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    let mut i = 0;
    let mut j = map[i].iter().position(|&space| space != Space::Void).expect("first line should have a non-void space");
    let mut dx = 1;
    let mut dy = 0;
    for instruction in &instructions {
        match instruction {
            Instruction::Move(n) => {
                for _ in 0..*n {
                    let mut new_i = (i as i32 + rows + dy) % rows;
                    let mut new_j = (j as i32 + cols + dx) % cols;

                    while map[new_i as usize][new_j as usize] == Space::Void {
                        new_i = (new_i + rows + dy) % rows;
                        new_j = (new_j + cols + dx) % cols;
                    }

                    if map[new_i as usize][new_j as usize] == Space::Wall {
                        break;
                    }

                    i = new_i as usize;
                    j = new_j as usize;
                }
            }
            Instruction::RotateRight => {
                let t = dy;
                dy = dx;
                dx = -t;
            }
            Instruction::RotateLeft => {
                let t = dy;
                dy = -dx;
                dx = t;
            }
        }
    }

    solution(i, j, dx, dy)
}

fn solve_part_2(input: &str) -> usize {
    let (map, instructions) = parse_input(input);

    let cube_faces = split_map_into_cube_faces(&map);

    let side_len = cube_faces[0].map.len();

    let mut cube_index = 0;
    let mut i = 0;
    let mut j = 0;
    let mut dx = 1;
    let mut dy = 0;

    for instruction in &instructions {
        match instruction {
            Instruction::Move(n) => {
                for _ in 0..*n {
                    let mut new_cube_index = cube_index;
                    let mut new_i = i;
                    let mut new_j = j;
                    let mut new_dx = dx;
                    let mut new_dy = dy;

                    let connection = if i == 0 && dy < 0 {
                        Some(&cube_faces[cube_index].up)
                    } else if i == side_len - 1 && dy > 0 {
                        Some(&cube_faces[cube_index].down)
                    } else if j == 0 && dx < 0 {
                        Some(&cube_faces[cube_index].left)
                    } else if j == side_len - 1 && dx > 0 {
                        Some(&cube_faces[cube_index].right)
                    } else {
                        None
                    };

                    if let Some(connection) = connection {
                        new_cube_index = connection.index;
                        (new_i, new_j) = connection.rotation.rotate_coordinates(i, j, side_len);
                        (new_dx, new_dy) = connection.rotation.rotate_facing(dx, dy);
                    }

                    new_i = (((new_i + side_len) as i32 + new_dy) % side_len as i32) as usize;
                    new_j = (((new_j + side_len) as i32 + new_dx) % side_len as i32) as usize;

                    if cube_faces[new_cube_index].map[new_i][new_j] == Space::Wall {
                        break;
                    }

                    cube_index = new_cube_index;
                    i = new_i;
                    j = new_j;
                    dx = new_dx;
                    dy = new_dy;
                }
            }
            Instruction::RotateRight => {
                let t = dy;
                dy = dx;
                dx = -t;
            }
            Instruction::RotateLeft => {
                let t = dy;
                dy = -dx;
                dx = t;
            }
        }
    }

    let final_i = cube_faces[cube_index].top_left.0 + i;
    let final_j = cube_faces[cube_index].top_left.1 + j;

    solution(final_i, final_j, dx, dy)
}

fn solution(i: usize, j: usize, dx: i32, dy: i32) -> usize {
    let facing_value = match (dx, dy) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => panic!("unexpected dx/dy values: ({dx}, {dy})"),
    };

    1000 * (i + 1) + 4 * (j + 1) + facing_value
}

fn split_map_into_cube_faces(map: &Vec<Vec<Space>>) -> Vec<CubeFace> {
    let num_non_void: usize = map.iter().map(|row| {
        row.iter().filter(|&&space| space != Space::Void).count()
    }).sum();

    let cube_face_size = num_non_void / 6;
    let cube_side_len = (cube_face_size as f64).sqrt().round() as usize;

    let mut cube_top_lefts: Vec<(usize, usize)> = Vec::new();
    for i in (0..map.len()).step_by(cube_side_len) {
        for j in (0..map[i].len()).step_by(cube_side_len) {
            if map[i][j] != Space::Void {
                cube_top_lefts.push((i, j));
            }
        }
    }

    let mut cube_faces_in_progress: Vec<CubeFaceInProgress> = Vec::with_capacity(6);
    for &(tl_i, tl_j) in &cube_top_lefts {
        let mut cube_map = vec![vec![Space::Void; cube_side_len]; cube_side_len];
        for i in 0..cube_side_len {
            for j in 0..cube_side_len {
                cube_map[i][j] = map[tl_i + i][tl_j + j];
            }
        }
        cube_faces_in_progress.push(CubeFaceInProgress::new((tl_i, tl_j), cube_map));
    }

    for cube_index in 0..6 {
        let (tl_i, tl_j) = cube_top_lefts[cube_index];
        if tl_i > 0 {
            if let Some(other_index) = cube_top_lefts.iter().position(|&(i, j)| i == tl_i - cube_side_len && j == tl_j) {
                cube_faces_in_progress[cube_index].up = Some(CubeConnection::new(other_index, Rotation::None));
            }
        }
        if tl_j > 0 {
            if let Some(other_index) = cube_top_lefts.iter().position(|&(i, j)| i == tl_i && j == tl_j - cube_side_len) {
                cube_faces_in_progress[cube_index].left = Some(CubeConnection::new(other_index, Rotation::None));
            }
        }
        if let Some(other_index) = cube_top_lefts.iter().position(|&(i, j)| i == tl_i + cube_side_len && j == tl_j) {
            cube_faces_in_progress[cube_index].down = Some(CubeConnection::new(other_index, Rotation::None));
        }
        if let Some(other_index) = cube_top_lefts.iter().position(|&(i, j)| i == tl_i && j == tl_j + cube_side_len) {
            cube_faces_in_progress[cube_index].right = Some(CubeConnection::new(other_index, Rotation::None));
        }
    }

    while !cube_faces_in_progress.iter().all(CubeFaceInProgress::is_fully_populated) {
        for i in 0..cube_faces_in_progress.len() {
            for (direction, connection) in cube_faces_in_progress[i].all_connections() {
                for (other_direction, other_connection) in cube_faces_in_progress[connection.index].all_connections() {
                    let other_rotated = connection.rotation.flip().rotate_direction(&other_direction);
                    if direction == other_rotated || direction == other_rotated.invert() {
                        continue;
                    }

                    let (rotation, direction) = match (direction, other_rotated) {
                        (Direction::Up, Direction::Left) => (Rotation::Right, Direction::Left),
                        (Direction::Up, Direction::Right) => (Rotation::Left, Direction::Right),
                        (Direction::Left, Direction::Up) => (Rotation::Left, Direction::Up),
                        (Direction::Left, Direction::Down) => (Rotation::Right, Direction::Down),
                        (Direction::Down, Direction::Left) => (Rotation::Left, Direction::Left),
                        (Direction::Down, Direction::Right) => (Rotation::Right, Direction::Right),
                        (Direction::Right, Direction::Up) => (Rotation::Right, Direction::Up),
                        (Direction::Right, Direction::Down) => (Rotation:: Left, Direction::Down),
                        _ => panic!("should never happen due to above if check: direction={direction:?}, other_direction={other_direction:?}"),
                    };

                    let rotated_rotation = rotation.rotate(&connection.rotation).rotate(&other_connection.rotation);
                    cube_faces_in_progress[i].set_if_none(&direction, CubeConnection::new(other_connection.index, rotated_rotation));
                }
            }
        }
    }

    cube_faces_in_progress.into_iter().map(CubeFaceInProgress::to_cube_face).collect()
}

fn parse_input(input: &str) -> (Vec<Vec<Space>>, Vec<Instruction>) {
    let lines: Vec<_> = input.lines().collect();

    let map_lines = &lines[..lines.len() - 2];
    let map = parse_map(map_lines);

    let instructions = parse_instructions(input.lines().last().expect("input should not be empty"));

    (map, instructions)
}

fn parse_map(map_lines: &[&str]) -> Vec<Vec<Space>> {
    let rows = map_lines.len();
    let cols = map_lines.iter().map(|line| line.len()).max().expect("map should not be empty");

    let mut map = vec![vec![Space::Void; cols]; rows];
    for (i, line) in map_lines.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            map[i][j] = match c {
                ' ' => Space::Void,
                '.' => Space::Empty,
                '#' => Space::Wall,
                _ => panic!("unexpected char: {c}"),
            }
        }
    }

    map
}

fn parse_instructions(line: &str) -> Vec<Instruction> {
    let mut chars = line.chars().peekable();

    let mut instructions: Vec<Instruction> = Vec::new();
    while let Some(&c) = chars.peek() {
        let instruction = match c {
            _c @ '0'..='9' => {
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if !('0'..='9').contains(&c) {
                        break;
                    }
                    s.push(chars.next().unwrap());
                }
                Instruction::Move(s.parse().expect("move distance should be an integer"))
            }
            'R' => {
                chars.next();
                Instruction::RotateRight
            }
            'L' => {
                chars.next();
                Instruction::RotateLeft
            }
            _ => panic!("unexpected char: {c}"),
        };
        instructions.push(instruction);
    }

    instructions
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("unable to read input file");

    let solution1 = solve(&input);
    println!("{solution1}");

    let solution2 = solve_part_2(&input);
    println!("{solution2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample_input/sample22.txt");

    #[test]
    fn test_sample_input_part_1() {
        assert_eq!(6032, solve(SAMPLE_INPUT));
    }

    #[test]
    fn test_sample_input_part_2() {
        assert_eq!(5031, solve_part_2(SAMPLE_INPUT));
    }
}