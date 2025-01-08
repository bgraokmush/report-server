use super::TabularLoader;

pub struct CsvDataSource {
    pub file_name: String,
}

impl TabularLoader for CsvDataSource {
    async fn load_data<T: serde::de::DeserializeOwned>(&self) -> Vec<T> {
        let content = tokio::fs::read(self.file_name.clone())
            .await
            .expect("Something went wrong reading the file");

        let mut reader = csv::Reader::from_reader(content.as_slice());

        reader
            .deserialize()
            .map(|result| result.expect("Unable to parse CSV"))
            .collect()
    }
}
