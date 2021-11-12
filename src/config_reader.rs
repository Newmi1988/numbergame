use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
    pub random: bool,
    pub numbers_big: Vec<u32>,
    pub numbers_small: Vec<u32>,
    pub target: u32,
}

pub fn read_config(path: &Path) -> Result<GameConfig, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let game_config = serde_yaml::from_reader(reader)?;

    Ok(game_config)
}
