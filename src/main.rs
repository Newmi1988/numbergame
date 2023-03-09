use std::path::Path;
use std::time::Instant;

mod config_reader;
mod game;
mod ntree;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
use clap::Parser;

/// A solver for the number round of the show Countdown. User defined
/// numbers can be set from a config file in yaml format.
#[derive(Parser)]
#[clap(version = VERSION, author = "Newmi1988")]
struct Opts {
    /// Sets a custom config file.
    #[clap(short, long, default_value = "game.yml", value_name = "FILE")]
    config: String,
    /// Start a game with random numbers
    #[clap(short, long)]
    random: bool,
    /// set the target
    #[clap(short, long, value_name = "n", default_value = "420")]
    target: u32,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.random {
        true => {
            println!("Starting random number game");
            // start a random game (random picks for big and small numbers)
            let mut game = game::Numbergame::new_random_numbergame(opts.target, 2, 4);

            println!("Big numbers: {:?}", game.numbers.big_number_selection);
            println!("Small numbers: {:?}", game.numbers.sml_number_selection);
            println!("Target : {:?}", game.target);

            let now = Instant::now();
            let solution: String = game.solve();
            let elapsed_time = now.elapsed();
            println!(
                "Solution equation : {}={} (took {} ms)",
                solution,
                opts.target,
                elapsed_time.as_millis()
            );
        }
        false => {
            let config_input: String = match opts.config.as_ref() {
                "game.yaml" => String::from("game.yml"),
                _ => {
                    println!("Using config file : {}", opts.config);
                    opts.config
                }
            };
            let config = config_reader::read_config(Path::new(&config_input)).unwrap();
            println!("Config: {:?}", config);

            // a game with given numbers
            println!("Starting game with user defined numbers");
            let numbers = game::Numbers {
                big_number_selection: config.numbers_big,
                sml_number_selection: config.numbers_small,
            };

            // create the game object
            let mut game_two = game::Numbergame::new_numbergame(config.target, numbers);
            println!("Big numbers: {:?}", game_two.numbers.big_number_selection,);
            println!("Small numbers: {:?}", game_two.numbers.sml_number_selection,);
            println!("Target : {:?}", game_two.target);

            let timer_game_from_config = Instant::now();
            let solution_game_from_config: String = game_two.solve();
            let elapsed_time_from_config = timer_game_from_config.elapsed();
            println!(
                "Solution equation : {}={} (took {} ms)",
                solution_game_from_config,
                config.target,
                elapsed_time_from_config.as_millis()
            );
        }
    }
}
