use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TopGames {
    pub games: Vec<Game>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub developer: String,
    pub genre: String,
    pub rating: f32,
}
