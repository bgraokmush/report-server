use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TopPlayers {
    pub players: Vec<Player>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    pub nickname: String,
    pub total_score: i32,
    pub join_year: i32,
}
