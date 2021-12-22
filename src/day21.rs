use std::collections::HashMap;

type GameState = [Player; 2];

// got these number on reddit...
const QUANTUM_DICE: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Clone, PartialEq, Eq, Hash)]
struct Player {
    position: u8,
    score: u64,
}

impl Player {
    fn step_by(&mut self, steps: u64) {
        self.position += (steps % 10) as u8;

        if self.position > 10 {
            self.position -= 10;
        }

        self.score += self.position as u64;
    }
}

struct SteinGate {
    universes: HashMap<GameState, u64>,
    in_progress: u64,
    completed: [u64; 2],
}

impl SteinGate {
    fn new(state: GameState) -> Self {
        let mut universes = HashMap::new();
        universes.insert(state, 1);

        Self {
            universes,
            in_progress: 1,
            completed: [0; 2],
        }
    }

    fn progress(&mut self) {
        while self.in_progress > 0 {
            self.play_turn(0);
            self.play_turn(1);
        }
    }

    fn play_turn(&mut self, player_idx: usize) {
        for (old_state, old_count) in std::mem::take(&mut self.universes) {
            self.in_progress -= old_count;

            for (roll, cases) in QUANTUM_DICE {
                let mut new_state = old_state.clone();
                new_state[player_idx].step_by(roll);

                let universes_in_progress = cases * old_count;

                if new_state[player_idx].score >= 21 {
                    self.completed[player_idx] += universes_in_progress;
                    continue;
                }

                *self.universes.entry(new_state).or_insert(0) += universes_in_progress;
                self.in_progress += universes_in_progress;
            }
        }
    }
}

pub struct DiracDice {
    state: [Player; 2],
}

impl crate::AdventOfCode for DiracDice {
    fn new(input: &str) -> Self {
        let mut iter = input
            .lines()
            .map(|v| v.get("Player 1 starting position: ".len()..).unwrap())
            .map(|v| v.parse().unwrap())
            .map(|v| Player {
                position: v,
                score: 0,
            });

        let p1 = iter.next().unwrap();
        let p2 = iter.next().unwrap();

        Self { state: [p1, p2] }
    }

    fn part1(&self) -> u64 {
        let mut dice: u64 = 1;
        let mut rolled: u64 = 0;
        let mut players = self.state.clone();

        while players[0].score < 1000 && players[1].score < 1000 {
            let steps = dice * 3 + 3;
            let player_idx = (rolled & 1) as usize;

            players[player_idx].step_by(steps);

            dice += 3;
            rolled += 3;

            if dice > 100 {
                dice -= 100;
            }
        }

        players.into_iter().map(|v| v.score).min().unwrap() * rolled
    }

    fn part2(&self) -> u64 {
        let mut stein_gate = SteinGate::new(self.state.clone());
        stein_gate.progress();
        stein_gate.completed.into_iter().max().unwrap()
    }
}
