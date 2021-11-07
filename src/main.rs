mod game;
mod ntree;

fn main() {
    let game = game::Numbergame::new_numbergame(102, 4, 7);

    println!("{:?}", game.numbers.big_number_selection);
    println!("{:?}", game.numbers.sml_number_selection);
    println!("Target : {:?}", game.target);

    // let n1 = ntree::CalcNumber::generate_number_with_operation(1, 2, "+").unwrap();
    // println!("{:?}", n1);

    // let n2 = ntree::CalcNumber::generate_number_with_operation(3, 1, "-").unwrap();
    // println!("{:?}", n2);


    // let n3 = ntree::CalcNumber::generate_number_with_operation(3, 3, "/").unwrap();
    // println!("{:?}", n3);

    // let n12 = n1.combine(2,"*").unwrap();
    // println!("{:?}", n12);

    game.generate_canidates();

}

