# m3h's Advent of Code 2023

See https://adventofcode.com for the challenges

I made an effort to push code that is largely as submitted (except for some auto-formatting, and re-structuring of the repo as a whole).
That also means that the code is meant to get me a solution fast, with no focus on ease of reading or speed (unless necessary to get a solution)

To see 'em in action, run 
```
cargo install cargo-aoc # Rust AoC framework
cargo aoc credentials TOKEN # Grab token from browser

# download all inputs
for day in `seq 1 25`; do cargo aoc input --day ${day}; done

cargo run
```
