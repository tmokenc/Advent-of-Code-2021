use std::convert::TryInto;

type Board = [u8; 5*5];

pub struct GiantSquid {
    calls: Vec<u8>,
    boards: Vec<Board>,
}

fn min_calls_to_win(board: &Board, calls: &[u8]) -> usize {
    let mut row = [0; 5];
    let mut column = [0; 5];
    let mut count = 0;
    
    for call in calls {
        count += 1;
        
        if let Some(idx) = board.iter().position(|v| v == call) {
            let row_idx = idx % 5;
            let column_idx = idx / 5;
            
            row[row_idx] += 1;
            column[column_idx] += 1;
            
            if row[row_idx] == 5 || column[column_idx] == 5 {
                break;
            }
        }
    }
    
    count
}

impl crate::AdventOfCode for GiantSquid {
    fn new(input: &str) -> Self {
        let mut iter = input.lines();
        let calls = iter
            .next()
            .unwrap()
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect::<Vec<u8>>();
            
        let mut boards = Vec::new();
        
        while iter.next().is_some() {
            let board: Board = iter
                .by_ref()
                .take(5)
                .flat_map(|v| v.split_whitespace())
                .map(|v| v.parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
                .try_into()
                .unwrap();
                
            boards.push(board);
        }
        
        Self {
            calls, boards,
        }
    }
    
    fn part1(&self) -> u64 {
        let (board, calls_count) = self.boards
            .iter()
            .map(|&v| (v, min_calls_to_win(&v, &self.calls)))
            .min_by_key(|v| v.1)
            .unwrap();
        
        let calls = self.calls.get(..calls_count).unwrap();
        let base_point: u64 = board.iter()
            .filter(|&v| !calls.iter().any(|x| x == v))
            .map(|&v| v as u64)
            .sum();
            
        base_point * *calls.last().unwrap() as u64
    }
    
    fn part2(&self) -> u64 {
        let (board, calls_count) = self.boards
            .iter()
            .map(|&v| (v, min_calls_to_win(&v, &self.calls)))
            .max_by_key(|v| v.1) // the only different from part 1
            .unwrap();
        
        let calls = self.calls.get(..calls_count).unwrap();
        let base_point: u64 = board.iter()
            .filter(|&v| !calls.iter().any(|x| x == v))
            .map(|&v| v as u64)
            .sum();
            
        base_point * *calls.last().unwrap() as u64
    }
}