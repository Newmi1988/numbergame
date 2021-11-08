use crate::ntree::CalcNumber;
use rand::Rng;
use itertools::Itertools;
use log::info;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Write;

pub struct Numbers {
    pub big_number_selection: Vec<u32>,
    pub sml_number_selection: Vec<u32>,
}

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

pub struct Numbergame<'game> {
    pub target: u32,
    pub selection_big_numbers: u32,
    pub selection_sml_numbers: u32,
    pub numbers: Numbers,
    pub derived: HashMap<u32, CalcNumber<'game>>,
    pub operators: Vec<String>,
}

impl<'game> Numbergame<'game> {
    pub fn new_random_numbergame(
        target: u32,
        selection_big_numbers: u32,
        selection_sml_numbers: u32,
    ) -> Numbergame<'game> {
        Numbergame {
            target: target,
            selection_big_numbers: selection_big_numbers,
            selection_sml_numbers: selection_sml_numbers,
            numbers: get_default_numbers(selection_sml_numbers),
            derived: HashMap::new(),
            operators: vec![
                "+".to_string(),
                "-".to_string(),
                "*".to_string(),
                "/".to_string(),
            ],
        }
    }

    pub fn new_numbergame(target : u32, numbers : Numbers) -> Numbergame<'game> {
        Numbergame {
            target : target,
            selection_big_numbers : numbers.big_number_selection.len() as u32,
            selection_sml_numbers : numbers.sml_number_selection.len() as u32,
            numbers : numbers,
            derived: HashMap::new(),
            operators: vec![
                "+".to_string(),
                "-".to_string(),
                "*".to_string(),
                "/".to_string(),
            ], 
        }
    }

    pub fn solve(&'game mut self) -> String {
        let mut found: bool = false;
        let numbers: Vec<u32> = [
            &self.numbers.sml_number_selection[..],
            &self.numbers.big_number_selection[..],
        ]
        .concat();
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

        if self.derived.contains_key(&self.target) {
            found = true;
            println!(
                "Found target : {:?}",
                self.derived.get_key_value(&self.target).unwrap()
            )
        }

        let mut equation: String = "".to_string();

        while found != true {
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
