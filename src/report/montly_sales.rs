use std::sync::Arc;

use axum::response::Html;
use tera::Tera;

use crate::{
    ds::{JsonDataSource, JsonLoader},
    model::SalesData,
};

use super::generate_report;

pub async fn generate_sales_report(tera: Arc<Tera>) -> Html<String> {
    let jds = JsonDataSource {
        file_name: "data/monthly_sales.json".to_string(),
    };
    let sales_data: SalesData = jds.load_data().await;
    generate_report(tera, "monthly_sales.html", sales_data).await
}
