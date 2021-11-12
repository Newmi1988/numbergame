use std::path::Path;
use std::time::Instant;

mod config_reader;
mod game;
mod ntree;

fn main() {
    let config_path = Path::new("game.yml");
    let config = config_reader::read_config(config_path).unwrap();
    println!("Config: {:?}", config);

    if config.random {
        println!("Starting random number game");
        // start a random game (random picks for big and small numbers)
        let mut game = game::Numbergame::new_random_numbergame(config.target, 2, 4);

        println!("Big numbers: {:?}", game.numbers.big_number_selection);
        println!("Small numbers: {:?}", game.numbers.sml_number_selection);
        println!("Target : {:?}", game.target);

        let now = Instant::now();
        let solution: String = game.solve();
        let elapsed_time = now.elapsed();
        println!(
            "Solution equation : {}={} (took {} ms)",
            solution,
            config.target,
            elapsed_time.as_millis()
        );
    } else {
        // a game with given numbers
        println!("Starting game with user defined numbers");
        let numbers = game::Numbers {
            big_number_selection: config.numbers_big,
            sml_number_selection: config.numbers_small,
        };

        // create the game object
        let mut game_two = game::Numbergame::new_numbergame(config.target, numbers);
        println!("Big numbers: {:?}", game_two.numbers.big_number_selection);
        println!("Small numbers: {:?}", game_two.numbers.sml_number_selection);
        println!("Target : {:?}", game_two.target);

        let timer_game_two = Instant::now();
        let solution_game_two: String = game_two.solve();
        let elapsed_time2 = timer_game_two.elapsed();
        println!(
            "Solution equation : {}={} (took {} ms)",
            solution_game_two,
            config.target,
            elapsed_time2.as_millis()
        );
    }
}
