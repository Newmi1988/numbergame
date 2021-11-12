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

# Is it a tree? 🌴 Yeah not really...
Every value is the calculated using two numbers and one of the following operations : + , - , * , / .
```
   a         b         c 
a°b a°c   b°a b°c   c°a c°b
```
However not all operations are commutative, depending on the operations. The tree increased in width from top to button.

Given ```n``` numbers that can be combined you get ```n!*4``` combinations (4 being the number of operations). In the next step you get ```((n+n!*4)!)*4``` possible combinations. There are many duplicates, as you can exspect. 

Using a hashmap it is easy to keep track of the calculated values and its ancestors. If the target value is found simply follow the references to get the combination of the initial values.

## 🏃‍♀️ How to run it:
1. Clone the repo
2. The config for the game is defined in _game.yml_ file. The target can be set with the key ```target```. You can run the solver in two configurations:

    1. If you want to use randomly chosen number set 
        ```
        random : true
        ```
        this will ignore the small and big numbers set in the others keys
    2. Use predefined numbers. The key ```numbers_big``` lets you set the big numbers. The key ```numbers_small``` lets you set the small numbers the algorithm uses.
4. Run the game-solver with 
    ```
    cargo run
    ```
    or compile it with ```cargo build --release``` and run 
    ```
    ./target/release/numbergame
    ```
    to even more speed 🚀

### 🛠 TODOS
❌ add cli with help
