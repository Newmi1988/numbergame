[![Rust](https://github.com/Newmi1988/numbergame/actions/workflows/rust.yml/badge.svg)](https://github.com/Newmi1988/numbergame/actions/workflows/rust.yml)
# Solver for the number round of the show countdown ⏱

A friend approached me with this fun little puzzle. We talked a bit about different strategies that can be applied to solve this game in under 30 seconds.

There are solutions out there that uses precalculated tables and a simple lookup. Python was deemed to slow, so my idea was to use rust to calculate all combinations on the fly to find the equation that leads to the target number.

The equation is found using a simple hashmap that maps a value to a struct containing the elements of every operation as well as the used operation. 

Struct with "references" to ingoing numbers:
```rust
pub struct CalculatedNumber<'game> {
    pub value: u32,
    pub left_element: u32,
    pub right_element: u32,
    pub operation: &'game str,
}
```

# My solution
It's a pretty brute force approach. Just combine the initial numbers with the given actions (+,-,*,/) and keep track of the newly generated values. Iterate till you find the target.

Using a hashmap it is easy to keep track of the calculated values and its ancestors. If the target value is found simply follow the references to get the combination of the initial values.

## 🏃‍♀️ How to run it:
1. Clone the repo
2. The config for the game is defined in ```game.yml``` file. The target can be set with the key ```target```. There are two keys to configre the game:
    1. The key ```numbers_big``` lets you set the big numbers. 
    2. The key ```numbers_small``` lets you set the small numbers the algorithm uses.
4. Run the game-solver with 
    ```
    cargo run
    ```
    or compile it with ```cargo build --release``` and run 
    ```
    ./target/release/numbergame
    ```
    to even more speed 🚀

## 💲 CLI
The new version has a cli to configure the program. Just compile it and run 
```bash
./target/release/numbergame -h
```
You will be greated by this:
```
numbergame 0.2.0

Tobias Newmiwaka

A solver for the number round of the show Countdown. User defined numbers can be set from a config
file in yaml format

USAGE:
    numbergame [OPTIONS]

OPTIONS:
    -c, --config <FILE>    Sets a custom config file [default: game.yml]
    -h, --help             Print help information
    -r, --random           Start a game with random numbers
    -t, --target <n>       set the target [default: 420]
    -V, --version          Print version information
```

### Usage
You can specify a config with the ```-c``` Argument like
```
./target/release/numbergame -c game.yml
```
For a game with random numbers the target number can be specified with the ```-t```
```
./target/release/numbergame -r -t 212
```

### 🛠 TODOS

✅ add cli with help

❌ add unit tests 
