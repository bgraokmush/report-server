pub mod json_loader;
pub use json_loader::*;

pub mod csv_ds;
pub use csv_ds::*;

use serde::de::DeserializeOwned;

pub trait JsonLoader {
    async fn load_data<T: DeserializeOwned>(&self) -> T;
}

pub trait TabularLoader {
    async fn load_data<T: DeserializeOwned>(&self) -> Vec<T>;
}
