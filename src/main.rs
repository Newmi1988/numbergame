mod game;
mod ntree;

fn main() {
    let mut game = game::Numbergame::new_random_numbergame(812, 2, 4);

    println!("{:?}", game.numbers.big_number_selection);
    println!("{:?}", game.numbers.sml_number_selection);
    println!("Target : {:?}", game.target);

    let solution : String = game.solve();
    println!("Solution equation : {}", solution);

}

