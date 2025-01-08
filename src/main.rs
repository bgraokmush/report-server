use axum::{routing::get, Router};
use report::{generate_sales_report, generate_top_games_report, generate_top_players_report, home};
use std::{net::SocketAddr, sync::Arc};
use tera::Tera;

mod ds;
mod model;
mod report;

#[tokio::main]
async fn main() {
    let tera = match Tera::new("templates/**/*") {
        Ok(t) => Arc::new(t),
        Err(e) => {
            eprintln!("Error on loading templates: {}", e);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/", {
            let tera = Arc::clone(&tera);
            get(move || async move { home(tera).await })
        })
        .route("/reports/sales/monthly", {
            let tera = Arc::clone(&tera);
            get(move || async move { generate_sales_report(tera).await })
        })
        .route("/reports/games/top", {
            let tera = Arc::clone(&tera);
            get(move || async move { generate_top_games_report(tera).await })
        })
        .route("/reports/player/top", {
            let tera = Arc::clone(&tera);
            get(move || async move { generate_top_players_report(tera).await })
        });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
