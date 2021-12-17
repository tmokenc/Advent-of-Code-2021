# Advent-of-Code-2021
My solution for the advent of code 2021, written in Rust

## Running time
My implementation solves AoC 2021 problems from microsecond (us) to 5 milliseconds (ms)<br>
Exception is the problem day 12 (Passage Pathing) - 88ms

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

## Performance

| Day - Name | Parse time | Part 1 | Part 2 | AoC link |
| :--------- | ---------: | -----: | -----: | :------: |
| [01 - Sonar Sweep](/src/day01.rs) | 93us | 4us | 19us | [🔗](https://adventofcode.com/2021/day/1) |
| [02 - Dive!](/src/day02.rs) | 79us | 8us | 8us | [🔗](https://adventofcode.com/2021/2) |
| [03 - Binary Diagnostic](/src/day03.rs) | 131us | 86us | 61us | [🔗](https://adventofcode.com/2021/3) |
| [04 - Giant Squid](/src/day04.rs) | 90us | 94us | 62us | [🔗](https://adventofcode.com/2021/4) |
| [05 - Hydrothermal Venture](/src/day05.rs) | 126us | 1ms 560us | 2ms 72us | [🔗](https://adventofcode.com/2021/5) |
| [06 - Lanternfish](/src/day06.rs) | 7us | 814ns | 1us 61ns | [🔗](https://adventofcode.com/2021/6) |
| [07 - The Treachery of Whales](/src/day07.rs) | 51us | 450us | 5ms 594us | [🔗](https://adventofcode.com/2021/7) |
| [08 - Seven Segment Search](/src/day08.rs) | 93us | 20us | 432us | [🔗](https://adventofcode.com/2021/8) |
| [09 - Smoke Basin](/src/day09.rs) | 89us | 423us | 3ms 927us | [🔗](https://adventofcode.com/2021/9) |
| [10 - Syntax Scoring](/src/day10.rs) | 41us | 96us | 96us | [🔗](https://adventofcode.com/2021/10) |
| [11 - Dumbo Octopus](/src/day11.rs) | 1us 908ns | 1ms 474us | 6ms 766us | [🔗](https://adventofcode.com/2021/11) |
| [12 - Passage Pathing](/src/day12.rs) | 11us | 2ms 822us | 125ms | [🔗](https://adventofcode.com/2021/12) |
| [13 - Transparent Origami](/src/day13.rs) | 209us | 152us | 1ms 120us | [🔗](https://adventofcode.com/2021/13) |
| [14 - Extended Polymerization](/src/day14.rs) | 8us | 35us | 164us | [🔗](https://adventofcode.com/2021/14) |
| [15 - Chiton](/src/day15.rs) | 23us | 2s 605ms | 1h 25m | [🔗](https://adventofcode.com/2021/15) |
| 16 - Packet Decoder | - | - | - | [🔗](https://adventofcode.com/2021/16) |
| 17 - Trick Shot | - | - | - | [🔗](https://adventofcode.com/2021/17) |

## Dependencies
- **humantime** - to display the running time
- **owo-colors** - to print colored result in terminal