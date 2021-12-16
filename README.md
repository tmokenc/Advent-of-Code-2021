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

## Dependencies
**humantime** - to display the running time
**owo-colors** - to print colored result in terminal