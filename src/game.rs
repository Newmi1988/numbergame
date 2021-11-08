use crate::ntree::CalcNumber;
use itertools::Itertools;
use log::info;
use rand::Rng;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Write;

/// Store the numbers of the game in this struct
pub struct Numbers {
    /// big numbers are given for the game see the get_default_numbers function
    pub big_number_selection: Vec<u32>,
    /// small numbers are between 1 and 9
    pub sml_number_selection: Vec<u32>,
}

/// sample some default numbers
/// 
/// # Arguments
/// 
/// * `select_n_small` number of small numbers to pick
/// 
/// # Examples 
/// ```
/// let random_default_numbers = get_default_numbers(2)
/// ```
pub fn get_default_numbers(select_n_small: u32) -> Numbers {
    let mut sml_random_number_vec: Vec<u32> =
        Vec::with_capacity(select_n_small.try_into().unwrap());
    let mut rng = rand::thread_rng();
    for _ in 0..sml_random_number_vec.capacity() {
        let mut number = rng.gen_range(1..10);
        while sml_random_number_vec.contains(&number) {
            number = rng.gen_range(1..10)
        }
        sml_random_number_vec.push(number)
    }

    Numbers {
        big_number_selection: vec![100, 75, 50, 25],
        sml_number_selection: sml_random_number_vec,
    }
}

/// the goal of the game is to find an equation using the start numbers and numbers calculated from them
pub struct Numbergame<'game> {
    pub target: u32,
    pub selection_big_numbers: u32,
    pub selection_sml_numbers: u32,
    pub numbers: Numbers,
    pub derived: HashMap<u32, CalcNumber<'game>>,
    pub operators: Vec<String>,
}

/// init and solve the game
impl<'game> Numbergame<'game> {
    /// Start the game with a random selection of numbers
    /// # Args
    /// 
    /// * `target` the number to reach
    /// * `selection_big_numbers` number of big numbers to select (still needs to be implemented)
    /// * `selection_sml_numbers` number of small numbers to select
    pub fn new_random_numbergame(
        target: u32,
        selection_big_numbers: u32,
        selection_sml_numbers: u32,
    ) -> Numbergame<'game> {
        Numbergame {
            target: target,
            selection_big_numbers: selection_big_numbers,
            selection_sml_numbers: selection_sml_numbers,
            /// get the sampled numbers as a Numbers struct
            numbers: get_default_numbers(selection_sml_numbers),
            /// hashmap for looking up calculated numbers
            derived: HashMap::new(),
            /// allowed operations
            operators: vec![
                "+".to_string(),
                "-".to_string(),
                "*".to_string(),
                "/".to_string(),
            ],
        }
    }

    /// start the game with user definde numbers
    /// 
    /// # Args 
    /// * `target` the number to reach
    /// * `numbers` user defined struct with arrays of numbers
    pub fn new_numbergame(target: u32, numbers: Numbers) -> Numbergame<'game> {
        Numbergame {
            target: target,
            selection_big_numbers: numbers.big_number_selection.len() as u32,
            selection_sml_numbers: numbers.sml_number_selection.len() as u32,
            numbers: numbers,
            derived: HashMap::new(),
            operators: vec![
                "+".to_string(),
                "-".to_string(),
                "*".to_string(),
                "/".to_string(),
            ],
        }
    }

    /// Solve the game
    
    pub fn solve(&'game mut self) -> String {
        // combine slices of the vectors to a new vector
        let numbers: Vec<u32> = [
            &self.numbers.sml_number_selection[..],
            &self.numbers.big_number_selection[..],
        ]
        .concat();

        // get the cartesian product (every combination of all items)
        for (a, b) in numbers.iter().tuple_combinations() {
            for op in self.operators.iter() {
                let tmp = match CalcNumber::generate_number_with_operation(*a, *b, op) {
                    Err(error) => {
                        info!("Error: {}", error);
                        continue;
                    }
                    Ok(res) => res,
                };

                self.derived.insert(tmp.value.clone(), tmp);
            }
        }

        // an initial hit is should be quite rare
        // generate more iteratively
        // uncomment the following lines to look into the calculated values
        // for (v, s) in &self.derived {
        //     println!("{} : {:?}", v, s)
        // }

        let mut equation: String = "".to_string();
        
        // if the number is in the first batch of combinations return the solution...
        if self.derived.contains_key(&self.target) {
            println!(
                "Found target : {:?}",
                self.derived.get_key_value(&self.target).unwrap()
            );
            equation = Numbergame::get_equation(&self.numbers, &self.derived, &self.target);
            return equation
        }

        // use the new combinations to calculate more
        let mut found: bool = false;
        while found != true {
            // add the hashmap keys to the vector
            let hashmap_keys: Vec<u32> = self.derived.keys().cloned().collect();
            let new_canidates: Vec<u32> = [
                &self.numbers.sml_number_selection[..],
                &self.numbers.big_number_selection[..],
                &hashmap_keys[..],
            ]
            .concat();
            for (a, b) in new_canidates.iter().tuple_combinations() {
                for op in self.operators.iter() {
                    let tmp = match CalcNumber::generate_number_with_operation(*a, *b, op) {
                        Err(error) => {
                            info!("Error: {}", error);
                            continue;
                        }
                        Ok(res) => res,
                    };
                    if self.derived.contains_key(&tmp.value) {
                        // println!("Value {} already in hashmap -> skipping", tmp.value);
                    } else {
                        self.derived.insert(tmp.value.clone(), tmp);
                    }
                }
            }
            // break if the target was found
            if self.derived.contains_key(&self.target) {
                println!(
                    "Found target : {:?}",
                    self.derived.get_key_value(&self.target).unwrap()
                );
                found = true;
                equation = Numbergame::get_equation(&self.numbers, &self.derived, &self.target);
            } else {
                continue;
            }
        }

        return equation;
    }

    fn get_equation(
        orig_selection: &Numbers,
        derived_values: &HashMap<u32, CalcNumber>,
        solution: &u32,
    ) -> String {
        let mut eq: String = "".to_string();
        let res: &CalcNumber = derived_values.get_key_value(solution).unwrap().1;
        if orig_selection
            .big_number_selection
            .contains(&res.left_element)
            || orig_selection
                .sml_number_selection
                .contains(&res.left_element)
        {
            write!(eq, "({}{}", res.left_element, res.operation).unwrap();
        } else {
            let returned_equation =
                Numbergame::get_equation(orig_selection, derived_values, &res.left_element);
            write!(eq, "({}{}", returned_equation, res.operation).unwrap();
        }

        if orig_selection
            .sml_number_selection
            .contains(&res.right_element)
            || orig_selection
                .big_number_selection
                .contains(&res.right_element)
        {
            write!(eq, "{})", res.right_element).unwrap();
        } else {
            let returned_equation =
                Numbergame::get_equation(orig_selection, derived_values, &res.right_element);
            write!(eq, "{})", returned_equation).unwrap();
        }
        return eq;
    }
}
