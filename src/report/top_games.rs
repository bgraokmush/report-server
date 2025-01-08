use std::sync::Arc;

use axum::response::Html;
use tera::Tera;

use crate::{
    ds::{JsonDataSource, JsonLoader},
    model::TopGames,
};

use super::generate_report;

pub async fn generate_top_games_report(tera: Arc<Tera>) -> Html<String> {
    let jds: JsonDataSource = JsonDataSource {
        file_name: "data/top_games.json".to_string(),
    };
    let top_games: TopGames = jds.load_data().await;
    generate_report(tera, "top_games.html", top_games).await
}
