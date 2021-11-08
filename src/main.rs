use std::time::Instant;

mod game;
mod ntree;

fn main() {
    // start a random game (random picks for big and small numbers)
    let mut game = game::Numbergame::new_random_numbergame(812, 2, 4);

    println!("Big numbers {:?}", game.numbers.big_number_selection);
    println!("Small numbers {:?}", game.numbers.sml_number_selection);
    println!("Target : {:?}", game.target);

    let now = Instant::now();
    let solution: String = game.solve();
    let elapsed_time = now.elapsed();
    println!(
        "Solution equation : {}, Took {} ms",
        solution,
        elapsed_time.as_millis()
    );

    // a game with given numbers
    let numbers = game::Numbers {
        big_number_selection: vec![100, 75, 50, 25],
        sml_number_selection: vec![2, 5, 7, 3],
    };

    // create the game object
    let mut game_two = game::Numbergame::new_numbergame(812, numbers);
    println!("Big numbers : {:?}", game_two.numbers.big_number_selection);
    println!("Small numbers{:?}", game_two.numbers.sml_number_selection);
    println!("Target : {:?}", game_two.target);

    let timer_game_two = Instant::now();
    let solution_game_two: String = game_two.solve();
    let elapsed_time2 = timer_game_two.elapsed();
    println!(
        "Solution equation : {}, Took {} ms",
        solution_game_two,
        elapsed_time2.as_millis()
    );
}
