use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreConfig {
    pub name: String,
    pub base_url: String,

    // Ejemplo: "{base_url}/search?q={query}"
    pub search_url_pattern: String, 

    
    pub product_container_selector: String,
    pub name_selector: String,
    pub price_selector: String,
    pub image_selector: String,
    pub link_selector: String,
    pub description_selector: Option<String>,
    pub enabled: bool,
}

impl Default for StoreConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            base_url: String::new(),
            search_url_pattern: "{base_url}/search?q={query}".to_string(),
            product_container_selector: String::new(),
            name_selector: String::new(),
            price_selector: String::new(),
            image_selector: String::new(),
            link_selector: String::new(),
            description_selector: None,
            enabled: true,
        }
    }
}

impl StoreConfig {
    pub fn new(name: String, base_url: String) -> Self {
        Self {
            name,
            base_url,
            ..Default::default()
        }
    }

    pub fn build_search_url(&self, query: &str) -> String {
        self.search_url_pattern
            .replace("{base_url}", &self.base_url)
            .replace("{query}", query)
    }

    pub fn is_valid(&self) -> bool {
        !self.name.is_empty()
            && !self.base_url.is_empty()
            && !self.product_container_selector.is_empty()
            && !self.name_selector.is_empty()
            && !self.price_selector.is_empty()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreManager {
    pub stores: Vec<StoreConfig>,
}

impl Default for StoreManager {
    fn default() -> Self {
        Self {
            stores: vec![
                // Configuración de ejemplo
                StoreConfig {
                    name: "Ejemplo Store".to_string(),
                    base_url: "https://ejemplo.com".to_string(),
                    search_url_pattern: "{base_url}/search?q={query}".to_string(),
                    product_container_selector: ".product-item".to_string(),
                    name_selector: ".product-name".to_string(),
                    price_selector: ".price".to_string(),
                    image_selector: ".product-image img".to_string(),
                    link_selector: "a".to_string(),
                    description_selector: Some(".description".to_string()),
                    enabled: true,
                },
            ],
        }
    }
}

impl StoreManager {
    pub fn new() -> Self {
        Self {
            stores: Vec::new(),
        }
    }

    pub fn add_store(&mut self, store: StoreConfig) {
        self.stores.push(store);
    }

    pub fn remove_store(&mut self, index: usize) -> Option<StoreConfig> {
        if index < self.stores.len() {
            Some(self.stores.remove(index))
        } else {
            None
        }
    }

    pub fn get_enabled_stores(&self) -> Vec<&StoreConfig> {
        self.stores.iter().filter(|store| store.enabled).collect()
    }

    pub fn update_store(&mut self, index: usize, updated_store: StoreConfig) -> bool {
        if index < self.stores.len() {
            self.stores[index] = updated_store;
            true
        } else {
            false
        }
    }
}