pub mod montly_sales;
pub use montly_sales::*;

pub mod top_games;
pub use top_games::*;

pub mod top_player;
pub use top_player::*;

use std::sync::Arc;

use axum::response::Html;
use serde::Serialize;
use tera::{Context, Tera};

pub async fn generate_report<T: Serialize>(
    tera: Arc<Tera>,
    template_name: &str,
    context_data: T,
) -> Html<String> {
    let mut context = Context::new();

    let data_map = serde_json::to_value(context_data)
        .expect("Failed to serialize context data")
        .as_object()
        .expect("Context data is not a valid object")
        .clone();

    for (key, value) in data_map {
        context.insert(key, &value);
    }

    let rendered = tera
        .render(template_name, &context)
        .map_err(|e| {
            eprintln!("Error on render operation: {:?}", e);
            std::process::exit(1);
        })
        .unwrap();

    Html(rendered)
}

pub async fn home(tera: Arc<Tera>) -> Html<String> {
    let context = Context::new();
    let rendered = tera.render("index.html", &context).unwrap_or_else(|e| {
        eprintln!("Error rendering index page: {:?}", e);
        "Failed to render index page.".to_string()
    });
    Html(rendered)
}
