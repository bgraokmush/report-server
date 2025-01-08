use std::sync::Arc;

use axum::response::Html;
use tera::Tera;

use crate::{
    ds::{CsvDataSource, TabularLoader},
    model::{Player, TopPlayers},
};

use super::generate_report;

pub async fn generate_top_players_report(tera: Arc<Tera>) -> Html<String> {
    let cvs = CsvDataSource {
        file_name: "data/top_players.txt".to_string(),
    };

    let players: Vec<Player> = cvs.load_data().await;
    let top_player = TopPlayers { players };
    generate_report(tera, "top_players.html", top_player).await
}
