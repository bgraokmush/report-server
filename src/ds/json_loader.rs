use super::JsonLoader;
use serde::de::DeserializeOwned;
use tokio::fs;

pub struct JsonDataSource {
    pub file_name: String,
}

impl JsonLoader for JsonDataSource {
    async fn load_data<T: DeserializeOwned>(&self) -> T {
        let content = fs::read_to_string(&self.file_name)
            .await
            .expect("Something went wrong reading the file");
        serde_json::from_str(&content).expect("Unable to parse JSON")
    }
}
