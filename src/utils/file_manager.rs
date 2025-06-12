use crate::models::{Product, StoreManager};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub struct FileManager;

impl FileManager {
    const STORES_FILE: &'static str = "stores.json";
    const RESULTS_FILE: &'static str = "search_results.json";
    const CONFIG_FILE: &'static str = "config.json";

    pub fn new() -> Self {
        Self
    }

    /// Carga la configuración de tiendas desde archivo
    pub fn load_stores(&self) -> StoreManager {
        if Path::new(Self::STORES_FILE).exists() {
            match fs::read_to_string(Self::STORES_FILE) {
                Ok(content) => {
                    match serde_json::from_str::<StoreManager>(&content) {
                        Ok(store_manager) => store_manager,
                        Err(e) => {
                            eprintln!("Error al parsear stores.json: {}", e);
                            StoreManager::default()
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error al leer stores.json: {}", e);
                    StoreManager::default()
                }
            }
        } else {
            // Crear archivo con configuración por defecto
            let default_stores = StoreManager::default();
            self.save_stores(&default_stores);
            default_stores
        }
    }

    /// Guarda la configuración de tiendas en archivo
    pub fn save_stores(&self, store_manager: &StoreManager) -> Result<(), String> {
        match serde_json::to_string_pretty(store_manager) {
            Ok(json) => {
                match fs::write(Self::STORES_FILE, json) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Error al escribir stores.json: {}", e)),
                }
            }
            Err(e) => Err(format!("Error al serializar stores: {}", e)),
        }
    }

    /// Guarda los resultados de búsqueda
    pub fn save_search_results(&self, products: &[Product]) -> Result<(), String> {
        let results = SearchResults {
            timestamp: chrono::Utc::now().to_rfc3339(),
            products: products.to_vec(),
        };

        match serde_json::to_string_pretty(&results) {
            Ok(json) => {
                match fs::write(Self::RESULTS_FILE, json) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Error al escribir resultados: {}", e)),
                }
            }
            Err(e) => Err(format!("Error al serializar resultados: {}", e)),
        }
    }

    /// Carga los últimos resultados de búsqueda
    pub fn load_search_results(&self) -> Vec<Product> {
        if Path::new(Self::RESULTS_FILE).exists() {
            match fs::read_to_string(Self::RESULTS_FILE) {
                Ok(content) => {
                    match serde_json::from_str::<SearchResults>(&content) {
                        Ok(results) => results.products,
                        Err(_) => Vec::new(),
                    }
                }
                Err(_) => Vec::new(),
            }
        } else {
            Vec::new()
        }
    }

    /// Carga la configuración general de la aplicación
    pub fn load_app_config(&self) -> AppConfig {
        if Path::new(Self::CONFIG_FILE).exists() {
            match fs::read_to_string(Self::CONFIG_FILE) {
                Ok(content) => {
                    match serde_json::from_str::<AppConfig>(&content) {
                        Ok(config) => config,
                        Err(_) => AppConfig::default(),
                    }
                }
                Err(_) => AppConfig::default(),
            }
        } else {
            let default_config = AppConfig::default();
            self.save_app_config(&default_config);
            default_config
        }
    }

    /// Guarda la configuración general de la aplicación
    pub fn save_app_config(&self, config: &AppConfig) -> Result<(), String> {
        match serde_json::to_string_pretty(config) {
            Ok(json) => {
                match fs::write(Self::CONFIG_FILE, json) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("Error al escribir config.json: {}", e)),
                }
            }
            Err(e) => Err(format!("Error al serializar config: {}", e)),
        }
    }

    /// Exporta productos a CSV
    pub fn export_to_csv(&self, products: &[Product], filename: &str) -> Result<(), String> {
        let mut csv_content = String::from("Nombre,Precio,URL,Tienda,Descripción\n");
        
        for product in products {
            let description = product.description.as_deref().unwrap_or("");
            csv_content.push_str(&format!(
                "\"{}\",\"{}\",\"{}\",\"{}\",\"{}\"\n",
                product.name.replace('"', "\"\""),
                product.price.replace('"', "\"\""),
                product.url.replace('"', "\"\""),
                product.store_name.replace('"', "\"\""),
                description.replace('"', "\"\"")
            ));
        }

        match fs::write(filename, csv_content) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error al exportar CSV: {}", e)),
        }
    }

    /// Crea un backup de la configuración
    pub fn create_backup(&self) -> Result<(), String> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_filename = format!("stores_backup_{}.json", timestamp);
        
        if Path::new(Self::STORES_FILE).exists() {
            match fs::copy(Self::STORES_FILE, &backup_filename) {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Error al crear backup: {}", e)),
            }
        } else {
            Err("No existe archivo de stores para hacer backup".to_string())
        }
    }
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResults {
    timestamp: String,
    products: Vec<Product>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub max_products_per_store: usize,
    pub request_delay_ms: u64,
    pub user_agent: String,
    pub auto_save_results: bool,
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            max_products_per_store: 50,
            request_delay_ms: 1000,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_string(),
            auto_save_results: true,
            theme: "dark".to_string(),
        }
    }
}