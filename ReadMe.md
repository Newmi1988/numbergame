# Solver for the number round of the show countdown ⏱

A friend approached me with this fun little puzzle. We talked a bit about different strategies that can be applied to solve this game in under 30 seconds.

There are solutions out there that use precalculated tables and a simple lookup. Python seemed to slow, so my idea was to use rust to calculate all combinations on the fly to find the equation that leads to the target number.

The equation is found using a simple hashmap that maps a value to a struct containing the elements of every operation. 

# Is it a tree? 🌴 Yeah not really...
Every node is the result of one of the following operations: + , - , * , / .
```
   a         b         c 
a°b a°c   b°a b°c   c°a c°b
```
Howevery as not all operations are commutative, depending on the operations the tree increased in width from top to button.

Given ```n``` numbers that can be combined you get ```n!*4``` combinations. In the next step you get ```((n+n!*4)!)*4``` possible combinations. There are many duplicates, as you can exspect. 

Using a hashmap it is easy to keep track of the calculated values (nodes of the tree) and its ancestors. If the taget value is found simple follow the references to the top to get the combination of the initial values.

## How to run it 🏃‍♀️
1. clone the repo
2. look into the src/main.rs
3. There are two functions to initialize the game.
    1. The first starts a game with randomly chosen numbers
    2. The second lets you define the numbers yourself.

4. Run the game with (or compile it for even more speed 🚀 ) 
```
cargo run
```
