# Advent-of-Code-2021
My solution for the advent of code 2021, written in Rust

## Error Handle
**NOPE!!!** `unwrap()` everything everywhere

## Use
To run all of the advent of code solutions
```sh
cargo run --release
```
To run only a specific day (replace {DAY_NUMBER} with the number of that day)
```sh
cargo run --release -- {DAY_NUMBER}
```

## Learned

- **Day 15** - Dijkstra path finding algorithm
- **Day 16** - Lisp runtime implementation
- **Day 19** - 3D vector rotation
- **Day 21** - *Using Steins Gate* to count timeline branches
- *Memory check* - day6 day14, these will cost terabytes of RAM if not using the right algorithm

## Performance

These timing was done on my shitty $5 VPS

| Day - Name | Parse time | Part 1 | Part 2 | AoC link |
| :--------- | ---------: | -----: | -----: | :------: |
| [01 - Sonar Sweep](/src/day01.rs) | 93us | 4us | 19us | [🔗](https://adventofcode.com/2021/day/1) |
| [02 - Dive!](/src/day02.rs) | 79us | 8us | 8us | [🔗](https://adventofcode.com/2021/day/2) |
| [03 - Binary Diagnostic](/src/day03.rs) | 131us | 86us | 61us | [🔗](https://adventofcode.com/2021/day/3) |
| [04 - Giant Squid](/src/day04.rs) | 90us | 94us | 62us | [🔗](https://adventofcode.com/2021/day/4) |
| [05 - Hydrothermal Venture](/src/day05.rs) | 126us | 1ms 560us | 2ms 72us | [🔗](https://adventofcode.com/2021/day/5) |
| [06 - Lanternfish](/src/day06.rs) | 7us | 814ns | 1us 61ns | [🔗](https://adventofcode.com/2021/day/6) |
| [07 - The Treachery of Whales](/src/day07.rs) | 51us | 450us | 5ms 594us | [🔗](https://adventofcode.com/2021/day/7) |
| [08 - Seven Segment Search](/src/day08.rs) | 93us | 20us | 432us | [🔗](https://adventofcode.com/2021/day/8) |
| [09 - Smoke Basin](/src/day09.rs) | 89us | 423us | 3ms 927us | [🔗](https://adventofcode.com/2021/day/9) |
| [10 - Syntax Scoring](/src/day10.rs) | 41us | 96us | 96us | [🔗](https://adventofcode.com/2021/day/10) |
| [11 - Dumbo Octopus](/src/day11.rs) | 1us 908ns | 1ms 474us | 6ms 766us | [🔗](https://adventofcode.com/2021/day/11) |
| [12 - Passage Pathing](/src/day12.rs) | 11us | 2ms 822us | 125ms | [🔗](https://adventofcode.com/2021/day/12) |
| [13 - Transparent Origami](/src/day13.rs) | 209us | 152us | 1ms 120us | [🔗](https://adventofcode.com/2021/day/13) |
| [14 - Extended Polymerization](/src/day14.rs) | 8us | 35us | 164us | [🔗](https://adventofcode.com/2021/day/14) |
| [15 - Chiton](/src/day15.rs) | 16us | 5ms 977us | 183ms | [🔗](https://adventofcode.com/2021/day/15) |
| [16 - Packet Decoder](/src/day16.rs) | 220us | 2us 776ns | 3us 779ns | [🔗](https://adventofcode.com/2021/day/16) |
| [17 - Trick Shot](/src/day17.rs) | 999ns | 100ns | 443us 027ns | [🔗](https://adventofcode.com/2021/day/17) |
| [18 - Snailfish](/src/day18.rs) | 93us | 2ms 471us | 44ms 780us | [🔗](https://adventofcode.com/2021/day/18) |
| [19 - Beacon Scanner](/src/day19.rs) | 21s 394ms | 82ns | 5us 190ns | [🔗](https://adventofcode.com/2021/day/19) |
| [20 - Trench Map](/src/day20.rs) | 677us | 10ms 355us | 629ms | [🔗](https://adventofcode.com/2021/day/20) |
| [21 - Dirac Dice](/src/day21.rs) | 250ns | 1us 194ns | 23ms 612us | [🔗](https://adventofcode.com/2021/day/21) |
| [22 - Reactor Reboot](/src/day22.rs) | 283us 973ns | 384us 484ns | 4ms 874us | [🔗](https://adventofcode.com/2021/day/22) |
| [23 - Amphipod](/src/day23.rs) | - | - | - | [🔗](https://adventofcode.com/2021/day/23) |
| [24 - Arithmetic Logic Unit](/src/day24.rs) | 11us 138ns | 130ns | 112ns | [🔗](https://adventofcode.com/2021/day/24) |
| [25 - Sea Cucumber](/src/day25.rs) | 259us 90ns | 216ms 56us | - | [🔗](https://adventofcode.com/2021/day/25) |

#### Note
- **Day 15 part 2**: ~~yep, [1 hour and 20 minutes](https://imgur.com/a/yAlGIHN)~~ Well, its execution time is now under 200 milliseconds...
- **Day 16**: The bit reader can be *a lot* faster, I use an inefficient implementation here
- **Day 17 part 2**: Brute forced it, computer's problems require programming solutions
- **Day 19**: I calculate the beacons and scanners location while parsing the input since it does not require the full input to be done, that's why the execution times are fast but the parsing is not. Also I have no idea how to optimize it
- **Day 22**: Not the algorithm that I came up with, rather say it's a port of [this cool Javascript impletation](https://www.reddit.com/r/adventofcode/comments/rlxhmg/2021_day_22_solutions/hpjbx3t/) instead, because it's extremely cooool
- **Day 23**: Solved it by hand thanks to [this website](https://amphipod.net/), maybe I will done it programmatically later
- **Day 24**: The input for example and real data is the same, because there is no example for it. Solved it by hand because it's easier...

## Dependencies
- **humantime** - to display the running time
- **owo-colors** - to print colored result in terminal