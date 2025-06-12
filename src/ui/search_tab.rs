use crate::models::{Product, StoreManager};
use crate::scraping::WebScraper;

pub struct SearchTab {
    search_term: String,
    is_searching: bool,
    search_status: String,
    scraper: WebScraper,
}

impl SearchTab {
    pub fn new() -> Self {
        Self {
            search_term: String::new(),
            is_searching: false,
            search_status: String::new(),
            scraper: WebScraper::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, store_manager: &mut StoreManager) -> Option<Vec<Product>> {
        let mut search_results = None;

        ui.heading("🔍 Búsqueda de Ofertas");
        ui.separator();

        // Campo de búsqueda
        ui.horizontal(|ui| {
            ui.label("Término de búsqueda:");
            ui.text_edit_singleline(&mut self.search_term);
        });

        ui.add_space(10.0);

        // Lista de tiendas habilitadas
        ui.group(|ui| {
            ui.label("Tiendas habilitadas para búsqueda:");
            ui.separator();

            let mut any_enabled = false;
            for store in &mut store_manager.stores {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut store.enabled, &store.name);
                    if store.enabled {
                        any_enabled = true;
                    }
                    ui.label(&store.base_url);
                });
            }

            if !any_enabled {
                ui.colored_label(egui::Color32::YELLOW, "⚠️ No hay tiendas habilitadas");
            }
        });

        ui.add_space(10.0);

        // Botón de búsqueda
        ui.horizontal(|ui| {
            let search_button = ui.add_enabled(
                !self.search_term.is_empty() && !self.is_searching,
                egui::Button::new("🔍 Buscar Productos")
            );

            if search_button.clicked() {
                self.start_search();
                search_results = self.perform_search(&self.search_term.clone(), store_manager);
            }

            if self.is_searching {
                ui.spinner();
                ui.label("Buscando...");
            }
        });

        // Mostrar estado de la búsqueda
        if !self.search_status.is_empty() {
            ui.add_space(5.0);
            ui.label(&self.search_status);
        }

        // Configuración avanzada de búsqueda
        ui.add_space(15.0);
        ui.collapsing("⚙️ Configuración Avanzada", |ui| {
            ui.label("Configuraciones adicionales para la búsqueda:");
            ui.horizontal(|ui| {
                ui.label("Máximo productos por tienda:");
                // Aquí podrías añadir un control numérico
            });
            
            ui.horizontal(|ui| {
                ui.label("Delay entre requests (ms):");
                // Aquí podrías añadir un control numérico
            });
        });

        search_results
    }

    fn start_search(&mut self) {
        self.is_searching = true;
        self.search_status = "Iniciando búsqueda...".to_string();
    }

    fn perform_search(&mut self, query: &str, store_manager: &StoreManager) -> Option<Vec<Product>> {
        let enabled_stores = store_manager.get_enabled_stores();
        let mut all_products = Vec::new();
        let mut successful_searches = 0;
        let mut failed_searches = 0;

        for store in enabled_stores {
            self.search_status = format!("Buscando en {}...", store.name);
            
            match self.scraper.search_products(query, store) {
                Ok(products) => {
                    successful_searches += 1;
                    all_products.extend(products);
                }
                Err(e) => {
                    failed_searches += 1;
                    eprintln!("Error buscando en {}: {}", store.name, e);
                }
            }
        }

        self.is_searching = false;
        self.search_status = format!(
            "Búsqueda completada: {} tiendas exitosas, {} fallidas. {} productos encontrados.",
            successful_searches, failed_searches, all_products.len()
        );

        if !all_products.is_empty() {
            Some(all_products)
        } else {
            None
        }
    }
}

impl Default for SearchTab {
    fn default() -> Self {
        Self::new()
    }
}