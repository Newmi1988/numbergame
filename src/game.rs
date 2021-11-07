use crate::ntree::CalcNumber;
use rand::Rng;
// use std::collections::VecDeque;
use std::convert::TryInto;
use std::collections::HashMap;
use itertools::Itertools;
use log::{info, warn};

pub struct Numbers {
    pub big_number_selection: Vec<u32>,
    pub sml_number_selection: Vec<u32>,
}

pub fn get_default_numbers(select_n_small: u32) -> Numbers {
    let mut sml_random_number_vec: Vec<u32> = Vec::with_capacity(select_n_small.try_into().unwrap());
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


pub struct Numbergame <'a> { 
    pub target: u32,
    pub selection_big_numbers: u32,
    pub selection_sml_numbers: u32,
    pub numbers: Numbers,
    pub derived : HashMap<&'a str,CalcNumber<'a>>,
    pub operators : Vec<String>
}

impl Numbergame <'_> {
    pub fn new_numbergame(
        target: u32,
        selection_big_numbers: u32,
        selection_sml_numbers: u32,
    ) -> Numbergame <'static> {
        Numbergame {
            target: target,
            selection_big_numbers: selection_big_numbers,
            selection_sml_numbers: selection_sml_numbers,
            numbers: get_default_numbers(selection_sml_numbers),
            derived : HashMap::new(),
            operators :  vec!["+".to_string(),"-".to_string(),"*".to_string(), "/".to_string()]
        }
    }

    pub fn generate_canidates(&self) {

        let mut _numbers : Vec<u32> = [&self.numbers.sml_number_selection[..], &self.numbers.big_number_selection[..]].concat();
        for (a,b) in _numbers.iter().tuple_combinations() {
            println!("{}x{}", a,b);
            for op in self.operators.iter() {
                let tmp = match CalcNumber::generate_number_with_operation(*a, *b, op) {
                    Err(error) => {
                        info!("Error: {}", error);
                        continue;
                    },
                    Ok(res) => res
                };

                println!("   {:?}", tmp);
                self.derived.insert(tmp.value.clone().to_string(), tmp);


            }
        }
        
    } 

}
