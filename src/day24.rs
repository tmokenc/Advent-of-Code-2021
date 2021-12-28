#[derive(Clone, Copy)]
struct StepParams {
    do_cmp: bool,
    cmp_now: i8,
    cmp_later: i8,
}

#[derive(Clone, Copy)]
struct Rule {
    cmp_to: usize,
    val: i8,
}

impl Rule {
    fn new(cmp_to: usize, val: i8) -> Self {
        Self { cmp_to, val }
    }

    fn from_params(params: [StepParams; 14]) -> [Self; 14] {
        let mut cmp_stack = Vec::new();
        let mut rules = [None; 14];
        for (i, step_params) in params.into_iter().enumerate() {
            if step_params.do_cmp {
                let Self { cmp_to, val } = cmp_stack.pop().unwrap();
                rules[i] = Some(Self::new(cmp_to, val + step_params.cmp_now));
                rules[cmp_to] = Some(Self::new(i, -val - step_params.cmp_now));
            } else {
                cmp_stack.push(Self::new(i, step_params.cmp_later))
            }
        }
        rules.map(Option::unwrap)
    }
}

pub struct ArithmeticLogicUnit {
    rules: [Rule; 14],
}

impl crate::AdventOfCode for ArithmeticLogicUnit {
    fn new(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let mut params = [None; 14];
        let get_param = |line_id: usize| lines[line_id].rsplit_once(' ').unwrap().1.parse().unwrap();
        for i in 0..14 {
            params[i] = Some(StepParams {
                do_cmp: get_param(i * 18 + 4) == 26,
                cmp_now: get_param(i * 18 + 5),
                cmp_later: get_param(i * 18 + 15),
            })
        }

        let rules = Rule::from_params(params.map(Option::unwrap));
        Self { rules }
    }

    fn part1(&self) -> u64 {
        self.rules
            .iter()
            .map(|r| 9.min(9 + r.val))
            .fold(0, |v, x| v * 10 + x as u64)
    }

    fn part2(&self) -> u64 {
        self.rules
            .iter()
            .map(|r| 1.max(1 + r.val))
            .fold(0, |v, x| v * 10 + x as u64)
    }
}