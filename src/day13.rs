use std::collections::HashSet;
use std::mem;

#[derive(Clone, Copy)]
enum Fold {
    X(usize),
    Y(usize),
}

impl Fold {
    fn new(s: &str) -> Self {
        let mut ins = s.get("fold along ".len()..).unwrap().split('=');
        let fold_along = ins.next();
        let position = ins.next().unwrap().parse::<usize>().unwrap();

        match fold_along {
            Some("x") => Self::X(position),
            Some("y") => Self::Y(position),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone)]
struct Paper {
    points: HashSet<(usize, usize)>,
}

fn new_position(mut pos: usize, folded: usize) -> usize {
    if pos > folded {
        let diff = pos - folded;
        pos = folded - diff;
    }

    pos
}

impl Paper {
    fn display(&self) {
        let mut board = Vec::new();

        for &(x, y) in &self.points {
            if y + 1 > board.len() {
                board.resize(y + 1, Vec::new());
            }

            if x + 1 > board[y].len() {
                board[y].resize(x + 1, " ");
            }

            board[y][x] = "#";
        }

        for line in board {
            for ch in line {
                print!("{}", ch);
            }

            print!("\n");
        }
    }

    fn fold(&mut self, f: Fold) {
        match f {
            Fold::X(idx) => self.fold_x(idx),
            Fold::Y(idx) => self.fold_y(idx),
        }
    }

    fn fold_x(&mut self, x_idx: usize) {
        for (x, y) in mem::take(&mut self.points) {
            if x == x_idx {
                continue;
            }

            let new_x = new_position(x, x_idx);
            self.points.insert((new_x, y));
        }
    }

    fn fold_y(&mut self, y_idx: usize) {
        for (x, y) in mem::take(&mut self.points) {
            if y == y_idx {
                continue;
            }

            let new_y = new_position(y, y_idx);
            self.points.insert((x, new_y));
        }
    }
}

pub struct TransparentOrigami {
    paper: Paper,
    instructions: Vec<Fold>,
}

impl crate::AdventOfCode for TransparentOrigami {
    fn new(input: &str) -> Self {
        let mut iter = input.lines();
        let points = iter
            .by_ref()
            .take_while(|v| !v.is_empty())
            .map(|v| {
                let mut i = v.split(',').map(|x| x.parse::<usize>().unwrap());
                (i.next().unwrap(), i.next().unwrap())
            })
            .collect::<HashSet<_>>();

        let instructions = iter.map(Fold::new).collect::<Vec<_>>();

        Self {
            paper: Paper { points },
            instructions,
        }
    }

    fn part1(&self) -> u64 {
        let mut paper = self.paper.clone();
        paper.fold(self.instructions[0]);
        paper.points.len() as u64
    }

    fn part2(&self) -> u64 {
        let mut paper = self.paper.clone();
        self.instructions
            .iter()
            .copied()
            .for_each(|v| paper.fold(v));
        paper.display();
        0
    }
}
