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
| [16 - Packet Decoder](/src/day16.rs) | - | - | - | [🔗](https://adventofcode.com/2021/day/16) |
| [17 - Trick Shot](/src/day17.rs) | 999ns | 100ns | 443us 027ns | [🔗](https://adventofcode.com/2021/day/17) |
| [18 - Snailfish](/src/day18.rs) | 93us | 2ms 471us | 44ms 780us | [🔗](https://adventofcode.com/2021/day/18) |
| [19 - Beacon Scanner](/src/day19.rs) | 21s 394ms | 82ns | 5us 190ns | [🔗](https://adventofcode.com/2021/day/19) |

#### Note
- **Day 15 part 2**: ~~yep, [1 hour and 20 minutes](https://imgur.com/a/yAlGIHN)~~ Well, its execution time is now under 200 milliseconds...
- **Day 17 part 2**: Brute forced it, computer's problems require programming solutions
- **Day 19**: I calculate the beacons and scanners location while parsing the input since it does not require the full input to be done, that's why the execution time are fast but the parsing is not

## Dependencies
- **humantime** - to display the running time
- **owo-colors** - to print colored result in terminal