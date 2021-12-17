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

| Day - Name | Parse time | Part 1 | Part 2 |
| :--------- | :--------: | :----: | :----: |
| [01 - Sonar Sweep](https://adventofcode.com/2021/day/1) | 93us | 4us | 19us |
| [02 - Dive!](https://adventofcode.com/2021/day/2) | 79us | 8us | 8us |
| [03 - Binary Diagnostic](https://adventofcode.com/2021/day/3) | 131us | 86us | 61us |
| [04 - Giant Squid](https://adventofcode.com/2021/day/4) | 90us | 94us | 62us |
| [05 - Hydrothermal Venture](https://adventofcode.com/2021/day/5) | 126us | 1ms 560us | 2ms 72us |
| [06 - Lanternfish](https://adventofcode.com/2021/day/6) | 7us | 814ns | 1us 61ns |
| [07 - The Treachery ò Whales](https://adventofcode.com/2021/day/7) | 51us | 450us | 5ms 594us |
| [08 - Seven Segment Search](https://adventofcode.com/2021/day/8) | 93us | 20us | 432us |
| [09 - Smoke Basin](https://adventofcode.com/2021/day/9) | 89us | 423us | 3ms 927us |
| [10 - Syntax Scoring](https://adventofcode.com/2021/day/10) | 41us | 96us | 96us |
| [11 - Dumbo Octopus](https://adventofcode.com/2021/day/11) | 1us 908ns | 1ms 474us | 6ms 766us |
| [12 - Passage Pathing](https://adventofcode.com/2021/day/12) | 11us | 2ms 822us | 125ms |
| [13 - Transparent Origami](https://adventofcode.com/2021/day/13) | 209us | 152us | 1ms 120us |
| [14 - Extended Polymerization](https://adventofcode.com/2021/day/14) | 8us | 35us | 164us |
| [15 - Chiton](https://adventofcode.com/2021/day/15) | 23us | 2s 605ms | 1h 25m |
| [16 - Packet Decoder](https://adventofcode.com/2021/day/16) | - | - | - |
| [17 - Trick Shot](https://adventofcode.com/2021/day/17) | - | - | - |

## Dependencies
- **humantime** - to display the running time
- **owo-colors** - to print colored result in terminal