use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub price: String,
    pub url: String,
    pub image_url: String,
    pub store_name: String,
    pub description: Option<String>,
}

impl Product {
    pub fn new(
        name: String,
        price: String,
        url: String,
        image_url: String,
        store_name: String,
    ) -> Self {
        Self {
            name,
            price,
            url,
            image_url,
            store_name,
            description: None,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    // extraer el precio 
    pub fn get_numeric_price(&self) -> f64 {
        self.price
            .chars()
            .filter(|c| c.is_numeric() || *c == '.' || *c == ',')
            .collect::<String>()
            .replace(',', ".")
            .parse()
            .unwrap_or(0.0)
    }
}